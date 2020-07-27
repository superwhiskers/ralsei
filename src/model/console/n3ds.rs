//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

//! A collection of type definitions used for mimicking a 3ds console
//!
//! The main focal point of this module is the [`Console3ds`] structure, which implements the
//! [`Console`] trait. This type contains a wealth of information all filtered through and exposed
//! through the API of the [`Console`] trait
//!
//! Aside from that, there is also an enumeration over the 3ds models at [`Model`]
//!
//! [`Console3ds`]: ./struct.Console3ds.html
//! [`Console`]: ../common/trait.Console.html
//! [`Model`]: ./enum.Model.html

use base64;
use http::header::{self, HeaderMap, HeaderValue};
use isocountry::CountryCode;
use isolanguage_1::LanguageCode;
use std::borrow::Cow;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
use thiserror::Error;

use crate::{
    internal::builder_set,
    model::{
        certificate::Certificate,
        console::common::{
            Console, ConsoleSerial, Environment, HeaderConstructionError, InvalidSerialError,
            Kind as ConsoleKind, Model as ConsoleModel, Region as ConsoleRegion,
            Type as ConsoleType,
        },
        server::Kind as ServerKind,
        title::{
            id::{TitleId, UniqueId},
            version::TitleVersion,
        },
    },
};

/// The 3ds console's model. For more information, see [3dbrew]
///
/// [3dbrew]: https://www.3dbrew.org/wiki/Cfg:GetSystemModel#System_Model_Values
#[derive(
    IntoStaticStr,
    AsRefStr,
    EnumString,
    Display,
    Copy,
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    PartialOrd,
    Ord,
)]
pub enum Model {
    #[strum(to_string = "CTR")]
    Nintendo3ds,
    #[strum(to_string = "SPR")]
    Nintendo3dsXl,
    #[strum(to_string = "FTR")]
    Nintendo2ds,
    #[strum(to_string = "KTR")]
    NintendoNew3ds,
    #[strum(to_string = "RED")]
    NintendoNew3dsXl,
    #[strum(to_string = "JAN")]
    NintendoNew2dsXl,
}

/// A builder-like type, used to ease in the creation of [`Console3ds`] types
///
/// [`Console3ds`]: ./struct.Console3ds.html
#[derive(Debug, Default)]
pub struct Console3dsBuilder<'a> {
    pub(crate) console: Console3ds<'a>,
}

macro_rules! generate_serial_derivation_method {

}

