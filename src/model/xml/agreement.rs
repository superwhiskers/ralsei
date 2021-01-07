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

//TODO(superwhiskers): hook this into the custom serialization/deserialization infrastructure

/// A representation of a Nintendo Network EULA document
#[derive(Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Agreements {
    /// A vector of [`Agreement`] types
    ///
    /// [`Agreement`]: ./struct.Agreement.html
    pub agreements: Vec<Agreement>,
}

/// A Nintendo Netwark account server agreement
#[derive(Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Agreement {
    /// The country code representing the country the agreement is intended for in the iso 3166-1
    /// alpha-2 format
    pub country: Option<CountryCode>,

    /// The language code representing the language the agreement is written it, within the iso
    /// 639-1 language code format
    pub language: Option<LanguageCode>,

    /// The date at which this specific agreement was published, formatted as specified by iso 8601
    pub publish_date: Option<DateTime<Utc>>,
    // /// The text to show on the "accept" button
    // pub accept:
}
