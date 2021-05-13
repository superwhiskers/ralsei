//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020-2021 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

//! A collection of type definitions used for mimicking a Wii U console
//!
//! The main focal point of this module is the [`ConsoleWiiU`] structure, which implements the
//! [`Console`] trait. This type contains a wealth of information all filtered through and exposed
//! through the API of the [`Console`] trait
//!
//! Unlike the 3ds, the Wii U is not required to send the model as there is only really one "model"
//! of Wii U, so there is no extra Wii U-specific model enumeration in this module, as there is in
//! the 3ds' module
//!
//! [`Console3ds`]: ./struct.Console3ds.html
//! [`Console`]: ../common/trait.Console.html

use hyper::header::{self, HeaderMap, HeaderValue};
use isocountry::CountryCode;
use isolanguage_1::LanguageCode;
use std::borrow::Cow;
use thiserror::Error;

use crate::{
    certificate::Certificate,
    console::common::{
        Console, ConsoleSerial, Environment, HeaderConstructionError, InvalidSerialError,
        Kind as ConsoleKind, Region as ConsoleRegion, Type as ConsoleType,
    },
    server::Kind as ServerKind,
    title::{
        id::{TitleId, UniqueId},
        version::TitleVersion,
    },
};
use ralsei_util::builder::builder_set;

/// A builder-like type, used to ease in the creation of [`ConsoleWiiU`] types
///
/// [`ConsoleWiiU`]: ./struct.ConsoleWiiU.html
#[derive(Debug, Default)]
pub struct ConsoleWiiUBuilder<'a> {
    pub(crate) console: ConsoleWiiU<'a>,
}

impl<'a> ConsoleWiiUBuilder<'a> {
    /// "Builds" the builder type, returning the internal [`ConsoleWiiU`]
    ///
    /// [`ConsoleWiiU`]: ./struct.ConsoleWiiU.html
    fn build(self) -> ConsoleWiiU<'a> {
        self.console
    }

    /// Derives the [`UniqueId`] from the console's current [`TitleId`], producing the
    /// [`unique_id`] field
    ///
    /// [`UniqueId`]: ../../title/id/struct.UniqueId.html
    /// [`TitleId`]: ../../title/id/struct.TitleId.html
    /// [`unique_id`]: ./struct.ConsoleWiiU.html#structfield.unique_id
    pub fn derive_unique_id_from_title_id(&mut self) -> Result<&mut Self, ConsoleWiiUBuilderError> {
        self.console.unique_id = Some(
            self.console
                .title_id
                .as_ref()
                .ok_or(ConsoleWiiUBuilderError::DeriveableFieldEmpty)?
                .unique_id(),
        );
        Ok(self)
    }

    /// Derives the device id from the console's [`Certificate`], producing the [`device_id`] field
    ///
    /// [`Certificate`]: ../../certificate/struct.Certificate.html
    /// [`device_id`]: ./struct.ConsoleWiiU.html#structfield.device_id
    pub fn derive_device_id_from_device_certificate(
        &mut self,
    ) -> Result<&mut Self, ConsoleWiiUBuilderError> {
        self.console.device_id = self
            .console
            .device_certificate
            .as_ref()
            .ok_or(ConsoleWiiUBuilderError::DeriveableFieldEmpty)?
            .name
            .device_id();
        Ok(self)
    }

    /// Derives the [`Region`] from the console's [`ConsoleSerial`], producing the [`region`] field
    ///
    /// [`Region`]: ../common/enum.Region.html
    /// [`ConsoleSerial`]: ../common/struct.ConsoleSerial.html
    /// [`region`]: ./struct.ConsoleWiiU.html#structfield.region
    pub fn derive_region_from_serial(&mut self) -> Result<&mut Self, ConsoleWiiUBuilderError> {
        self.console.region = Some(
            self.console
                .serial
                .as_ref()
                .ok_or(ConsoleWiiUBuilderError::DeriveableFieldEmpty)?
                .region()?,
        );
        Ok(self)
    }

    /// Derives the [`Type`] from the console's [`ConsoleSerial`], producing the [`device_type`]
    /// field
    ///
    /// [`Type`]: ../common/enum.Type.html
    /// [`ConsoleSerial`]: ../common/struct.ConsoleSerial.html
    /// [`device_type`]: ./struct.ConsoleWiiU.html#structfield.device_type
    pub fn derive_device_type_from_serial(&mut self) -> Result<&mut Self, ConsoleWiiUBuilderError> {
        self.console.device_type = Some(
            self.console
                .serial
                .as_ref()
                .ok_or(ConsoleWiiUBuilderError::DeriveableFieldEmpty)?
                .device_type()?,
        );
        Ok(self)
    }

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
}

