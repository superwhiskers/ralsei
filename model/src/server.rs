//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020-2021 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

//TODO(superwhiskers): finish documentation
//! This module provides

use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

//TODO(superwhiskers): create ralsei-service-mii or something and shove this there
/// The default (official Nintendo) host for the Mii CDN
const DEFAULT_MII_CDN_HOST: &str = "mii-secure.account.nintendo.net";

/// An enumeration over the nintendo network server kinds, each containing a URL that points to the
/// host
///
/// The address provided should be the *host* of the server;
/// it should not be a url to the api endpoint
#[non_exhaustive]
#[derive(
    IntoStaticStr, AsRefStr, EnumString, Display, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord,
)]
pub enum Kind<'a> {
    Account(&'a str),
    Mii(&'a str),
}
