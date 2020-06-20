//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use rustls::{Certificate, RootCertStore};
use webpki::Error as WebpkiError;

// naming scheme: <ctr | cafe | nintendo> - <common | cacert> - <additional information> - <cert | key>
//
// common appears to be used for data that is common to all consoles, which is why it is a part of
// the naming scheme for constants here. there is no chance that i will upload console-unique
// information. do not ask

// client-side ceritificates

/// The client certificate used by the 3ds in authentication to numerous Nintendo HTTP services
pub const CTR_COMMON_1_CERT: &[u8] = include_bytes!("ctr-common-1.crt.der");

/// The corresponding private key for [`CTR_COMMON_1_CERT`]
///
/// [`CTR_COMMON_1_CERT`]: ./constant.CTR_COMMON_1_CERT.html
pub const CTR_COMMON_1_KEY: &[u8] = include_bytes!("ctr-common-1.key.der");

/// The client certificate used by the WiiU in authentication to numerous Nintendo HTTP services
pub const WUP_COMMON_1_CERT: &[u8] = include_bytes!("wup-common-1.crt.der");

/// The corresponding private key for [`WUP_COMMON_1_CERT`]
///
/// [`WUP_COMMON_1_CERT`]: ./constant.WUP_COMMON_1_CERT.html
pub const WUP_COMMON_1_KEY: &[u8] = include_bytes!("wup-common-1.key.der");

/// The client certificate used by the WiiU in authentication to the Nintendo Network account
/// service
pub const WUP_ACCOUNT_1_CERT: &[u8] = include_bytes!("wup-account-1.crt.der");

/// The corresponding private key for [`WUP_ACCOUNT_1_CERT`]
///
/// [`WUP_ACCOUNT_1_CERT`]: ./constant.WUP_ACCOUNT_1_CERT.html
pub const WUP_ACCOUNT_1_KEY: &[u8] = include_bytes!("wup-account-1.key.der");

// server-side certificates used for verification

pub const NINTENDO_CACERT_CA_CERT: &[u8] = include_bytes!("nintendo-cacert-ca.crt.der");

pub const NINTENDO_CACERT_CA_G2_CERT: &[u8] = include_bytes!("nintendo-cacert-ca-g2.crt.der");

pub const NINTENDO_CACERT_CA_G3_CERT: &[u8] = include_bytes!("nintendo-cacert-ca-g3.crt.der");

pub const NINTENDO_CACERT_CLASS2_CA_CERT: &[u8] =
    include_bytes!("nintendo-cacert-class2-ca.crt.der");

pub const NINTENDO_CACERT_CLASS2_CA_G2_CERT: &[u8] =
    include_bytes!("nintendo-cacert-class2-ca-g2.crt.der");

pub const NINTENDO_CACERT_CLASS2_CA_G3_CERT: &[u8] =
    include_bytes!("nintendo-cacert-class2-ca-g3.crt.der");

/// Generates a [`RootCertStore`] containing all Nintendo server certificates
///
/// [`RootCertStore`]: https://docs.rs/rustls/0.17.0/rustls/struct.RootCertStore.html
pub fn generate_cacert_bundle() -> Result<RootCertStore, WebpkiError> {
    let mut store = RootCertStore::empty();

    store.add(&Certificate(NINTENDO_CACERT_CA_CERT.to_vec()))?;
    store.add(&Certificate(NINTENDO_CACERT_CA_G2_CERT.to_vec()))?;
    store.add(&Certificate(NINTENDO_CACERT_CA_G3_CERT.to_vec()))?;
    store.add(&Certificate(NINTENDO_CACERT_CLASS2_CA_CERT.to_vec()))?;
    store.add(&Certificate(NINTENDO_CACERT_CLASS2_CA_G2_CERT.to_vec()))?;
    store.add(&Certificate(NINTENDO_CACERT_CLASS2_CA_G3_CERT.to_vec()))?;

    Ok(store)
}
