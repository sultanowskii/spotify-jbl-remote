use std::io::Read;

use crate::{
    input_event::{
        InputEvent,
        INPUT_EVENT_CHUNK_SIZE
    },
    errors::exit_with_error,
    device::{
        find_jbl_device_input_file,
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

    let device_list = match get_input_device_list() {
        Ok(l) => l,
        Err(e) => {
            exit_with_error_or_return!(
                format!(
                    "Can't access /proc/bus/input/devices: {}",
                    e,
                ).as_str(),
                poll_mode
            );
        }
    };

    let device_handler_filename = match find_jbl_device_input_file(device_list) {
        Some(filename) => filename,
        None => {
            exit_with_error_or_return!(
                "Can't find a device input handler file. \
                Make sure your JBL speaker is connected.",
                poll_mode
            );
        },
    };

    let file = match open_event_file(&device_handler_filename) {
        Ok(f) => f,
        Err(e) => {
            exit_with_error_or_return!(
                format!(
                    "Can't access JBL input event file. \
                    Make sure you have permissions to read /dev/input/*. Error: {}",
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