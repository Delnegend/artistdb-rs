mod utils;

use std::rc::Rc;

use clap::{arg, command, Parser};
use inotify::{Inotify, WatchMask};
use std::path::PathBuf;
use tracing::error;
use utils::{process_artists::Artists, supported_socials::SupportedSocials};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    watch: bool,
    #[arg(short, long)]
    format: bool,

    #[arg(short, long, default_value = "./artists.txt")]
    in_file: String,
    #[arg(short, long, default_value = "./src/public/artists")]
    out_dir: String,
    #[arg(long, default_value = "500")]
    save_delay: u64,
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let args = Args::parse();

    if args.format {
        let new_contents =
            Artists::from_file(Rc::from(SupportedSocials::default()), &args.in_file).to_original();
        let old_content = std::fs::read_to_string(&args.in_file).unwrap();

        let bak_path = format!(
            "{}-{}.bak",
            &args.in_file,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );

        std::fs::write(bak_path, old_content).unwrap();
        std::fs::write(&args.in_file, new_contents).unwrap();

        return;
    }

    let pipeline = crate::utils::pipeline::Pipeline {
        in_file: &args.in_file,
        out_dir: &args.out_dir,
        supported_socials: Rc::from(SupportedSocials::default()),
    };
    pipeline.run();

    if !args.watch {
        return;
    }

    let mut inotify = Inotify::init().expect("Error while initializing inotify instance");

    // Watch for modify and close events.
    inotify
        .watches()
        .add(PathBuf::from(&args.in_file).as_path(), WatchMask::MODIFY)
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
