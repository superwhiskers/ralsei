//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use futures::stream::TryStreamExt;
use http::{
    uri::{Authority, InvalidUri, PathAndQuery},
    Error as HttpError, HeaderMap, HeaderValue, Request, Uri, Version as HttpVersion,
};
use hyper::{
    client::{Client as HttpClient, HttpConnector, ResponseFuture},
    Body, Error as HyperError,
};
use hyper_tls::HttpsConnector;
use native_tls::{
    Certificate, Error as NativeTlsError, Identity, TlsConnector as NativeTlsConnector,
};
use parking_lot::RwLock;
use quick_xml::de::{from_reader as xml_from_reader, DeError as XmlDeError};
use std::{borrow::Cow, convert::TryFrom, io::BufReader, sync::Arc};
use thiserror::Error;
use tokio_tls::TlsConnector;

use crate::{
    keypairs::{CTR_COMMON_1, NINTENDO_CACERTS, WUP_ACCOUNT_1},
    model::{
        console::common::{Console, HeaderConstructionError, Kind as ConsoleKind},
        network::Nnid,
        server::{Kind as ServerKind, DEFAULT_ACCOUNT_SERVER_HOST},
        xml::error as error_xml,
    },
};

/// A client for the Nintendo Network account servers
///
/// -- add more descriptive documentation here --
pub struct Client<'a, C: Console<'a> + Send + Clone> {
    /// The host of the account server (not the api endpoint)
    ///
    /// If no value is provided, it is initialized with [`DEFAULT_ACCOUNT_SERVER_HOST`].
    ///
    /// [`DEFAULT_ACCOUNT_SERVER_HOST`]: ../../model/server/constant.DEFAULT_ACCOUNT_SERVER_HOST.html
    pub host: RwLock<Cow<'a, str>>,

    /// The console data we are connecting to the account server with
    ///
    /// This field is used to generate a set of HTTP headers that are passed to the server in
    /// requests to mimic a real console.
    pub console: Arc<RwLock<C>>,

    /// A cache of the headers to avoid recalling [`Console.http_headers()`]
    ///
    /// [`Console.http_headers()`]: ../../model/console/common/trait.Console.html#tymethod.http_headers
    pub(crate) cached_headers: RwLock<HeaderMap<HeaderValue>>,

    /// The HTTP client used to make requests to the account server
    pub(crate) http: HttpClient<HttpsConnector<HttpConnector>, Body>,
}

