use std::io::Read;

use spotify_jbl_remote::{
    input_event::{InputEvent, INPUT_EVENT_CHUNK_SIZE},
    errors::exit_with_error,
    device::{
        find_jbl_device_input_file,
        get_input_device_list,
        open_event_file,
    },
};

fn main() {
    let spotify_dbus = match spotify_jbl_remote::spotify::SpotifyDBus::new() {
        Ok(d) => d,
        Err(_) => {
            exit_with_error(
                "Can't connect to Spotify DBus. \
                Please make sure Spotify and I are executed by the same user.",
            );
        }
    };
    
    let device_list = match get_input_device_list() {
        Ok(l) => l,
        Err(e) => {
            exit_with_error(
                format!(
                    "Can't access /proc/bus/input/devices: {}",
                    e,
                ).as_str()
            );
        }
    };

    let device_handler_filename = match find_jbl_device_input_file(device_list) {
        Some(filename) => filename,
        None => {
            exit_with_error(
                "Can't find a device input handler file. \
                Make sure your JBL speaker is connected.",
            );
        },
    };

    let file = match open_event_file(&device_handler_filename) {
        Ok(f) => f,
        Err(e) => {
            exit_with_error(
                format!(
                    "Can't access JBL input event file. \
                    Make sure you have permissions to read /dev/input/*. Error: {}",
                    e,
                ).as_str()
            );
        },
    };

    let mut chunk = Vec::with_capacity(INPUT_EVENT_CHUNK_SIZE);

    loop {
        chunk.clear();

        match (&file).take(INPUT_EVENT_CHUNK_SIZE as u64).read_to_end(&mut chunk) {
            Ok(n) => {
                if n != INPUT_EVENT_CHUNK_SIZE {
                    eprintln!("Error occured parsing input chunk: Invalid chunk size read");
                    continue;
                }
            }
            Err(e) => {
                exit_with_error(
                    format!(
                        "Looks like device is not available anymore. Error: {}",
                        e,
                    )
                    .as_str()
                );
            }
        };

        let input_event = match InputEvent::try_from(&chunk) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error occured parsing input chunk: {}", e);
                continue;
            }
        };

        match spotify_dbus.handle_input_event(input_event) {
            Ok(_) => {},
            Err(e) => eprintln!("Error occured during communication with Spotify DBus: {}", e),
        };
    }
}
