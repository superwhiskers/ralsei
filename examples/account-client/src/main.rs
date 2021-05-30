//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use clap::clap_app;
use iso::language::Iso639_1;
use isocountry::CountryCode;
use parking_lot::RwLock;
use std::{borrow::Cow, convert::TryFrom, str::FromStr, sync::Arc};

use ralsei_model::{
    certificate::Certificate,
    console::{
        common::{ConsoleSerial, Environment as DeviceEnvironment},
        n3ds::Console3ds,
    },
    network::Nnid,
    title::{id::TitleId, version::TitleVersion},
};
use ralsei_service_account::{
    client::{AgreementVersionParameter, Client},
    xml::agreement::AgreementKindValue,
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
        (@subcommand titleid =>
            (about: "parse a title id and display as much information about it as possible")
            (@arg TITLE_ID: +required "the title id to parse, in hexadecimal format (without the `0x` prefix)")
        )
        (@subcommand eulas =>
            (about: "get the latest nintendo network eulas for the given country")
            (@arg COUNTRY: +required "the country to get the eula for")
        )
        (@subcommand timezones =>
            (about: "get the timezones from a specified country with names in the specified language")
            (@arg COUNTRY: +required "the country to get timezones from")
            (@arg LANGUAGE: +required "the language to have the names in")
        )
    ).get_matches();

    let console = Arc::new(RwLock::new(Console3ds::new(|b| {
        Ok(b.device_id(1) // dummy
            .serial(ConsoleSerial(Cow::Borrowed("CW404567772"))) // 3dbrew serial number (at https://www.3dbrew.org/wiki/Serials)
            .derive_region_from_serial()?
            .derive_device_model_from_serial()?
            .derive_device_type_from_serial()?
            .system_version(TitleVersion(0x02E0))
            .country(CountryCode::USA)
            .client_id(Cow::Borrowed("ea25c66c26b403376b4c5ed94ab9cdea"))
            .client_secret(Cow::Borrowed("d137be62cb6a2b831cad8c013b92fb55"))
            .fpd_version(0)
            .environment(DeviceEnvironment::L(1))
            .title_id(TitleId(0x000400100002C000))
            .derive_unique_id_from_title_id()?
            .title_version(TitleVersion(3))
            .language(Iso639_1::En))
    })?));

    let client = Client::new(None, console.clone(), None, None, None)?;

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
        ("serial", Some(arguments)) => {
            let serial = ConsoleSerial(Cow::Borrowed(
                arguments
                    .value_of("SERIAL")
                    .expect("no serial was provided (this should never happen)"),
            ));
            println!("valid: {:?}", serial.verify().is_ok());
            println!("region: {:?}", serial.region());
            println!("device model: {:?}", serial.device_model());
            println!("device type: {:?}", serial.device_type());
        }
        ("titleid", Some(arguments)) => {
            let title_id = TitleId(u64::from_str_radix(
                arguments
                    .value_of("TITLE_ID")
                    .expect("no title id was provided (this never should happen)"),
                16,
            )?);
            let unique_id = title_id.unique_id();
            println!("platform: {:?}", title_id.platform());
            println!("category: {:?}", title_id.category());
            println!(
                "part of the normal category: {:?}",
                title_id.category().map(|c| c.is_normal())
            );
            println!("unique id: {:?}", unique_id);
            println!("variation: {:?}", title_id.variation());
            println!("new3ds only: {}", unique_id.is_new3ds_only());
            println!("unique id group: {:?}", unique_id.group());
        }
        ("eulas", Some(arguments)) => println!(
            "eulas: {:?}",
            client
                .agreements(
                    AgreementKindValue::Eula,
                    CountryCode::for_alpha2_caseless(
                        arguments
                            .value_of("COUNTRY")
                            .expect("no country code was provided (this should never happen"),
                    )?,
                    AgreementVersionParameter::Latest
                )
                .await?
        ),
        ("timezones", Some(arguments)) => println!(
            "timezones: {:?}",
            client
                .timezones(
                    CountryCode::for_alpha2_caseless(
                        arguments
                            .value_of("COUNTRY")
                            .expect("no country code was provided (this should never happen"),
                    )?,
                    Iso639_1::from_str(
                        arguments
                            .value_of("LANGUAGE")
                            .expect("no language code was provided (this should never happen"),
                    )?,
                )
                .await?
        ),
        _ => println!("you shouldn't have done that"),
    }
    Ok(())
}