impl<'a, C: Console<'a> + Send + Clone> Client<'a, C> {
    /// Create a new Client using the provided [`Console`]
    ///
    /// If no value for the `host` parameter is provided, the corresponding struct field, [`host`],
    /// is initialized to [`DEFAULT_ACCOUNT_SERVER_HOST`].
    ///
    /// If no value for the `keypair` parameter is provided, it is initialized to the default
    /// (official Nintendo) client certificate for the console that the provided [`Console`]
    /// implementor reports itself as.
    ///
    /// If no value for the `cacert_bundle` parameter is provided, it is initialized to the default
    /// (official Nintendo) certificate authority bundle, [`NINTENDO_CACERTS`]
    ///
    /// [`Console`]: ../../model/console/common/trait.Console.html
    /// [`host`]: #structfield.host
    /// [`DEFAULT_ACCOUNT_SERVER_HOST`]: ../../model/server/constant.DEFAULT_ACCOUNT_SERVER_HOST.html
    /// [`NINTENDO_CACERTS`]: ../../keypairs/constant.NINTENDO_CACERTS.html
    pub fn new<'b>(
        host: Option<Cow<'a, str>>,
        console: Arc<RwLock<C>>,
        identity: Option<Identity>,
        cacert_bundle: Option<Cow<'b, [Certificate]>>,
    ) -> Result<Self, ClientError> {
        let host = host.unwrap_or(Cow::Borrowed(DEFAULT_ACCOUNT_SERVER_HOST));
        Ok(Client {
            host: RwLock::new(host.clone()),
            console: Arc::clone(&console),
            cached_headers: RwLock::new(console.read().http_headers(ServerKind::Account(&host))?),
            http: HttpClient::builder().build(HttpsConnector::from((
                {
                    let mut http = HttpConnector::new();
                    http.enforce_http(false);
                    http
                },
                TlsConnector::from({
                    let mut builder = NativeTlsConnector::builder();

                    builder.identity(if let Some(identity) = identity {
                        identity
                    } else {
                        match console.read().kind() {
                            ConsoleKind::N3ds => Identity::from_pkcs12(CTR_COMMON_1, "ralsei")?,
                            ConsoleKind::WiiU => Identity::from_pkcs12(WUP_ACCOUNT_1, "ralsei")?,
                            kind => return Err(ClientError::UnsupportedConsoleKind(kind)),
                        }
                    });

                    if let Some(cacert_bundle) = cacert_bundle {
                        for cert in cacert_bundle.into_owned() {
                            builder.add_root_certificate(cert);
                        }
                    } else {
                        for cert in &NINTENDO_CACERTS {
                            builder.add_root_certificate(Certificate::from_der(cert)?);
                        }
                    }

                    builder.build()?
                }),
            ))),
        })
    }

    /// Refresh the cached http headers
    ///
    /// This method blocks until a read lock can be acquired on the [`console`] and [`host`]
    /// fields, and a write lock can be acquired on the `cached_headers` field.
    ///
    /// [`console`]: #structfield.console
    /// [`host`]: #structfield.host
    pub fn refresh_header(&self) -> Result<(), ClientError> {
        *self.cached_headers.write() = self
            .console
            .read()
            .http_headers(ServerKind::Account(&self.host.read()))?;
        Ok(())
    }

    /// Execute a request using the provided [`Request`]
    ///
    /// [`Request`]: https://docs.rs/http/0.2.1/http/request/struct.Request.html
    pub fn request(&self, mut request: Request<Body>) -> ResponseFuture {
        request
            .headers_mut()
            .extend(self.cached_headers.read().clone());
        self.http.request(request)
    }

    /// Check if a user with the given [`Nnid`] exists on the provided account server
    ///
    /// [`Nnid`]: ../../model/network/struct.Nnid.html
    pub async fn does_user_exist(&self, nnid: Nnid<'_>) -> Result<bool, ClientError> {
        let mut path = nnid.0.into_owned();
        path.insert_str(0, "/v1/api/people/");
        let response = self
            .request(
                Request::builder()
                    .method("GET")
                    .uri(
                        Uri::builder()
                            .scheme("https")
                            .authority(Authority::try_from(self.host.read().as_ref())?)
                            .path_and_query(PathAndQuery::try_from(path.as_str())?)
                            .build()?,
                    )
                    .version(HttpVersion::HTTP_11)
                    .body(Body::empty())?,
            )
            .await?;
        match response.status().as_u16() {
            200 => Ok(false),
            400 | 401 => {
                let error = xml_from_reader::<_, error_xml::Errors>(BufReader::new(
                    response
                        .into_body()
                        .try_fold(Vec::new(), |mut accumulator, chunk| async move {
                            accumulator.extend_from_slice(&chunk);
                            Ok(accumulator)
                        })
                        .await?
                        .as_slice(),
                ))?;
                match error.first_code() {
                    Some(error_xml::ErrorCode::Known(
                        error_xml::ErrorCodeValue::AccountIdExists,
                    )) => Ok(true),
                    _ => Err(error.into()),
                }
            }
            status => Err(ClientError::UnknownStatusCode(status)),
        }
    }
}

/// A list of possible errors encountered while using the [`Client`]
///
/// [`Client`]: ./struct.Client.html
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum ClientError {
    /// An error encountered when the provided [`Kind`] of console is not supported
    ///
    /// [`Kind`]:  ../../model/console/common/enum.Kind.html
    #[error("`{0}` is an unsupported console Kind")]
    UnsupportedConsoleKind(ConsoleKind),

    /// An error encountered when the header values provided by the [`Console`] are invalid
    ///
    /// [`Console`]: ../../model/console/common/trait.Console.html
    #[error("An error was encountered while constructing headers")]
    HeaderConstructionError(#[from] HeaderConstructionError),

    /// An error encountered while using the native tls implementation
    #[error("An error was encountered while using the native tls implementation")]
    NativeTlsError(#[from] NativeTlsError),

    /// An error encountered if the Nintendo Network API raises an error
    #[error("An error was encountered while using the Nintendo Network account API")]
    ErrorXml(#[from] error_xml::Errors),

    /// An error was encountered while using hyper
    #[error("An error was encountered while using the `hyper` library")]
    HyperError(#[from] HyperError),

    /// An error was encountered while using the http library
    #[error("An error was encountered while using the `http` library")]
    HttpError(#[from] HttpError),

    /// An error was encountered while constructing a Uri
    #[error("An error was encountered while constructing a Uri")]
    UriConstructionError(#[from] InvalidUri),

    /// An error was encountered while deserializing XML
    #[error("An error was encountered while deserializing XML")]
    XmlDeError(#[from] XmlDeError),

    /// The Nintendo Network API returned an unknown status code
    #[error("The Nintendo Network API returned an unknown status code, `{0}`")]
    UnknownStatusCode(u16),
}