impl<'a> Console3dsBuilder<'a> {
    /// "Builds" the builder type, returning the internal [`Console3ds`]
    ///
    /// [`Console3ds`]: ./struct.Console3ds.html
    fn build(self) -> Console3ds<'a> {
        self.console
    }

    /// Sets the [`title_id`] field to the provided [`TitleId`] and derives the [`UniqueId`] from
    /// it, producing the [`unique_id`] field
    ///
    /// [`title_id`]: ./struct.Console3ds.html#structfield.title_id
    /// [`TitleId`]: ../../title/id/struct.TitleId.html
    /// [`UniqueId`]: ../../title/id/struct.UniqueId.html
    /// [`unique_id`]: ./struct.Console3ds.html#structfield.unique_id
    pub fn title_id_and_unique_id(&mut self, title_id: TitleId) -> &mut Self {
        self.console.unique_id = Some(title_id.unique_id());
        self.console.title_id = Some(title_id);
        self
    }

    /// Sets the [`device_certificate`] field to the provided [`Certificate`] and derives the
    /// device id from it, producing the [`device_id`] field
    ///
    /// [`device_certificate`]: ./struct.Console3ds.html#structfield.device_certificate
    /// [`Certificate`]: ../../certificate/struct.Certificate.html
    /// [`device_id`]: ./struct.Console3ds.html#structfield.device_id
    pub fn device_certificate_and_device_id(
        &mut self,
        device_certificate: Certificate<'a>,
    ) -> &mut Self {
        self.console.device_id = device_certificate.name.device_id();
        self.console.device_certificate = Some(device_certificate);
        self
    }

    /// Sets the [`serial`] field to the provided [`ConsoleSerial`] and derives the [`Region`] from
    /// it, producing the [`region`] field
    ///
    /// [`serial`]: ./struct.Console3ds.html#structfield.serial
    /// [`ConsoleSerial`]: ../common/struct.ConsoleSerial.html
    /// [`Region`]: ../common/enum.Region.html
    /// [`region`]: ./struct.Console3ds.html#structfield.region
    pub fn serial_and_region(
        &mut self,
        serial: ConsoleSerial<'a>,
    ) -> Result<&mut Self, InvalidSerialError> {
        self.console.region = Some(serial.region()?);
        self.console.serial = Some(serial);
        Ok(self)
    }

    /// Sets the [`serial`] field to the provided [`ConsoleSerial`] and derives the [`Model`] from
    /// it, producing the [`device_model`] field
    ///
    /// [`serial`]: ./struct.Console3ds.html#structfield.serial
    /// [`ConsoleSerial`]: ../common/struct.ConsoleSerial.html
    /// [`Model`]: ./enum.Model.html
    /// [`device_model`]: ./struct.Console3ds.html#structfield.device_model
    pub fn serial_and_device_model(
        &mut self,
        serial: ConsoleSerial<'a>,
    ) -> Result<&mut Self, Console3dsBuilderError> {
        self.console.device_model = Some(match serial.device()? {
            (ConsoleModel::Nintendo3ds, _) => Model::Nintendo3ds,
            (ConsoleModel::Nintendo3dsXl, _) => Model::Nintendo3dsXl,
            (ConsoleModel::Nintendo2ds, _) => Model::Nintendo2ds,
            (ConsoleModel::NintendoNew3ds, _) => Model::NintendoNew3ds,
            (ConsoleModel::NintendoNew3dsXl, _) => Model::NintendoNew3dsXl,
            (ConsoleModel::NintendoNew2dsXl, _) => Model::NintendoNew2dsXl,
            (model, _) => return Err(Console3dsBuilderError::UnimplementedConsoleModel(model)),
        });
        self.console.serial = Some(serial);
        Ok(self)
    }

    /// Sets the [`serial`] field to the provided [`ConsoleSerial`] and derives the [`Type`] from
    /// it, producing the [`device_type`] field
    ///
    /// [`serial`]: ./struct.Console3ds.html#structfield.serial
    /// [`ConsoleSerial`]: ../common/struct.ConsoleSerial.html
    /// [`Type`]: ../common/enum.Type.html
    /// [`device_type`]: ./struct.Console3ds.html#structfield.device_type
    pub fn serial_and_device_type(
        &mut self,
        serial: ConsoleSerial<'a>,
    ) -> Result<&mut Self, Console3dsBuilderError> {
        self.console.device_type = Some(serial.device()?.1);
        self.console.serial = Some(serial);
        Ok(self)
    }

    // pub fn serial_region_and_device_model

    builder_set!("device_type", console, device_type, ConsoleType);
    builder_set!("device_id", console, device_id, u32);
    builder_set!("serial", console, serial, ConsoleSerial<'a>);
    builder_set!("system_version", console, system_version, TitleVersion);
    builder_set!("region", console, region, ConsoleRegion);
    builder_set!("country", console, country, CountryCode);
    builder_set!("client_id", console, client_id, Cow<'a, str>);
    builder_set!("client_secret", console, client_secret, Cow<'a, str>);
    builder_set!("fpd_version", console, fpd_version, u16);
    builder_set!("environment", console, environment, Environment);
    builder_set!("title_id", console, title_id, TitleId);
    builder_set!("unique_id", console, unique_id, UniqueId);
    builder_set!("title_version", console, title_version, TitleVersion);
    builder_set!(
        "device_certificate",
        console,
        device_certificate,
        Certificate<'a>
    );
    builder_set!("language", console, language, LanguageCode);
    builder_set!("api_version", console, api_version, u16);
    builder_set!("device_model", console, device_model, Model);
}

/// An enumeration over all possible errors that can occur when using a [`Console3dsBuilder`]
///
/// [`Console3dsBuilder`]: ./struct.Console3dsBuilder.html
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Console3dsBuilderError {
    /// An error encountered when using a [`Serial`]
    ///
    /// [`Serial`]: ../common/struct.Serial.html
    #[error("An error was encountered while using a Serial")]
    InvalidSerialError(#[from] InvalidSerialError),

    /// An error encountered when the provided [`Serial`] has a console model that has no
    /// corresponding model for a 3ds
    ///
    /// [`Serial`]: ../common/struct.Serial.html
    #[error("The provided Serial has a console model has no corresponding 3ds model")]
    UnimplementedConsoleModel(ConsoleModel),
}

/// A structure containing all possible information that can be used in the mimicking of a 3ds
///
/// All fields are optional, and if they are `None`, then they will be omitted wherever this
/// structure is used, unless that field is mandatory.
///
/// Usage of this structure implies that the mocked device's platform id (a value used in the
/// headers of requests to the account server and possibly others) is `0`
#[non_exhaustive]
#[derive(Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Console3ds<'a> {
    /// inherent: `X-Nintendo-Platform-ID` = 0

    /// provides `X-Nintendo-Device-Type`
    pub device_type: Option<ConsoleType>,

    /// provides `X-Nintendo-Device-ID`
    pub device_id: Option<u32>,

    /// provides `X-Nintendo-Serial-Number`
    pub serial: Option<ConsoleSerial<'a>>,

    /// provides `X-Nintendo-System-Version`
    pub system_version: Option<TitleVersion>,

    /// provides `X-Nintendo-Region`
    pub region: Option<ConsoleRegion>,

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

    /// provides: `X-Nintendo-Title-ID`
    pub title_id: Option<TitleId>,

    /// provides: `X-Nintendo-Unique-ID`
    pub unique_id: Option<UniqueId>,

    /// provides `X-Nintendo-Application-Version`
    pub title_version: Option<TitleVersion>,

    /// provides `X-Nintendo-Device-Cert`
    /// guaranteed to be 384 bytes long
    pub device_certificate: Option<Certificate<'a>>,

    /// provides `Accept-Language`
    pub language: Option<LanguageCode>,

    /** 3ds-specific */

    /// provides `X-Nintendo-API-Version`
    pub api_version: Option<u16>,

    /// provides `X-Nintendo-Device-Model`
    pub device_model: Option<Model>,
}

impl<'a> Console3ds<'a> {
    /// Creates a new [`Console3ds`] using the provided closure, which is passed a
    /// [`Console3dsBuilder`] to operate upon
    ///
    /// [`Console3ds`]: ./struct.Console3ds.html
    /// [`Console3dsBuilder`]: ./struct.Console3dsBuilder.html
    pub fn new<F>(f: F) -> Self
    where
        F: for<'b> FnOnce(&'b mut Console3dsBuilder<'a>) -> &'b mut Console3dsBuilder<'a>,
    {
        let mut builder = Console3dsBuilder::default();
        f(&mut builder);
        builder.build()
    }
}

impl<'a> Console<'a> for Console3ds<'_> {
    fn kind(&self) -> ConsoleKind {
        ConsoleKind::N3ds
    }

    fn http_headers(
        &self,
        server: ServerKind<'_>,
    ) -> Result<HeaderMap<HeaderValue>, HeaderConstructionError> {
        match server {
            ServerKind::Account(_) => {
                let mut h = HeaderMap::new();

                // unsure if this is necessary
                // let _ = h.append(header::USER_AGENT, "".parse().unwrap());

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
                    let _ = h.append("X-Nintendo-Serial-Number", serial.0.parse()?);
                }

                if let Some(system_version) = &self.system_version {
                    let _ = h.append(
                        "X-Nintendo-System-Version",
                        format!("{:0>4X}", system_version.0).parse()?,
                    );
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
                        format!("{:0>4X}", u16::from(title_version.major())).parse()?,
                    );
                }

                if let Some(device_model) = self.device_model {
                    let _ = h.append("X-Nintendo-Device-Model", device_model.to_string().parse()?);
                }

                if let Some(device_certificate) = &self.device_certificate {
                    let _ = h.append(
                        "X-Nintendo-Device-Cert",
                        HeaderValue::from_str(
                            base64::encode(device_certificate.to_bytes()?).as_ref(),
                        )?,
                    );
                }

                Ok(h)
            }
            _ => Err(HeaderConstructionError::UnimplementedServerKind(
                server.into(),
            )),
        }
    }
}
