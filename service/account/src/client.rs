//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020-2021 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use chrono::{
    offset::{TimeZone, Utc},
    DateTime,
};
use futures::stream::TryStreamExt;
use http::{
    header::{HeaderMap, HeaderValue, ToStrError as HeaderValueToStrError},
    uri::{Authority, InvalidUri, PathAndQuery},
    Error as HttpError, Request, Uri, Version as HttpVersion,
};
use hyper::{
    client::{Client as HttpClient, HttpConnector, ResponseFuture},
    Body, Error as HyperError,
};
use hyper_tls::HttpsConnector;
use iso::language::{Iso639_1, Language};
use isocountry::CountryCode;
use native_tls::{
    Certificate, Error as NativeTlsError, Identity, TlsConnector as NativeTlsConnector,
};
use parking_lot::RwLock;
use quick_xml::Reader as XmlReader;
use std::{
    borrow::Cow,
    convert::TryFrom,
    mem::MaybeUninit,
    num::ParseIntError,
    str::{self, FromStr},
    sync::Arc,
};
use tokio_native_tls::TlsConnector;

use crate::{
    common::{account_api_endpoints, DEFAULT_ACCOUNT_SERVER_HOST},
    xml::{
        agreement::{AgreementKindValue, Agreements},
        error as error_xml,
        errors::Error as XmlErrorExtension,
        timezone::Timezones,
    },
};
use ralsei_keypairs::{CTR_COMMON_1, NINTENDO_CACERTS, WUP_ACCOUNT_1};
use ralsei_model::{
    console::common::{Console, HeaderConstructionError, Kind as ConsoleKind},
    network::Nnid,
    server::Kind as ServerKind,
};
use ralsei_util::xml::{
    errors::Error as XmlError,
    framework::{BufferPool, FromXml},
    GLOBAL_BUFFER_POOL,
};

macro handle_error_xml($self:ident, $response:ident) {{
    let mut error = error_xml::Errors::default();
    error
        .from_xml(
            &mut XmlReader::from_reader(
                $response
                    .into_body()
                    .try_fold(Vec::new(), |mut accumulator, chunk| async move {
                        accumulator.extend_from_slice(&chunk);
                        Ok(accumulator)
                    })
                    .await?
                    .as_slice(),
            ),
            $self.pool.clone(),
        )
        .await?;
    Err(error.into())
}}

/// A client for the Nintendo Network account servers
///
/// -- add more descriptive documentation here --
pub struct Client<'a, C: Console<'a> + Send + Clone> {
    /// The host of the account server (not the api endpoint)
    ///
    /// If no value is provided, it is initialized with [`DEFAULT_ACCOUNT_SERVER_HOST`].
    pub host: RwLock<Cow<'a, str>>,

    /// The console data we are connecting to the account server with
    ///
    /// This field is used to generate a set of HTTP headers that are passed to the server in
    /// requests to mimic a real console.
    pub console: Arc<RwLock<C>>,

    /// The pool we are storing [`Vec<u8>`]s in.
    ///
    /// This is used to speed up XML deserialization by reusing memory as much as possible,
    /// removing the overhead of memory allocation
    pub pool: BufferPool,

