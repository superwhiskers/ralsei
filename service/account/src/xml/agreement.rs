//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020-2021 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use chrono::{offset::Utc, DateTime};
use isocountry::CountryCode;
use isolanguage_1::LanguageCode;
use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Reader, Writer,
};
use std::{
    borrow::Cow,
    error, fmt,
    io::{BufRead, Read, Write},
    str::FromStr,
};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

use ralsei_util::xml::{
    errors::{Error as XmlError, FormattingError, Result},
    framework::{BufferPool, FromXml, ToXml},
    helpers::{
        generate_xml_field_read_by_propagation, generate_xml_field_write,
        generate_xml_field_write_by_propagation, generate_xml_struct_read,
        generate_xml_struct_read_check,
    },
};

//TODO(superwhiskers): hook this into the custom serialization/deserialization infrastructure

/// A representation of a Nintendo Network EULA document
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Agreements<'a> {
    /// A vector of [`Agreement`] types
    ///
    /// [`Agreement`]: ./struct.Agreement.html
    pub agreements: Vec<Agreement<'a>>,
}

impl<'a> Agreements<'a> {
    /// Returns the first [`Agreement`] or `None` if there are none
    ///
    /// [`Agreement`]: ./struct.Agreement.html
    pub fn first(&self) -> Option<&Agreement> {
        self.agreements.first()
    }

    /// Returns the first [`Agreement`]'s [`AgreementKind`] or `None` if there are none
    ///
    /// [`Agreement`]: ./struct.Agreement.html
    /// [`AgreementKind`]: ./enum.AgreementKind.html
    pub fn first_kind(&self) -> Option<&AgreementKind> {
        self.agreements.first().map(|v| &v.kind)
    }
}

/// A Nintendo Network account server agreement
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Agreement<'a> {
    /// The country code representing the country the agreement is intended for in the iso 3166-1
    /// alpha-2 format
    pub country: Option<CountryCode>,

    /// The language code representing the language the agreement is written it, within the iso
    /// 639-1 language code format
    pub language: Option<LanguageCode>,

    /// The date at which this specific agreement was published, formatted as specified by iso 8601
    pub publish_date: Option<DateTime<Utc>>,

    /// The text to be displayed on the `accept` button
    pub accept_text: Option<Cow<'a, str>>,

    /// The text to be displayed on the `cancel` button
    pub cancel_text: Option<Cow<'a, str>>,

    /// The title of the agreement
    pub title_text: Option<Cow<'a, str>>,

    /// The body of the agreement
    pub body_text: Option<Cow<'a, str>>,

    /// The kind of agreement
    pub kind: AgreementKind<'a>,

    /// The agreement's version
    pub version: Option<u16>,
}

//TODO(superwhiskers): add a Display impl for this, as well as any other necessary trait
// implementations for convenience

/// A container for a Nintendo Network account server agreement kind, handling unknown kinds as
/// well as known ones
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum AgreementKind<'a> {
    Known(AgreementKindValue),

    Unknown(Cow<'a, str>),
}

/// An enumeration over possible [`Agreement`] kinds
///
/// [`Agreement`]: ./struct.Agreement.html
#[non_exhaustive]
#[derive(
    IntoStaticStr, AsRefStr, EnumString, Display, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord,
)]
pub enum AgreementKindValue {
    #[strum(to_string = "NINTENDO-NETWORK-EULA")]
    Eula,
    // TODO(superwhiskers): figure out all possible agreements, if there even are more
}
