// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::types::CpLoginResponse;
use crate::{CheckpointConfig, Host};
use anyhow::{anyhow, Context};
use cp_api::Client;
use rpassword;

pub(crate) fn invoke(
    config: &CheckpointConfig,
    accept_invalid_certs: bool,
    want_read_only: bool,
) -> anyhow::Result<CpLoginResponse> {
    let host = match &config.host {
        Host::IPv4(x) => x.to_string(),
        Host::IPv6(x) => x.to_string(),
        Host::FQDN(x) => x.clone(),
    };

    let port = config.port.unwrap_or(443);

    let password: String = if let Some(x) = &config.password {
        x.clone()
    } else {
        rpassword::prompt_password(format!(
            "Please input password for user {}: ",
            &config.username
        ))?
    };

    let mut client = Client::new(&host, port);
    client.accept_invalid_certs(accept_invalid_certs);
    client.read_only(want_read_only);
    let login = client
        .login(&config.username, &password)
        .context("Error when attempting to login")?;

    if login.is_success() {
        println!("Login successful...");

        let res: CpLoginResponse = serde_json::from_value::<CpLoginResponse>(login.data).unwrap();

        client.logout().context("Error when attempting to logout")?;
        println!("Logout successful.");

        return Ok(res);
    } else if login.is_client_error() {
        return Err(anyhow!(
            "client error {} {}",
            login.data["code"],
            login.data["message"]
        ));
    } else if login.is_server_error() {
        return Err(anyhow!(
            "server error {} {}",
            login.data["code"],
            login.data["message"]
        ));
    };

    unreachable!();
}
