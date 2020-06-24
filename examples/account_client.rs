//
// ralsei - fast nintendo library in rust
//
// copyright (c) 2020 superwhiskers <whiskerdev@protonmail.com>
// this source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at http://mozilla.org/MPL/2.0/.
//

use clap::clap_app;
use isocountry::CountryCode;
use isolanguage_1::LanguageCode;
use parking_lot::RwLock;
use std::{borrow::Cow, sync::Arc};

use ralsei::{
    client::account::Client,
    model::{
        console::{
            common::{
                Environment as DeviceEnvironment, Region as DeviceRegion, Type as DeviceType,
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
    ).get_matches();

    let console = Arc::new(RwLock::new(Console3ds {
        device_type: Some(DeviceType::Retail),
        device_id: Some(1),               // dummy
        serial: Some(Cow::Borrowed("1")), // dummy
        system_version: Some(Cow::Borrowed("02D0")),
        region: Some(DeviceRegion::UnitedStates),
        country: Some(CountryCode::USA),
        client_id: Some(Cow::Borrowed("ea25c66c26b403376b4c5ed94ab9cdea")),
        client_secret: Some(Cow::Borrowed("d137be62cb6a2b831cad8c013b92fb55")),
        fpd_version: Some(0),
        environment: Some(DeviceEnvironment::L(1)),
        title_id: Some(TitleId(0x000400100002C000)),
        title_version: Some(TitleVersion(0003)),
        device_certificate: None, // dummy
        language: Some(LanguageCode::En),
        api_version: Some(1),
        device_model: Some(N3dsModel::Nintendo3ds),
    }));

    let client = Client::new(None, console.clone(), None, None)?;

    match app.subcommand() {
        ("user", Some(arguments)) => println!(
            "does the user exist: {}",
            client
                .does_user_exist(Nnid(Cow::Borrowed(
                    arguments.value_of("NNID").unwrap_or("placeholder")
                )))
                .await?
        ),
        _ => println!("you shouldn't have done that"),
    }
    Ok(())
}