/// An enumeration over all possible errors that can occur when using a [`ConsoleWiiUBuilder`]
///
/// [`ConsoleWiiUBuilder`]: ./struct.ConsoleWiiUBuilder.html
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum ConsoleWiiUBuilderError {
    /// An error encountered when using a [`Serial`]
    ///
    /// [`Serial`]: ../common/struct.Serial.html
    #[error("An error was encountered while using a Serial")]
    InvalidSerialError(#[from] InvalidSerialError),

    /// An error encountered when the field to derive another from has nothing in it
    #[error("The field to derive from is None")]
    DeriveableFieldEmpty,
}

/// A structure containing all possible information that can be used in the mimicking of a Wii U
///
/// All fields are optional, and if they are `None`, then they will be omitted wherever this
/// structure is used, unless that field is mandatory.
///
/// Usage of this structure implies that the mocked device's platform id (a value used in the
/// headers of requests to the account server and possible others) is `1`
#[non_exhaustive]
#[derive(Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ConsoleWiiU<'a> {
    /// inherent: `X-Nintendo-Platform-ID` = 1

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

    /// provides `X-Nintendo-Title-ID`
    pub title_id: Option<TitleId>,

    /// provides `X-Nintendo-Unique-ID`
    pub unique_id: Option<UniqueId>,

    /// provides `X-Nintendo-Application-Version`
    pub title_version: Option<TitleVersion>,

    /// provides `X-Nintendo-Device-Cert`
    /// guaranteed to be 384 bytes long
    pub device_certificate: Option<Certificate<'a>>,

    /// provides `Accept-Language`
    pub language: Option<LanguageCode>,
}

impl<'a> ConsoleWiiU<'a> {
    /// Creates a new [`ConsoleWiiU`] using the provided closure, which is passed a
    /// [`ConsoleWiiUBuilder`} to operate upon
    ///
    /// [`ConsoleWiiU`]: ./struct.ConsoleWiiU.html
    /// [`ConsoleWiiUBuilder`]: ./struct.ConsoleWiiUBuilder.html
    pub fn new<F>(f: F) -> Result<Self, ConsoleWiiUBuilderError>
    where
        F: for<'b> FnOnce(
            &'b mut ConsoleWiiUBuilder<'a>,
        )
            -> Result<&'b mut ConsoleWiiUBuilder<'a>, ConsoleWiiUBuilderError>,
    {
        let mut builder = ConsoleWiiUBuilder::default();
        f(&mut builder)?;
        Ok(builder.build())
    }

    /// Creates a new [`ConsoleWiiU`] from the provided [`ConsoleWiiUBuilder`]
    ///
    /// While there aren't many cases in which this would be used, it is left here for when
    /// avoiding closures is preferred
    ///
    /// [`ConsoleWiiU`]: ./struct.ConsoleWiiU.html
    /// [`ConsoleWiiUBuilder`]: ./struct.ConsoleWiiUBuilder.html
    pub fn from_builder(builder: ConsoleWiiUBuilder<'a>) -> Self {
        builder.build()
    }
}

impl<'a> Console<'a> for ConsoleWiiU<'_> {
    fn kind(&self) -> ConsoleKind {
        ConsoleKind::WiiU
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
                let _ = h.append("X-Nintendo-API-Version", "1".parse()?);

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
