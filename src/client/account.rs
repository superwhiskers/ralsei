//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use http::{HeaderMap, HeaderValue, Request};
use hyper::{
    client::{Client as HttpClient, HttpConnector, ResponseFuture},
    Body,
};
use hyper_rustls::HttpsConnector;
use parking_lot::RwLock;
use quick_xml::de::{from_str as xml_from_str, DeError as XmlDeError};
use rustls::{Certificate, ClientConfig as RustlsClientConfig, PrivateKey, TLSError};
use std::borrow::Cow;
use thiserror::Error;
use webpki::Error as WebpkiError;

use crate::{
    keypairs::{self, CTR_COMMON_1_CERT, CTR_COMMON_1_KEY, WUP_ACCOUNT_1_CERT, WUP_ACCOUNT_1_KEY},
    model::{
        console::common::{Console, HeaderConstructionError, Kind as ConsoleKind},
        server::{ServerKind, DEFAULT_ACCOUNT_SERVER_HOST},
        xml::error_xml,
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
    pub console: RwLock<C>,

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
    /// [`Console`]: ../../model/console/common/trait.Console.html
    /// [`host`]: #structfield.host
    /// [`DEFAULT_ACCOUNT_SERVER_HOST`]: ../../model/server/constant.DEFAULT_ACCOUNT_SERVER_HOST.html
    pub fn new(
        host: Option<Cow<'a, str>>,
        console: C,
        keypair: Option<(Vec<Certificate>, PrivateKey)>,
    ) -> Result<Self, ClientError> {
        let host = host.unwrap_or(Cow::Borrowed(DEFAULT_ACCOUNT_SERVER_HOST));
        Ok(Client {
            host: RwLock::new(host.clone()),
            console: RwLock::new(console.clone()),
            cached_headers: RwLock::new(console.http_headers(ServerKind::Account(&host))?),
            http: HttpClient::builder().build(HttpsConnector::from((HttpConnector::new(), {
                let mut config = RustlsClientConfig::new();
                let (certs, key) = if let Some(keypair) = keypair {
                    keypair
                } else {
                    match console.kind() {
                        ConsoleKind::N3ds => (
                            vec![Certificate(CTR_COMMON_1_CERT.to_vec())],
                            PrivateKey(CTR_COMMON_1_KEY.to_vec()),
                        ),
                        ConsoleKind::WiiU => (
                            vec![Certificate(WUP_ACCOUNT_1_CERT.to_vec())],
                            PrivateKey(WUP_ACCOUNT_1_KEY.to_vec()),
                        ),
                        kind => return Err(ClientError::UnsupportedConsoleKind(kind)),
                    }
                };
                config.set_single_client_cert(certs, key)?;
                config.root_store = keypairs::generate_cacert_bundle()?;
                config
            }))),
        })
    }

    /// Refresh the cached http headers
    ///
    /// This method blocks until a read lock can be acquired on the [`console`] and [`host`]
    /// fields, and a write lock can be acquired on the `cached_headers` field.
    ///
    /// [`console`]: #structfield.console
    /// [`host`]: #structfield.host
    pub fn refresh_header(&self) -> Result<(), HeaderConstructionError> {
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

    // Check if a user with the given NNID exists on the provided account server
}

/// A list of possible errors encountered while using the [`Client`]
///
/// [`Client`]: ./struct.Client.html
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

    /// An error encountered if a TLSError arises
    #[error("An error was encountered while using the TLS protocol")]
    TLSError(#[from] TLSError),

    /// An error encountered if a webpki::Error arises
    #[error("An error was encountered while using the `webpki` library")]
    WebpkiError(#[from] WebpkiError),

    #[doc(hidden)]
    #[error("You shouldn't be seeing this error. Please file an issue on the git repository")]
    NonExhaustive,
}
