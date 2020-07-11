//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use hyper::header::{HeaderMap, HeaderValue};
use std::borrow::Cow;

use crate::model::{
    console::common::{
        Console, Environment, HeaderConstructionError, Kind as ConsoleKind, Region, Type,
    },
    server::Kind as ServerKind,
    title::id::TitleId,
};

/// information required to emulate a wii u
///
/// any fields for which None is provided will
/// be omitted in the header output.
#[non_exhaustive]
#[derive(Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ConsoleWiiU<'a> {
    /// inherent: `X-Nintendo-Platform-ID` = 1

    /// provides `X-Nintendo-Device-Type`
    pub device_type: Option<Type>,

    /// provides `X-Nintendo-Device-ID`
    pub device_id: Option<u64>,

    /// provides `X-Nintendo-Serial-Number`
    pub serial: Option<Cow<'a, str>>,

    /// provides `X-Nintendo-System-Version`
    pub system_version: Option<Cow<'a, str>>,

    /// provides `X-Nintendo-Region`
    pub region: Option<Region>,

    /// provides `X-Nintendo-Country`
    pub country: Option<Cow<'a, str>>,

    /// provides `X-Nintendo-Client-ID`
    pub client_id: Option<Cow<'a, str>>,

    /// provides `X-Nintendo-Client-Secret`
    pub client_secret: Option<Cow<'a, str>>,

    /// provides `X-Nintendo-FPD-Version`
    pub fpd_version: Option<u16>,

    /// provides `X-Nintendo-Environment`
    pub environment: Option<Environment>,

    /// the unique id is constructed from this, as it is a mere segment
    /// if this field is omitted, the following headers are also omitted as a result
    /// - `X-Nintendo-Title-ID`
    /// - `X-Nintendo-Unique-ID`
    pub title_id: Option<TitleId>,

    /// provides `X-Nintendo-Application-Version`
    pub application_version: Option<u16>,

    /// provides `X-Nintendo-Device-Cert`
    /// guaranteed to be 384 bytes long
    pub device_certificate: Option<Vec<u8>>,
}

impl<'a> Console<'a> for ConsoleWiiU<'_> {
    fn kind(&self) -> ConsoleKind {
        ConsoleKind::WiiU
    }

    fn http_headers(
        &self,
        server: ServerKind<'_>,
    ) -> Result<HeaderMap<HeaderValue>, HeaderConstructionError> {
        Err(HeaderConstructionError::UnimplementedServerKind(
            server.into(),
        ))
    }
}
