//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020-2021 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

//! This module provides

use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// The default (official Nintendo) host for the account server
pub const DEFAULT_ACCOUNT_SERVER_HOST: &str = "account.nintendo.net";

/// The default (official Nintendo) host for the Mii CDN
pub const DEFAULT_MII_CDN_HOST: &str = "mii-secure.account.nintendo.net";

/// A macro designed to simplify creation of constants of paths for various api endpoints
macro generate_api_endpoints($doc:literal, $module_identifier:ident as $base_endpoint:literal => [$($name:ident = $path:literal),+]) {
    #[doc = $doc]
    pub mod $module_identifier {
        $(
            #[doc = "A constant containing the absolute path for the `"]
            #[doc = $path]
            #[doc = "` api endpoint relative to the base endpoint `"]
            #[doc = $base_endpoint]
            #[doc = "`"]
            pub const $name: &str = concat!($base_endpoint, $path, "/");
        ),+
    }
}

generate_api_endpoints!(
    "A module containing paths to various endpoints of the Nintendo Network account server",
    account_api_endpoints as "/v1/api" => [PEOPLE = "/people"]
);

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
