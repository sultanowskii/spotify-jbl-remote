use std::io::Read;

use spotify_jbl_remote::{
    input_event::{InputEvent, EventType, INPUT_EVENT_CHUNK_SIZE},
    errors::exit_with_error,
    device::{find_jbl_device_input_file, open_event_file},
};

fn main() {
    let device_handler_filename = match find_jbl_device_input_file() {
        Some(filename) => filename,
        None => exit_with_error("Unable to find a device input handler file."),
    };

    let mut file = match open_event_file(&device_handler_filename) {
        Ok(f) => f,
        Err(e) => exit_with_error(format!("Error occurred trying to open a device file: {}", e).as_str())
    };

    loop {
        let mut chunk = Vec::with_capacity(INPUT_EVENT_CHUNK_SIZE);

        match file.by_ref().take(INPUT_EVENT_CHUNK_SIZE as u64).read_to_end(&mut chunk) {
            Ok(n) => {
                if n != INPUT_EVENT_CHUNK_SIZE {
                    eprintln!("Error occured parsing input chunk: Invalid chunk size read");
                    continue;
                }
            }
            Err(e) => exit_with_error(format!("Looks like device is not available anymore (error while reading chunk: {})", e).as_str())
        };

        let input_event = match InputEvent::try_from(&chunk) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error occured parsing input chunk: {}", e);
                continue;
            }
        };

        // we want button to be unpressed + ignoring syn events
        if input_event.type_ == EventType::Syn || input_event.value != 0 {
            continue;
        }

        println!("ayo mr white");
    }
}
