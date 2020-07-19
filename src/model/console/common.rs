//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

//! A collection of type definitions for data shared between consoles
//!
//! The type included here that you will be interacting with most will be the [`Console`] trait,
//! which abstracts over console data to allow you to use it without being aware of which console
//! it is.
//!
//! Aside from that, there are a number of other shared type definitions used in forming a
//! console's data that are defined here, such as the [`Region`] enumeration.
//!
//! [`Console`]: ./trait.Console.html
//! [`Region`]: ./enum.Region.html

use http::header::{HeaderMap, HeaderValue, InvalidHeaderValue};
use num_derive::{FromPrimitive, ToPrimitive};
use std::{borrow::Cow, fmt, num::ParseIntError, str::FromStr};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
use thiserror::Error;

use crate::model::{certificate::CertificateError, server::Kind as ServerKind};

/// An abstraction over the various console-specific data structures
///
/// It provides methods to use the console's data without knowing about the console itself.
pub trait Console<'a> {
    /// Returns the [`Kind`] of console that this [`Console`] instance is emulating
    ///
    /// This is mainly necessary for things like automatic client certificate application, to avoid
    /// using the wrong client certificate for the console that we are attempting to emulate.
    ///
    /// [`Kind`]: ./enum.Kind.html
    /// [`Console`]: ./trait.Console.html
    fn kind(&self) -> Kind;

    /// Constructs a [`HeaderMap`] from the console's data used when contacting the specified
    /// [`ServerKind`].
    ///
    /// In the event that the console has no headers to provide to the chosen [`ServerKind`], it
    /// will return an error of [`HeaderConstructionError::UnimplementedServerKind`].
    ///
    /// If your console data is invalid in that it is too large/malformed and cannot be placed as a
    /// header's value, it will return an error of [`HeaderConstructionError::InvalidHeaderValue`].
    ///
    /// [`HeaderMap`]: https://docs.rs/http/0.2.1/http/header/struct.HeaderMap.html
    /// [`ServerKind`]: ../server/enum.ServerKind.html
    /// [`HeaderConstructionError::UnimplementedServerKind`]: ./enum.HeaderConstructionError.html#variant.UnimplementedServerKind
    /// [`HeaderConstructionError::InvalidHeaderValue`]: ./enum.HeaderConstructionError.html#variant.InvalidHeaderValue
    fn http_headers(
        &self,
        server: ServerKind<'_>,
    ) -> Result<HeaderMap<HeaderValue>, HeaderConstructionError>;

    // given that most apis past the http-based ones are console-specific,
    // there's little need for more abstracted data tidbits to be implemented
}

/// A list of possible errors encountered while constructing headers
///
/// It is used by all implementors of [`Console`].
///
/// [`Console`]: ./trait.Console.html
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum HeaderConstructionError {
    /// An error returned when one of your console's details is invalid in the context of a header
    /// value.
    #[error("One of your console's details is an invalid header value")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),

    /// An error returned when a [`Certificate`] could not be converted to or from bytes
    ///
    /// [`Certificate`]: ../../certificate/struct.Certificate.html
    #[error("A Certificate could not be converted to or from bytes")]
    CertificateError(#[from] CertificateError),

    /// An error returned when the server that you are requesting headers for has no corresponding
    /// headers to be recieved from the console that you intend to mimic.
    #[error("`{0:?}` is not an implemented ServerKind")]
    UnimplementedServerKind(&'static str),
}

/// A list of Nintendo consoles that can implement the [`Console`] trait
///
/// While it is entirely possible to create a Switch client, this is not listed here as there are
/// currently no plans to provide an implementation for one.
///
/// [`Console`]: ./trait.Console.html
#[non_exhaustive]
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
pub enum Kind {
    N3ds,
    WiiU,
}

/// Enumeration of possible (3ds/WiiU) console environments
///
/// While not console-specific, it is not accessible through the [`Console`] trait and must instead
/// be gotten through the underlying structure.
///
/// [`Console`]: ./trait.Console.html
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Environment {
    L(u8),
    D(u8),
    S(u8),
    T(u8),
    J(u8),
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::L(n) => "L".to_string() + &n.to_string(),
                Self::D(n) => "D".to_string() + &n.to_string(),
                Self::S(n) => "S".to_string() + &n.to_string(),
                Self::T(n) => "T".to_string() + &n.to_string(),
                Self::J(n) => "J".to_string() + &n.to_string(),
            }
        )
    }
}

