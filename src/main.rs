use clap::Parser;

use spotify_jbl_remote::{
    args::Args,
    sjr::event_loop
};

fn main() {
    let args = Args::parse();
    loop {
        event_loop(&args);
        if !args.poll {
            break;
        }
    }
}
