//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020-2021 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

//! This module contains various miscellaneous macros and other "utility" items that don't have
//! anything else to be grouped with

//TODO(superwhiskers): consider refactoring this to generate functions that create urls to
// endpoints in order to handle parametric urls better:
//
// example:
// ```
// /content/agreements/:agreement/:country-code/:version
// ```
//
// usage:
// ```
// account_api_endpoints::agreements("Nintendo-Network-EULA", "US", "@latest")
// //=> /content/agreements/Nintendo-Network-EULA/US/@latest
//
// potentially consider making this a macro that accepts identifiers or strings for any parameter,
// and can generate an entirely static url if only strings are provided, and a fully dynamic url
// where the parameters are if identifiers are provided. however, this is obviously an
// overcomplication of how to handle this

/// A macro designed to simplify creation of constants of paths for various api endpoints
pub macro generate_api_endpoints($doc:literal, $module_identifier:ident as $base_endpoint:literal => [$($name:ident = $path:literal),+]) {
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
