//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020-2021 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use chrono::format::ParseError as DateTimeParseError;
use isocountry::CountryCodeParseErr as CountryCodeParseError;
use iso::language::Error as LanguageCodeParseError;
use std::num::ParseIntError;

use ralsei_util::xml::errors::ResultWithError;

/// A convenience alias for [`Result`] types within this module
///
/// [`Result`]: https://doc.rust-lang.org/nightly/std/result/enum.Result.html
pub type Result<T> = ResultWithError<T, Error>;

/// A specialized error type enumerating over errors that may occur specifically while dealing with
/// the xml structures defined in this module
#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An error that may arise while parsing a country code
    #[error("An error was encountered while parsing a country code")]
    CountryCodeParseError(#[from] CountryCodeParseError),

    /// An error that may arise while parsing a language code
    #[error("An error was encountered while parsing a language code")]
    LanguageCodeParseError(#[from] LanguageCodeParseError),

    /// An error that may arise while parsing a datetime
    #[error("An error was encountered while parsing a datetime")]
    DateTimeParseError(#[from] DateTimeParseError),

    /// An error that may arise while parsing an integer
    #[error("An error was encountered while parsing an integer")]
    IntegerParseError(#[from] ParseIntError),
}