    /// A cache of the headers to avoid recalling [`Console::http_headers`]
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
    /// If no value for the `pool` parameter is provided, it is initialized to a global pool of
    /// vectors with a fixed capacity of 100
    ///
    /// [`host`]: #structfield.host
    pub fn new<'b>(
        host: Option<Cow<'a, str>>,
        console: Arc<RwLock<C>>,
        identity: Option<Identity>,
        cacert_bundle: Option<Cow<'b, [Certificate]>>,
        pool: Option<BufferPool>,
    ) -> Result<Self, ClientError> {
        let host = host.unwrap_or(Cow::Borrowed(DEFAULT_ACCOUNT_SERVER_HOST));
        Ok(Client {
            host: RwLock::new(host.clone()),
            console: Arc::clone(&console),
            pool: if let Some(pool) = pool {
                pool
            } else {
                GLOBAL_BUFFER_POOL.clone()
            },
            cached_headers: RwLock::new(
                console
                    .read()
                    .http_headers(ServerKind::Account(Cow::Borrowed(&host)))?,
            ),
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
            .http_headers(ServerKind::Account(Cow::Borrowed(&self.host.read())))?;
        Ok(())
    }

    /// Execute a request using the provided [`Request`]
    #[inline]
    pub fn request(&self, mut request: Request<Body>) -> ResponseFuture {
        request
            .headers_mut()
            .extend(self.cached_headers.read().clone());
        self.http.request(request)
    }

    /// Check if a user with the given [`Nnid`] exists on the provided account server
    pub async fn does_user_exist(&self, nnid: Nnid<'_>) -> Result<bool, ClientError> {
        let mut path = nnid.0.into_owned();
        path.insert_str(0, account_api_endpoints::PEOPLE);
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
                let mut error = error_xml::Errors::default();
                error
                    .from_xml(
                        &mut XmlReader::from_reader(
                            response
                                .into_body()
                                .try_fold(Vec::new(), |mut accumulator, chunk| async move {
                                    accumulator.extend_from_slice(&chunk);
                                    Ok(accumulator)
                                })
                                .await?
                                .as_slice(),
                        ),
                        self.pool.clone(),
                    )
                    .await?;
                match error.first_code() {
                    Some(error_xml::ErrorCode::Known(
                        error_xml::ErrorCodeValue::AccountIdExists,
                    )) => Ok(true),
                    _ => Err(error.into()),
                }
            }
            status => Err(ClientError::UnexpectedStatusCode(status)),
        }
    }

    /// Retrieve [`Agreements`] from the provided account server based on their
    /// [kind](AgreementKindValue), their associated [`CountryCode`], and
    /// [version](AgreementVersionParameter)
    pub async fn agreements(
        &self,
        kind: AgreementKindValue,
        country: CountryCode,
        version: AgreementVersionParameter,
    ) -> Result<Agreements<'_>, ClientError> {
        let kind = kind.to_string();
        let version = version.to_string();

        // the length is determined by the length of
        // - the agreements path string - 54
        // - the kind - dynamic
        // - the slash - 1
        // - the country - 2
        // - the slash - 1
        // - the version - dynamic
        let mut path = String::with_capacity(58 + kind.len() + version.len());

        path.push_str(account_api_endpoints::AGREEMENTS);
        path.push_str(&kind);
        path.push('/');
        path.push_str(country.alpha2());
        path.push('/');
        path.push_str(&version);

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
            200 => {
                let mut agreements = Agreements::default();
                agreements
                    .from_xml(
                        &mut XmlReader::from_reader(
                            response
                                .into_body()
                                .try_fold(Vec::new(), |mut accumulator, chunk| async move {
                                    accumulator.extend_from_slice(&chunk);
                                    Ok(accumulator)
                                })
                                .await?
                                .as_slice(),
                        ),
                        self.pool.clone(),
                    )
                    .await?;
                Ok(agreements)
            }
            400 | 401 => handle_error_xml!(self, response),
            status => Err(ClientError::UnexpectedStatusCode(status)),
        }
    }

    /// Retrieve [`Timezones`] from the provided account server based on their associated
    /// [`CountryCode`] and [language](Iso639_1)
    pub async fn timezones(
        &self,
        country: CountryCode,
        language: Iso639_1,
    ) -> Result<Timezones<'_>, ClientError> {
        // the length is determined by the length of
        // - the timezones path string - 27
        // - the country - 2
        // - the slash - 1
        // - the language - 2
        let mut path: [MaybeUninit<u8>; 32] = MaybeUninit::uninit_array();

        MaybeUninit::write_slice(&mut path[..27], account_api_endpoints::TIMEZONES.as_bytes());
        MaybeUninit::write_slice(&mut path[27..29], country.alpha2().as_bytes());
        path[29] = MaybeUninit::new(b'/');
        MaybeUninit::write_slice(&mut path[30..], language.code().as_bytes());

        let response = self
            .request(
                Request::builder()
                    .method("GET")
                    .uri(
                        Uri::builder()
                            .scheme("https")
                            .authority(Authority::try_from(self.host.read().as_ref())?)
                            .path_and_query(PathAndQuery::try_from(unsafe {
                                // SAFETY: `path` has been initialized
                                // SAFETY: all bytes are written from valid utf8
                                str::from_utf8_unchecked(MaybeUninit::slice_assume_init_ref(&path))
                            })?)
                            .build()?,
                    )
                    .version(HttpVersion::HTTP_11)
                    .body(Body::empty())?,
            )
            .await?;
        match response.status().as_u16() {
            200 => {
                let mut timezones = Timezones::default();
                timezones
                    .from_xml(
                        &mut XmlReader::from_reader(
                            response
                                .into_body()
                                .try_fold(Vec::new(), |mut accumulator, chunk| async move {
                                    accumulator.extend_from_slice(&chunk);
                                    Ok(accumulator)
                                })
                                .await?
                                .as_slice(),
                        ),
                        self.pool.clone(),
                    )
                    .await?;
                Ok(timezones)
            }
            400 | 401 => handle_error_xml!(self, response),
            status => Err(ClientError::UnexpectedStatusCode(status)),
        }
    }

    /// Retrieve the current time according to the account server, in UTC
    pub async fn time(&self) -> Result<DateTime<Utc>, ClientError> {
        let response = self
            .request(
                Request::builder()
                    .method("GET")
                    .uri(
                        Uri::builder()
                            .scheme("https")
                            .authority(Authority::try_from(self.host.read().as_ref())?)
                            .path_and_query(PathAndQuery::try_from(account_api_endpoints::TIME)?)
                            .build()?,
                    )
                    .version(HttpVersion::HTTP_11)
                    .body(Body::empty())?,
            )
            .await?;
        match response.status().as_u16() {
            200 => {
                let integer_timestamp = i64::from_str(
                    response
                        .headers()
                        .get("X-Nintendo-Date")
                        .ok_or(ClientError::MissingHeader("X-Nintendo-Date"))?
                        .to_str()?,
                )?;
                Ok(Utc
                    .timestamp_millis_opt(integer_timestamp)
                    .single()
                    .ok_or(ClientError::TimestampParseError(integer_timestamp))?)
            }
            401 => handle_error_xml!(self, response),
            status => Err(ClientError::UnexpectedStatusCode(status)),
        }
    }

    /// Map one valid id on the network to another
    pub async fn convert_id(&self) -> Result<(), ClientError> {
    }
}

