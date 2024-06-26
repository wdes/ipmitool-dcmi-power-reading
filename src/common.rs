use clap::{Args, Parser, ValueEnum};
use ipmi_rs::{
    connection::{
        rmcp::{Active, Rmcp},
        File,
    },
    Ipmi,
};
/**
 * @copyright datdenkikniet - Copyright (c) 2023 The `ipmi-rs` developers
 * @source https://github.com/datdenkikniet/ipmi-rs/blob/v0.2.1/examples/common.rs
 */
use std::{io::ErrorKind, time::Duration};

pub enum IpmiConnectionEnum {
    Rmcp(Ipmi<Rmcp<Active>>),
    File(Ipmi<File>),
}

#[derive(Parser)]
struct CliOpts {
    #[clap(flatten)]
    pub common: CommonOpts,
}

#[derive(ValueEnum, Clone, Copy)]
pub enum OutputFormats {
    Text,
    Json,
}

#[derive(Args)]
pub struct CommonOpts {
    /// The connection URI to use
    #[clap(default_value = "file:///dev/ipmi0", long, short)]
    connection_uri: String,
    /// How many milliseconds to wait before timing out while waiting for a response
    #[clap(default_value = "2000", long)]
    timeout_ms: u64,
    /// The format to output
    #[clap(default_value = "text", value_enum, long)]
    format: OutputFormats,
}

fn error<T>(val: T) -> std::io::Error
where
    T: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    std::io::Error::new(ErrorKind::Other, val)
}

impl CommonOpts {
    pub fn get_format(&self) -> OutputFormats {
        self.format
    }
    pub fn get_connection(&self) -> std::io::Result<IpmiConnectionEnum> {
        let timeout = Duration::from_millis(self.timeout_ms);

        if self.connection_uri.starts_with("file://") {
            let (_, path) = self.connection_uri.split_once("file://").unwrap();

            log::debug!("Opening file {path}");

            let file = File::new(path, timeout)?;
            let ipmi = Ipmi::new(file);
            Ok(IpmiConnectionEnum::File(ipmi))
        } else if self.connection_uri.starts_with("rmcp://") {
            let (_, data) = self.connection_uri.split_once("rmcp://").unwrap();

            let err =
                || error("Invalid connection URI. Format: `rmcp://[username]:[password]@[address]");

            let (username, rest) = data.split_once(':').ok_or(err())?;

            let (password, address) = rest.split_once('@').ok_or(err())?;

            log::debug!("Opening connection to {address}");

            let rmcp = Rmcp::new(address, timeout)?;
            let activated = rmcp
                .activate(Some(username), password.as_bytes())
                .map_err(|e| error(format!("RMCP activation error: {:?}", e)))?;

            let ipmi = Ipmi::new(activated);
            Ok(IpmiConnectionEnum::Rmcp(ipmi))
        } else {
            Err(error(format!(
                "Invalid connection URI {}",
                self.connection_uri
            )))
        }
    }
}
