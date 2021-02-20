//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020-2021 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use ralsei_util::misc::generate_api_endpoints;

/// The default (official Nintendo) host for the account server
pub const DEFAULT_ACCOUNT_SERVER_HOST: &str = "account.nintendo.net";

generate_api_endpoints!(
    "A module containing paths to various endpoints of the Nintendo Network account server",
    account_api_endpoints as "/v1/api" =>
        [PEOPLE = "/people"]
);
