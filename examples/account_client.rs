//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use base64;
use clap::clap_app;
use isocountry::CountryCode;
use isolanguage_1::LanguageCode;
use parking_lot::RwLock;
use std::{borrow::Cow, convert::TryFrom, sync::Arc};

use ralsei::{
    client::account::Client,
    model::{
        certificate::Certificate,
        console::{
            common::{
                ConsoleSerial, Environment as DeviceEnvironment, Region as DeviceRegion,
                Type as DeviceType,
            },
            n3ds::{Console3ds, Model as N3dsModel},
        },
        network::Nnid,
        title::{id::TitleId, version::TitleVersion},
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = clap_app!(account_client =>
        (version: "0.0.1")
        (author: "superwhiskers <whiskerdev@protonmail.com>")
        (about: "a simple nintendo network account server client implemented in rust using the ralsei library")
        (@setting SubcommandRequired)
        (@subcommand user =>
            (about: "checks if a user exists")
            (@arg NNID: +required "the nnid to check for")
        )
        (@subcommand certificate =>
            (about: "parse a b64-encoded certificate and display as much information about it as possible")
            (@arg CERTIFICATE: +required "the certificate to parse, in b64 format")
        )
        (@subcommand serial =>
            (about: "parse a serial number and display as much information about it as possible")
            (@arg SERIAL: +required "the serial to parse")
        )
    ).get_matches();

    let console = Arc::new(RwLock::new(Console3ds::new(|b| {
        b.device_type(DeviceType::Retail)
            .device_id(1) // dummy
            .serial(Cow::Borrowed("1")) // dummy
            .system_version(TitleVersion(0x02D0))
            .region(DeviceRegion::UnitedStates)
            .country(CountryCode::USA)
            .client_id(Cow::Borrowed("ea25c66c26b403376b4c5ed94ab9cdea"))
            .client_secret(Cow::Borrowed("d137be62cb6a2b831cad8c013b92fb55"))
            .fpd_version(0)
            .environment(DeviceEnvironment::L(1))
            .title_id_and_unique_id(TitleId(0x000400100002C000))
            .title_version(TitleVersion(0003))
            .language(LanguageCode::En)
            .api_version(1)
            .device_model(N3dsModel::Nintendo3ds)
    })));

    let client = Client::new(None, console.clone(), None, None)?;

    match app.subcommand() {
        ("user", Some(arguments)) => println!(
            "does the user exist: {}",
            client
                .does_user_exist(Nnid(Cow::Borrowed(
                    arguments
                        .value_of("NNID")
                        .expect("no nnid was provided (this should never happen)")
                )))
                .await?
        ),
        ("certificate", Some(arguments)) => {
            let cert = Certificate::try_from(
                base64::decode(
                    arguments
                        .value_of("CERTIFICATE")
                        .expect("no certificate was provided (this should never happen)"),
                )?
                .as_ref(),
            )?;
            println!("certificate: {:?}", cert);
            println!("device id: {:?}", cert.name.device_id());
            println!("console: {:?}", cert.name.console_kind());
            println!(
                "does issuer match a known one?: {:?}",
                cert.issuer.known_issuer()
            );
            println!("re-encoded: {}", base64::encode(cert.to_bytes()?));
        }
        ("serial", Some(arguments)) => println!(
            "valid: {}",
            ConsoleSerial(Cow::Borrowed(
                arguments
                    .value_of("SERIAL")
                    .expect("no serial was provided (this should never happen)")
            ))
            .check()
            .is_ok()
        ),
        _ => println!("you shouldn't have done that"),
    }
    Ok(())
}
