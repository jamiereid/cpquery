use crate::{CheckpointConfig, Host};
use anyhow::{anyhow, Context};
use cp_api::Client;
use std::io;

pub(crate) fn invoke(
    config: &CheckpointConfig,
    accept_invalid_certs: bool,
    want_read_only: bool,
) -> anyhow::Result<()> {
    let host = match &config.host {
        Host::IPv4(x) => x.to_string(),
        Host::IPv6(x) => x.to_string(),
        Host::FQDN(x) => x.clone(),
    };

    let port = config.port.unwrap_or(443);

    let password = if let Some(x) = &config.password {
        x.clone()
    } else {
        println!("Please input password for user {}:", &config.username);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read in password from stdin");
        input
    };

    let mut client = Client::new(&host, port);
    client.accept_invalid_certs(accept_invalid_certs);
    client.read_only(want_read_only);
    let login = client
        .login(&config.username, &password)
        .context("Error when attempting to login")?;

    if login.is_success() {
        println!("Login successful...");
        println!("UID: {}", client.uid());
        println!("API Server Version: {}", client.api_server_version());
        println!("SID: {}", client.sid());

        client.logout().context("Error when attempting to logout")?;
        println!("Logout successful.");

        return Ok(());
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
    }

    Ok(())
}
