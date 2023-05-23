use clap::Parser;

use crate::spotify::event_handle_driver::DriverName;

#[derive(Parser)]
#[command(name = "spotify-jbl-remote")]
#[command(author = "Artur Sultanov, t.me/sultanowskii")]
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

    #[arg(
        short,
        long,
        value_name = "NO_UDEV",
        help = "To explore /dev/input/* to find JBL file. Use it only if you udev rule doesn't work for you",
    )]
    pub no_udev: bool,

    #[arg(
        short,
        long,
        value_name = "INPUT_DEVICE_FILENAME",
        help = "Specify JBL input device filename. Has the highest proprity",
    )]
    pub input_device_filename: Option<String>,
}