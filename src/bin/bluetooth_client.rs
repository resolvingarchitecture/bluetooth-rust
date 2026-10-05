extern crate log;
extern crate simple_logger;

use log::{trace};
use bluetooth_client::BluetoothClient;
use clap::{crate_version, App, AppSettings, Arg};

fn main() {
    simple_logger::init().unwrap();
    let m = App::new("bluetooth_client")
        .about("A SAM I2P client for the local I2P router instance. Not compliant with any version yet.")
        .version(crate_version!())
        .author("Brian Taylor <brian@resolvingarchitecture.io>")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ColorAlways)
        .arg(
            Arg::with_name("max_connection_attempts")
                .help("Maximum attempts to make a connection before failure is accepted. Each failed attempt results in waiting 3 seconds prior to making another attempt.")
                .short("c")
                .long("max_connection_attempts")
                .takes_value(true)
        )
        .subcommand(
            App::new("name")
                .about("get friendly name")
        )
        .subcommand(
            App::new("address")
                .about("get address")
        )
        .get_matches();
}