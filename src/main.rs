use std::{fs::OpenOptions, io::Read, process};

use spotify_jbl_remote::input_event::{InputEvent, EventType};

const CHUNK_SIZE: usize = 24;

fn error(msg: &str) -> ! {
    eprintln!("{}", msg);
    eprintln!("Aborting.");
    process::exit(-1);
}

fn main() {
    let mut file_options = OpenOptions::new();
    file_options.read(true);
    file_options.write(false);

    let mut file = match file_options.open("/dev/input/event18") {
        Ok(f) => f,
        Err(e) => error(format!("Error ocured trying to open a device file: {}", e).as_str())
    };


    loop {
        let mut chunk = Vec::with_capacity(CHUNK_SIZE);

        match file.by_ref().take(CHUNK_SIZE as u64).read_to_end(&mut chunk) {
            Ok(n) => {
                if n != CHUNK_SIZE {
                    eprintln!("Error occured parsing input chunk: Invalid chunk size read");
                    continue;
                }
            }
            Err(e) => error(format!("Error ocured trying to read a chunk from device file: {}", e).as_str())
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
