//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use http::header::{HeaderMap, HeaderValue, InvalidHeaderValue};
use num_derive::{FromPrimitive, ToPrimitive};
use std::fmt;
use thiserror::Error;

use crate::model::server::Kind as ServerKind;

/* console abstraction */

/// an abstraction over a nintendo console
/// implementing some common, necessary operations
/// in order to act like it without being
/// aware of what it is
pub trait Console<'a> {
    /// returns http headers for the provided server. if
    /// there are no headers to provide, None is returned instead
    fn http_headers(
        &self,
        server: ServerKind<'a>,
    ) -> Result<HeaderMap<HeaderValue>, HeaderConstructionError<'a>>;

    // given that most apis past the http-based ones are console-specific,
    // there's little need for more abstracted data tidbits to be implemented
}

/// all possible errors that can occur while constructing a skeleton
#[derive(Error, Debug)]
pub enum HeaderConstructionError<'a> {
    #[error("`{0}` is an invalid header value")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),

    #[error("`{0:?}` is not an implemented ServerKind")]
    UnimplementedServerKind(ServerKind<'a>),
}

/* common console information */

/// the environment of the console
///
/// to our knowledge, the integer must be
/// a single-digit decimal number
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Environment {
    /// this is probably the one you want
    /// with 1 as the value
    L(u8),

    /// most likely development environments
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

/// the variant of the device
#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Type {
    Developer = 1,
    Retail = 2,
}

/// the region of the console
///
/// of these regions, australia is
/// not an actual game region, and
/// instead takes european games
#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Region {
    Japan = 1,
    UnitedStates = 2,
    Europe = 4,
    Australia = 8,
    China = 16,
    Korea = 32,
    Taiwan = 64,
}
