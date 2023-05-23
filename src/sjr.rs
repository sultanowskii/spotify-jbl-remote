use std::io::Read;

use crate::{
    input_event::{
        InputEvent,
        INPUT_EVENT_CHUNK_SIZE
    },
    errors::exit_with_error,
    device::{
        find_jbl_input_device_file,
        get_input_device_list,
        open_event_file,
    },
    spotify::{
        SpotifyDBus,
        EventHandler,
    },
    args::Args,
};

macro_rules! exit_with_error_or_return {
    ($message:expr, $poll:expr) => {
        if $poll {
            return;
        } else {
            exit_with_error($message);
        }
    };
}

// Get input device filename based on CLI arguments.
fn get_input_device_filename(args: &Args) -> Result<String, String> {
    // If filename is specified explicitly, use it
    if args.input_device_filename.is_some() {
        return Ok(args.input_device_filename.to_owned().unwrap());
    }

    // If udev is used, then simply use symlink.
    if !args.no_udev {
        return Ok(String::from("/dev/spotify-jbl"));
    }

    // If udev is not used, then find event filename
    let device_list = match get_input_device_list() {
        Ok(l) => l,
        Err(e) => {
            return Err(
                format!(
                    "Can't access /proc/bus/input/devices: {}",
                    e,
                )
            );
        }
    };

    return match find_jbl_input_device_file(device_list) {
        Some(filename) => Ok(filename),
        None => {
            return Err(
                "Can't find input device file. \
                Make sure your JBL speaker is connected.".to_string(),
            );
        },
    };
}

pub fn event_loop(args: &Args) {
    let poll_mode = args.poll;
    
    let spotify_dbus = match SpotifyDBus::new() {
        Ok(d) => d,
        Err(_) => {
            exit_with_error_or_return!(
                "Can't connect to Spotify DBus. \
                Please make sure Spotify and I are executed by the same user.",
                poll_mode
            );
        }
    };
    let event_handler = EventHandler { dbus: spotify_dbus };
    
    let event_handle_driver = args.driver.try_into_driver();

    let input_device_filename = match get_input_device_filename(args) {
        Ok(f) => f,
        Err(msg) => {
            exit_with_error_or_return!(
                msg,
                poll_mode
            );
        }
    };

    let file = match open_event_file(&input_device_filename) {
        Ok(f) => f,
        Err(e) => {
            exit_with_error_or_return!(
                format!(
                    "Can't access JBL input event file ({}). \
                    Make sure it exists and you have permissions to read it. Error: {}",
                    &input_device_filename,
                    e,
                ).as_str(),
                poll_mode
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
                exit_with_error_or_return!(
                    format!(
                        "Looks like device is not available anymore. Error: {}",
                        e,
                    )
                    .as_str(),
                    poll_mode
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

        match event_handler.handle_input_event(event_handle_driver.as_ref(), &input_event) {
            Ok(_) => {},
            Err(e) => eprintln!("Error occured during communication with Spotify DBus: {}", e),
        };
    }
}