/// An enumeration over the ways a version can be represented to the agreement xml retrieval
/// endpoint of an account server
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum AgreementVersionParameter {
    Latest,
    Version(u16),
}

impl ToString for AgreementVersionParameter {
    fn to_string(&self) -> String {
        match &self {
            Self::Latest => "@latest".to_string(),
            Self::Version(version_number) => version_number.to_string(),
        }
    }
}

/// A list of possible errors encountered while using the [`Client`]
#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    /// An error encountered when the provided [`Kind`](ConsoleKind) of console is not supported
    #[error("`{0}` is an unsupported console Kind")]
    UnsupportedConsoleKind(ConsoleKind),

    /// An error encountered when the header values provided by the [`Console`] are invalid
    #[error("An error was encountered while constructing headers")]
    HeaderConstructionError(#[from] HeaderConstructionError),

    /// An error encountered while using the native tls implementation
    #[error("An error was encountered while using the native tls implementation")]
    NativeTlsError(#[from] NativeTlsError),

    /// An error encountered if the Nintendo Network API raises an error
    #[error("An error was encountered while using the Nintendo Network account API")]
    ErrorXml(#[from] error_xml::Errors<'static>), //TODO(superwhiskers): look into the necessity of this being 'static

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
    XmlError(#[from] XmlError<XmlErrorExtension>),

    /// An error was encountered while deserializing XML
    #[error("An error was encountered while deserializing XML")]
    XmlError(#[from] XmlError<XmlErrorExtension>),

    /// The Nintendo Network API returned an unexpected status code
    #[error("The Nintendo Network API returned an unexpected status code, `{0}`")]
    UnexpectedStatusCode(u16),

    /// The Nintendo Network API returned a response that lacks an expected header
    #[error("The Nintendo Network API returned a response that lacked an expected header, `{0}`")]
    MissingHeader(&'static str),

    /// An error encountered when a header value is unable to be interpreted as a &str
    #[error("An error was encountered while trying to interpret a header value as a &str")]
    HeaderValueToStrError(#[from] HeaderValueToStrError),

    /// An error encountered when a string cannot be parsed as an integer
    #[error("An error was encountered while trying to parse a string as an integer")]
    IntegerParseError(#[from] ParseIntError),

    // /// An error encountered when an integer timestamp does not have a single representation
}
