use clap::Parser;

use crate::spotify::event_handle_driver::DriverName;

#[derive(Parser)]
#[command(name = "spotify-jbl-remote")]
#[command(author = "Artur S. t.me/sultanowskii")]
#[command(version = "0.1")]
pub struct Args {
    #[arg(
        short,
        long,
        value_name = "INPUT_EVENT_DRIVER",
        default_value_t = DriverName::Default,
        value_hint = clap::ValueHint::CommandString,
        value_enum,
        help = "Input event driver"
    )]
    pub driver: DriverName,

    #[arg(
        short,
        long,
        value_name = "POLL_MODE",
        help = "To run forever, waiting for external device to connect",
    )]
    pub poll: bool,
}