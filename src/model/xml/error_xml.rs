//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use serde::{Deserialize, Serialize};
use std::{error, fmt};

/// A representation of a Nintendo Network error xml document
#[serde(rename = "errors")]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Errors {
    /// A vector of [`Error`] types
    ///
    /// [`Error`]: ./struct.Error.html
    pub errors: Vec<Error>,
}

impl fmt::Display for Errors {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // here, we assume that there is only one error present. this presumption has held true in
        // all known cases, so we believe that there is no need to handle the edge case of there
        // being multiple
        self.errors[0].fmt(formatter)
    }
}

impl error::Error for Errors {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // ditto
        if self.errors.len() == 0 {
            None
        } else {
            Some(&self.errors[0])
        }
    }
}

/// A Nintendo Network account server error
#[serde(rename = "error")]
#[derive(Serialize, Deserialize, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Error {
    /// The cause of the error
    pub cause: String,

    /// The error code. Appears to always be represented as four digits right-aligned
    pub code: u16,

    /// The error message
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Error code `{:0>4}` arised with message `{}` and cause `{}`",
            self.code, self.message, self.cause
        )
    }
}

impl error::Error for Error {} // no actual implementation necessary. the default ones suffice
