//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use deadpool::managed::PoolError;
use quick_xml::Error as QuickXmlError;
use std::{error::Error as StdError, str::Utf8Error, string::FromUtf8Error};

/// A convenience alias for Result types within this module
pub type Result<T> = std::result::Result<T, Error>;

/// An enumeration over errors that can arise while working with the datatypes provided within this
/// module
#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An error that may arise while working with quick-xml
    #[error("An error was encountered while using the `quick-xml` library")]
    QuickXmlError(#[from] QuickXmlError),

    /// An error that may arise while using deadpool
    #[error("An error was encountered while using the `deadpool` library")]
    PoolError(#[from] PoolError<!>),

    /// An error that may arise while parsing bytes as UTF-8
    #[error("An error was encountered while parsing bytes as UTF-8")]
    Utf8Error(#[from] Utf8Error),

    /// An error that may arise while creating a [`String`] from a [`Vec`]
    ///
    /// [`String`]: https://doc.rust-lang.org/nightly/std/string/struct.String.html
    /// [`Vec`]: https://doc.rust-lang.org/nightly/std/vec/struct.Vec.html
    #[error("An error was encountered while creating a String from a Vec")]
    FromUTF8Error(#[from] FromUtf8Error),

    /// The XML is improperly formatted
    #[error("The XML document is improperly formatted")]
    Formatting(#[from] FormattingError),
}

/// An enumeration over possible XML document formatting errors
#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum FormattingError {
    /// A value was of the improper kind
    #[error("A value that parses to something of the type `{0}` was expected, but something else was encountered")]
    InvalidValue(&'static str, Box<dyn StdError>),

    /// An unexpected event was reached in the document
    #[error("An unexpected event (`{0}`) was reached in the document")]
    UnexpectedEvent(String),

    /// An unexpected opening tag was reached in the document
    #[error("An unexpected opening tag (`{0}`) was reached in the document")]
    UnexpectedOpeningTag(String),

    /// An unexpected closing tag was reached in the document
    #[error("An unexpected closing tag (`{0}`) was reached in the document")]
    UnexpectedClosingTag(String),
}
