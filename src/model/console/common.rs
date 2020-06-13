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
use std::fmt;
use thiserror::Error;

use crate::model::server::ServerKind;

/// An abstraction over the various console-specific data structures
///
/// It provides methods to use the console's data without knowing about the console itself.
pub trait Console<'a> {
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
        server: ServerKind<'a>,
    ) -> Result<HeaderMap<HeaderValue>, HeaderConstructionError<'a>>;

    // given that most apis past the http-based ones are console-specific,
    // there's little need for more abstracted data tidbits to be implemented
}

/// A list of possible errors encountered while constructing headers
///
/// It is used by all implementors of [`Console`].
///
/// [`Console`]: ./trait.Console.html
#[derive(Error, Debug)]
pub enum HeaderConstructionError<'a> {
    /// An error returned when one of your console's details is invalid in the context of a header
    /// value.
    #[error("`{0}` is an invalid header value")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),

    /// An error returned when the server that you are requesting headers from has no corresponding
    /// headers to be recieved from the console that you intend to mimic.
    #[error("`{0:?}` is not an implemented ServerKind")]
    UnimplementedServerKind(ServerKind<'a>),
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

/// List of possible console regions.
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
