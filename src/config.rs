/*
 * Copyright (c) 2019, Joyent, Inc.
 */


use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::net::Ipv4Addr;
use std::path::Path;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MantaDomain(pub String);

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Config {
    name: MantaDomain,
    trustedIP: Ipv4Addr,
    adminIPs: Option<Vec<Ipv4Addr>>,
    mantaIPs: Option<Vec<Ipv4Addr>>,
    untrustedIPs: Option<Vec<Ipv4Addr>>,
    zookeeper: ZookeeperConfig
}

#[derive(Serialize, Deserialize)]
pub struct ZookeeperConfig {
    servers: Vec<ZookeeperServer>,
    timeout: u64
}

#[derive(Serialize, Deserialize)]
pub struct ZookeeperServer {
    host: String,
    port: u32
}

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Config`.
    let c = serde_json::from_reader(reader)?;

    Ok(c)
}
