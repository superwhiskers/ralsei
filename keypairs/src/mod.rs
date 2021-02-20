//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020-2021 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

// naming scheme: <ctr | cafe | nintendo> - <common | cacert> - <additional information>
//
// common appears to be used for data that is common to all consoles, which is why it is a part of
// the naming scheme for constants here. there is no chance that i will upload console-unique
// information. do not ask
//
// the password for all pkcs12 identities is "ralsei"

// client identities

/// The client identity used by the 3ds in authentication to numerous Nintendo HTTP services
pub const CTR_COMMON_1: &[u8] = include_bytes!("ctr-common-1.pkcs12");

/// The client identity used by the WiiU in authentication to numerous Nintendo HTTP services
pub const WUP_COMMON_1: &[u8] = include_bytes!("wup-common-1.pkcs12");

/// The client identity used by the WiiU in authentication to the Nintendo Network account service
pub const WUP_ACCOUNT_1: &[u8] = include_bytes!("wup-account-1.pkcs12");

// server-side certificates used for verification

pub const NINTENDO_CACERTS: [&[u8]; 6] = [
    NINTENDO_CACERT_CA_CERT,
    NINTENDO_CACERT_CA_G2_CERT,
    NINTENDO_CACERT_CA_G3_CERT,
    NINTENDO_CACERT_CLASS2_CA_CERT,
    NINTENDO_CACERT_CLASS2_CA_G2_CERT,
    NINTENDO_CACERT_CLASS2_CA_G3_CERT,
];

pub const NINTENDO_CACERT_CA_CERT: &[u8] = include_bytes!("nintendo-cacert-ca.crt.der");

pub const NINTENDO_CACERT_CA_G2_CERT: &[u8] = include_bytes!("nintendo-cacert-ca-g2.crt.der");

pub const NINTENDO_CACERT_CA_G3_CERT: &[u8] = include_bytes!("nintendo-cacert-ca-g3.crt.der");

pub const NINTENDO_CACERT_CLASS2_CA_CERT: &[u8] =
    include_bytes!("nintendo-cacert-class2-ca.crt.der");

pub const NINTENDO_CACERT_CLASS2_CA_G2_CERT: &[u8] =
    include_bytes!("nintendo-cacert-class2-ca-g2.crt.der");

pub const NINTENDO_CACERT_CLASS2_CA_G3_CERT: &[u8] =
    include_bytes!("nintendo-cacert-class2-ca-g3.crt.der");
