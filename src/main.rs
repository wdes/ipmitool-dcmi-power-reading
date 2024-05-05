use std::io::Error;

use binary_layout::binary_layout;
use chrono::{TimeZone, Utc};
use clap::Parser;
use common::{CommonOpts, IpmiConnectionEnum};
use ipmi_rs::connection::{IpmiConnection, LogicalUnit, Message, NetFn, Request};

mod common;

#[derive(Parser)]
pub struct Command {
    #[clap(flatten)]
    common: CommonOpts,
}
const IPMI_DCMI: u8 = 0xDC;
const IPMI_NETFN_DCGRP: u8 = 0x2C;
const IPMI_DCMI_GETRED: u8 = 0x02; // Get power reading - Enhanced Power Statistics
const IPMI_DCMI_MODE_POWER_STATUS: u8 = 0x01;
#[allow(dead_code)]
const IPMI_DCMI_MODE_ENHANCED_POWER_STATISTICS: u8 = 0x02;
const IPMI_DCMI_SAMPLE_TIME: u8 = 0x00; // TODO: make this configurable for IPMI_DCMI_MODE_ENHANCED_POWER_STATISTICS ?

fn get_message() -> std::io::Result<Message> {
    Ok(Message::new_request(
        NetFn::from(IPMI_NETFN_DCGRP),
        IPMI_DCMI_GETRED,
        vec![
            IPMI_DCMI,                   // Group Extension Identification
            IPMI_DCMI_MODE_POWER_STATUS, // Mode Power Status or Enhanced Power Statistics
            IPMI_DCMI_SAMPLE_TIME,       // Value if IPMI_DCMI_MODE_ENHANCED_POWER_STATISTICS
            0x00,                        // reserved
        ],
    ))
}

// See: https://github.com/ipmitool/ipmitool/blob/IPMITOOL_1_8_19/lib/ipmi_dcmi.c#L1398-L1454
fn ipmi_dcmi_pwr_rd(ipmi: IpmiConnectionEnum) -> Result<(), Error> {
    let message = get_message()?;

    let mut request: Request = Request::new(message, LogicalUnit::Zero);

    let result = match ipmi {
        common::IpmiConnectionEnum::Rmcp(mut r) => r.inner_mut().send_recv(&mut request)?,
        common::IpmiConnectionEnum::File(mut f) => f.inner_mut().send_recv(&mut request)?,
    };
    let response_data = result.data();

    log::debug!("Completion code: 0x{:02X}", result.cc());
    log::debug!("NetFN: 0x{:02X} ({:?})", result.netfn_raw(), result.netfn());
    log::debug!("Cmd: 0x{:02X}", result.cmd());
    log::debug!("Data: {:02X?}", response_data);

    // Example: [DC, D2, 00, 02, 00, D4, 01, B8, 00, 89, 72, 37, 66, E8, 03, 00, 00, 40]
    binary_layout!(power_data, LittleEndian, {
        grp_id: u8, /* first byte: Group Extension ID */
        curr_pwr: u16,
        min_sample: u16,
        max_sample: u16,
        avg_pwr: u16,
        time_stamp: u32, /* time since epoch */
        sample: u32,
        state: u8,
    });
    let view = power_data::View::new(response_data);
    //let grp_id: u8 = view.grp_id().read();
    let curr_pwr: u16 = view.curr_pwr().read();
    let min_sample: u16 = view.min_sample().read();
    let max_sample: u16 = view.max_sample().read();
    let avg_pwr: u16 = view.avg_pwr().read();
    let time_stamp: u32 = view.time_stamp().read();
    let date_time = Utc.timestamp_opt(time_stamp as i64, 0).unwrap();
    let sample: u32 = view.sample().read();
    let state: u8 = view.state().read();

    //println!("grp_id: {}:{:02X?}", grp_id, grp_id);
    println!("");
    println!(
        "    Instantaneous power reading              : {:<8} Watts",
        curr_pwr
    );
    println!(
        "    Minimum during sampling period           : {:<8} Watts",
        min_sample
    );
    println!(
        "    Maximum during sampling period           : {:<8} Watts",
        max_sample
    );

    println!(
        "    Average power reading over sample period : {:<8} Watts",
        avg_pwr
    );
    println!(
        "    IPMI timestamp                           : {}",
        date_time
    );
    println!(
        "    Sampling period                          : {} Milliseconds",
        sample
    );
    println!(
        "    Power reading state is                   : {}",
        match state {
            0x40 => "activated",
            _ => "deactivated",
        }
    );
    println!("");
    println!("");
    Ok(())
}

fn main() -> std::io::Result<()> {
    pretty_env_logger::formatted_builder()
        .parse_filters(&std::env::var("RUST_LOG").unwrap_or("info".to_string()))
        .init();

    let command = Command::parse();
    let ipmi = command.common.get_connection()?;
    ipmi_dcmi_pwr_rd(ipmi)
}
