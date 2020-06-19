//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use dyn_clone::clone_box;
use http::{HeaderMap, HeaderValue};
use hyper::{
    client::{Client as HttpClient, HttpConnector},
    Body,
};
use hyper_rustls::HttpsConnector;
use parking_lot::RwLock;
use pem::{self, PemError};
use rustls::{Certificate, ClientConfig as RustlsClientConfig, PrivateKey, TLSError};
use std::borrow::Cow;
use thiserror::Error;

use crate::{
    keypairs::{CTR_COMMON_1_CERT, CTR_COMMON_1_KEY, WIIU_COMMON_1_CERT, WIIU_COMMON_1_KEY},
    model::{
        console::common::{Console, HeaderConstructionError, Kind as ConsoleKind},
        server::{ServerKind, DEFAULT_ACCOUNT_SERVER_HOST},
    },
};

/// A client for the Nintendo Network account servers
///
/// -- add more descriptive documentation here --
pub struct Client<'a> {
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
    pub console: RwLock<Box<dyn Console<'a> + Send>>,

    /// A cache of the headers to avoid recalling [`Console.http_headers()`]
    ///
    /// [`Console.http_headers()`]: ../../model/console/common/trait.Console.html#tymethod.http_headers
    pub(crate) cached_headers: RwLock<HeaderMap<HeaderValue>>,

    /// The HTTP client used to make requests to the account server
    pub(crate) http: HttpClient<HttpsConnector<HttpConnector>, Body>,
}

impl<'a> Client<'a> {
    /// Create a new Client using the provided [`Console`]
    ///
    /// If no value for the `host` parameter is provided, the corresponding struct field, [`host`],
    /// is initialized to [`DEFAULT_ACCOUNT_SERVER_HOST`].
    ///
    /// [`Console`]: ../../model/console/common/trait.Console.html
    /// [`host`]: #structfield.host
    /// [`DEFAULT_ACCOUNT_SERVER_HOST`]: ../../model/server/constant.DEFAULT_ACCOUNT_SERVER_HOST.html
    pub fn new<'b>(
        host: Option<Cow<'a, str>>,
        console: Box<dyn Console<'a> + Send>,
        keypair: Option<(Vec<Certificate>, PrivateKey)>,
    ) -> Result<Self, ClientError> {
        let host = host.unwrap_or(Cow::Borrowed(DEFAULT_ACCOUNT_SERVER_HOST));
        Ok(Client {
            host: RwLock::new(host.clone()),
            console: RwLock::new(clone_box(&*console)),
            cached_headers: RwLock::new(console.http_headers(ServerKind::Account(&host))?),
            http: HttpClient::builder().build(HttpsConnector::from((HttpConnector::new(), {
                let mut config = RustlsClientConfig::new();
                let (certs, key) = if let Some(keypair) = keypair {
                    keypair
                } else {
                    match console.kind() {
                        ConsoleKind::N3ds => (
                            vec![Certificate(pem::parse(CTR_COMMON_1_CERT)?.contents)],
                            PrivateKey(pem::parse(CTR_COMMON_1_KEY)?.contents),
                        ),
                        ConsoleKind::WiiU => (
                            vec![Certificate(pem::parse(WIIU_COMMON_1_CERT)?.contents)],
                            PrivateKey(pem::parse(WIIU_COMMON_1_KEY)?.contents),
                        ),
                        kind => return Err(ClientError::UnsupportedConsoleKind(kind)),
                    }
                };
                config.set_single_client_cert(certs, key)?;
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
    pub fn refresh_header<'b>(&self) -> Result<(), HeaderConstructionError> {
        Ok(*self.cached_headers.write() = self
            .console
            .read()
            .http_headers(ServerKind::Account(&self.host.read()))?)
    }
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

    /// An error encountered if PEM-encoded data is found to be invalid
    #[error("An error was encountered while parsing a PEM file")]
    PemError(#[from] PemError),

    /// An error encountered if a TLSError arises
    #[error("An error was encountered while using the TLS protocol")]
    TLSError(#[from] TLSError),

    #[doc(hidden)]
    #[error("You shouldn't be seeing this error. Please file an issue on the git repository")]
    NonExhaustive,
}