/// Enumeration of possible console variants (Developer/Retail)
#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Type {
    Developer = 1,
    Retail = 2,
}

/// List of possible console regions
///
/// The way it appears to be implemented on the console itself matches the implementation of a
/// bitfield, but here it is instead represented as an enumeration
///
/// Side note: Of these regions, [`Region::Australia`] is not a game region, and instead takes
/// games from [`Region::Europe`].
///
/// [`Region::Australia`]: #variant.Australia
/// [`Region::Europe`]: #variant.Europe
#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Region {
    Japan = 0b0000001,
    UnitedStates = 0b0000010,
    Europe = 0b0000100,
    Australia = 0b0001000,
    China = 0b0010000,
    Korea = 0b0100000,
    Taiwan = 0b1000000,
}

/// A Nintendo console's serial
pub struct ConsoleSerial<'a>(pub Cow<'a, str>);

// TODO(superwhiskers): split out check digit generation to allow people to create a valid one?
// TODO(superwhiskers): make console structs use this to derive console region and for general
//                      user-friendliness

impl ConsoleSerial<'_> {
    /// Verifies the integer portion of the console's serial
    ///
    /// If no error was returned, the check succeeded. Otherwise, the specific error can be figured
    /// out by checking the returned [`InvalidSerialError`]
    ///
    /// [`InvalidSerialError`]: ./enum.InvalidSerialError.html
    pub fn check(&self) -> Result<(), InvalidSerialError> {
        let serial_number = self.number()?;
        (10 - (((((serial_number / 100000000) % 10)
            + ((serial_number / 1000000) % 10)
            + ((serial_number / 10000) % 10)
            + ((serial_number / 100) % 10))
            + ((((serial_number / 10000000) % 10)
                + ((serial_number / 100000) % 10)
                + ((serial_number / 1000) % 10)
                + ((serial_number / 10) % 10))
                * 3))
            % 10)
            == serial_number % 10)
            .then_some(())
            .ok_or(InvalidSerialError::CheckDigitInvalid)
    }

    /// Returns the integer portion of the serial (including check digit)
    pub fn number(&self) -> Result<u32, InvalidSerialError> {
        Ok(u32::from_str(
            self.0
                .get(match self.region()? {
                    Region::Japan
                    | Region::Europe
                    | Region::Australia
                    | Region::Korea
                    | Region::China => 3..12,
                    Region::UnitedStates | Region::Taiwan => 2..11,
                })
                .ok_or(InvalidSerialError::OutOfBounds)?,
        )?)
    }

    /// Returns the appropriate region for the region portion of the serial
    ///
    /// Currently, it does not touch the optional second letter of the region portion as there is
    /// currently no information as to its significance
    pub fn region(&self) -> Result<Region, InvalidSerialError> {
        Ok(
            match self
                .0
                .chars()
                .nth(1)
                .ok_or(InvalidSerialError::OutOfBounds)?
            {
                'J' => Region::Japan,
                'W' => Region::UnitedStates,
                'S' => Region::Taiwan,
                'E' => Region::Europe,
                'A' => Region::Australia,
                'K' => Region::Korea,
                'C' => Region::China,
                r => return Err(InvalidSerialError::InvalidRegion(r)),
            },
        )
    }
}

/// An enumeration over the possible errors that can occur when verifying a [`Serial`]
///
/// [`Serial`]: ./struct.Serial.html
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum InvalidSerialError {
    /// An error returned when the integer section of the serial is an invalid integer
    #[error("The integer portion of your serial was unable to be parsed")]
    InvalidInteger(#[from] ParseIntError),

    /// An error returned when the serial is invalid
    #[error("The integer portion of your serial has an invalid check digit")]
    CheckDigitInvalid,

    /// An error returned when the region is invalid
    #[error("The region portion of your serial is not valid")]
    InvalidRegion(char),

    /// An error returned when the data being operated upon is too small
    #[error("The provided serial number is not long enough")]
    OutOfBounds,
}
