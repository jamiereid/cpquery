// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::Context;
use clap::{ArgAction, Parser, Subcommand};
use dirs;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;
use std::path::PathBuf;

pub(crate) mod commands;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Host {
    IPv4(std::net::Ipv4Addr),
    IPv6(std::net::Ipv6Addr),
    FQDN(String),
}

#[derive(Debug, Deserialize)]
struct CheckpointConfig {
    host: Host,
    port: Option<u16>,
    username: String,
    password: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Config {
    checkpoint: CheckpointConfig,
}

/// Query Checkpoint Firewall appliances via the REST API
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,

    /// Use a specific config file
    #[clap(short = 'C', long)]
    config_file: Option<PathBuf>,

    /// ignore ssl errors
    #[clap(short = 'k', long = "insecure", action=ArgAction::SetTrue)]
    accept_invalid_certs: bool,
}

#[derive(Debug, Subcommand)]
enum Command {
    TestConnection,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let file_to_load = if let Some(f) = args.config_file {
        f
    } else {
        if let Some(standard_conf_dir) = dirs::config_dir() {
            let mut f = standard_conf_dir;
            f.push("ncs.toml");
            f
        } else {
            anyhow::bail!("Unable to find config file location, try specifying manually.");
        }
    };

    let file_next_to_binary = if let Ok(exe_path) = std::env::current_exe() {
        let mut f = exe_path;
        f.push("ncs.toml");
        f
    } else {
        anyhow::bail!("Unable to determine current location of binary");
    };

    let config: Config = Figment::new()
        .merge(Toml::file(file_to_load))
        .merge(Toml::file(file_next_to_binary))
        .merge(Env::prefixed("NCS_"))
        .extract()
        .context("Unable to load configuration")?;

    let read_only = true;
    match args.command {
        Command::TestConnection => commands::test_connection::invoke(
            &config.checkpoint,
            args.accept_invalid_certs,
            read_only,
        )?,
    }

    Ok(())
}
