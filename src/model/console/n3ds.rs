//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use http::header::{self, HeaderMap, HeaderValue};
use isocountry::CountryCode;
use isolanguage_1::LanguageCode;
use std::borrow::Cow;
use strum_macros::{Display, EnumString};

use crate::model::{
    console::common::{Console, Environment, HeaderConstructionError, Region, Type},
    server::ServerKind,
    title::{id::TitleId, version::TitleVersion},
};

/// the model of the console. data for a 3ds-only header
#[derive(EnumString, Display, Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Model {
    #[strum(to_string = "CTR")]
    Nintendo3ds,
    #[strum(to_string = "SPR")]
    Nintendo3dsXL,
    #[strum(to_string = "KTR")]
    NintendoNew3ds,
    #[strum(to_string = "FTR")]
    Nintendo2ds,
    #[strum(to_string = "RED")]
    NintendoNew3dsXL,
    #[strum(to_string = "JAN")]
    NintendoNew2dsXL,
}

/// information required to emulate a 3ds.
///
/// any fields for which None is provided will be
/// omitted in the header output.
#[derive(Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Console3ds<'a> {
    /// inherent: `X-Nintendo-Platform-ID` = 0

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
    pub country: Option<CountryCode>,

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
    pub title_version: Option<TitleVersion>,

    /// provides `X-Nintendo-Device-Cert`
    /// guaranteed to be 384 bytes long
    pub device_certificate: Option<Vec<u8>>,

    /** unsure */

    /// provides `Accept-Language`
    pub language: Option<LanguageCode>,

    /** 3ds-specific */

    /// provides `X-Nintendo-API-Version`
    pub api_version: Option<u16>,

    /// provides `X-Nintendo-Device-Model`
    pub device_model: Option<Model>,
}

impl<'a> Console<'a> for Console3ds<'_> {
    fn http_headers(
        &self,
        server: ServerKind<'a>,
    ) -> Result<HeaderMap<HeaderValue>, HeaderConstructionError<'a>> {
        match server {
            ServerKind::Account(_) => {
                let mut h = HeaderMap::new();

                // unsure if this is necessary
                // let _ = h.append(header::USER_AGENT, "".parse().unwrap());

                // while i know that this will not fail, i add a proper error check
                // anyway
                let _ = h.append("X-Nintendo-Platform-ID", "0".parse()?);

                if let Some(device_type) = self.device_type {
                    let _ = h.append(
                        "X-Nintendo-Device-Type",
                        HeaderValue::from(device_type as u16),
                    );
                }

                if let Some(device_id) = self.device_id {
                    let _ = h.append("X-Nintendo-Device-ID", HeaderValue::from(device_id));
                }

                if let Some(serial) = &self.serial {
                    let _ = h.append("X-Nintendo-Serial-Number", serial.parse()?);
                }

                if let Some(system_version) = &self.system_version {
                    let _ = h.append("X-Nintendo-System-Version", system_version.parse()?);
                }

                if let Some(region) = self.region {
                    let _ = h.append("X-Nintendo-Region", HeaderValue::from(region as u16));
                }

                if let Some(country) = self.country {
                    let _ = h.append("X-Nintendo-Country", country.alpha2().parse()?);
                }

                if let Some(language) = self.language {
                    let _ = h.append(header::ACCEPT_LANGUAGE, language.code().parse()?);
                }

                if let Some(client_id) = &self.client_id {
                    let _ = h.append("X-Nintendo-Client-ID", client_id.parse()?);
                }

                if let Some(client_secret) = &self.client_secret {
                    let _ = h.append("X-Nintendo-Client-Secret", client_secret.parse()?);
                }

                let _ = h.append(header::ACCEPT, "*/*".parse()?);

                if let Some(api_version) = self.api_version {
                    let _ = h.append("X-Nintendo-API-Version", HeaderValue::from(api_version));
                }

                if let Some(fpd_version) = self.fpd_version {
                    let _ = h.append("X-Nintendo-FPD-Version", HeaderValue::from(fpd_version));
                }

                if let Some(environment) = self.environment {
                    let _ = h.append("X-Nintendo-Environment", environment.to_string().parse()?);
                }

                if let Some(title_id) = self.title_id {
                    let _ = h.append("X-Nintendo-Title-ID", HeaderValue::from(title_id.0));
                    let _ = h.append(
                        "X-Nintendo-Unique-ID",
                        format!("{:0>5X}", u32::from(title_id.unique_id().0)).parse()?,
                    );
                }

                if let Some(title_version) = self.title_version {
                    let _ = h.append(
                        "X-Nintendo-Application-Version",
                        format!("{:04X}", u16::from(title_version.major())).parse()?,
                    );
                }

                if let Some(device_model) = self.device_model {
                    let _ = h.append("X-Nintendo-Device-Model", device_model.to_string().parse()?);
                }

                if let Some(device_certificate) = &self.device_certificate {
                    let _ = h.append(
                        "X-Nintendo-Device-Cert",
                        HeaderValue::from_bytes(&device_certificate)?,
                    );
                }

                Ok(h)
            }
            // ServerKind::Mii(host) => [()].into_iter().collect::<HeaderMap<HeaderValue>>(),
            _ => Err(HeaderConstructionError::UnimplementedServerKind(server)),
        }
    }
}
