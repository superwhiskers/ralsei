//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

pub const DEFAULT_ACCOUNT_SERVER_HOST: &str = "account.nintendo.net";
pub const DEFAULT_MII_CDN_HOST: &str = "mii-secure.account.nintendo.net";

/// an enumeration over the nintendo network server kinds.
/// each of them contains a url pointing to the host
///
/// the address provided should be the *host* of the server;
/// it should not be a url to the api endpoint.
#[derive(
    IntoStaticStr, AsRefStr, EnumString, Display, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord,
)]
pub enum ServerKind<'a> {
    Account(&'a str),
    Mii(&'a str),

    #[doc(hidden)]
    NonExhaustive,
}
