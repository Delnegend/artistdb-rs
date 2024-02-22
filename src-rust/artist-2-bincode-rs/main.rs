mod utils;

use clap::{command, Parser};
use inotify::{Inotify, WatchMask};
use std::path::PathBuf;
use tracing::error;
use utils::{constants::Constants, pipeline::Pipeline};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    watch: bool,

    #[arg(short, long, default_value = "./artists.toml")]
    input_file: String,
    #[arg(short, long, default_value = "./src/public/artists")]
    output_dir: String,
    #[arg(long, default_value = "500")]
    toml_save_delay_ms: u64,
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let constants = Constants::default();
    let args = Args::parse();
    let mut pipeline = Pipeline::new(&constants, &args);

    pipeline.run();

    if !args.watch {
        return;
    }

    let mut inotify = Inotify::init().expect("Error while initializing inotify instance");

    // Watch for modify and close events.
    inotify
        .watches()
        .add(PathBuf::from(&args.input_file).as_path(), WatchMask::MODIFY)
        .expect("Failed to add file watch");

    // Read events that were added with `Watches::add` above.
    let mut buffer = [0; 1024];
    'scoped: loop {
        let events = match inotify.read_events_blocking(&mut buffer) {
            Ok(events) => events,
            Err(err) => {
                error!("Error while reading events: {}", err);
                continue 'scoped;
            }
        };

        for event in events {
            if event.mask != inotify::EventMask::MODIFY {
                continue 'scoped;
            }
        }

        pipeline.run();
    }
}
