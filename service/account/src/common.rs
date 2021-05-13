//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020-2021 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

/// The default (official Nintendo) host for the account server
pub const DEFAULT_ACCOUNT_SERVER_HOST: &str = "account.nintendo.net";

//TODO(superwhiskers): make a proper macro for this as the existing one doesn't work at all with
//                     multiple endpoints
/// A module containing paths to various endpoints of the Nintendo Network account server
pub mod account_api_endpoints {
    pub const PEOPLE: &str = "/v1/api/people/";
    pub const EULAS: &str = "/v1/api/content/agreements/Nintendo-Network-EULA/";
}
