/*
 * Copyright (c) 2019, Joyent, Inc.
 */

use clap::{App, Arg, ArgMatches, crate_version};

static ABOUT: &'static str = "Muppet is an HTTP loadbalancer (haproxy) and \
                              small daemon that interacts with ZooKeeper via \
                              registrar. The muppet daemon will update the \
                              loadbalancer with new configuration as hosts \
                              come and go from the given service name.";

pub fn parse<'a, 'b>(app: String) -> ArgMatches<'a> {
    App::new(app)
        .about(ABOUT)
        .version(crate_version!())
        .arg(Arg::with_name("file")
             .help("Configuration file")
             .short("f")
             .long("file")
             .takes_value(true)
             .required(false))
        .arg(Arg::with_name("verbose")
             .help("Verbose output. Use multiple times for more verbose.")
             .short("v")
             .long("verbose")
             .multiple(true)
             .takes_value(false)
             .required(false))
        .get_matches()
}
