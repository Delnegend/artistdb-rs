mod constants;
mod processor;
mod toml_parser;

use std::fs::write;

use clap::{command, Parser};
use constants::Constants;
use inotify::{EventMask, Inotify, WatchMask};
use murmur3::murmur3_x64_128 as hash;
use tracing::{debug, error, info, warn};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    watch: bool,
    #[arg(short, long)]
    input_file: String,
    #[arg(short, long)]
    output_dir: Option<String>,
}

fn _watcher() {
    // let mut buffer = [0u8; 4096];
    // 'main: loop { // no need to panic here, inotify blocks until there is an event
    //     let events = inotify.read_events_blocking(&mut buffer).unwrap();
    //     for event in events {
    //         if !event.mask.contains(EventMask::MODIFY) {
    //             continue 'main;
    //         }
    //         println!("artists.toml has been modified");
    //     }
    // }
}

fn main() {
    let constants = Constants::new();
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let mut parsed_artists = toml_parser::from(&args.input_file);
    parsed_artists.sort_by(|a, b| a.username.cmp(&b.username));

    let processed_artists = processor::from(&constants, parsed_artists);

    processed_artists.into_iter().for_each(|artist| {
        let json = serde_json::to_string_pretty(&artist).unwrap();

        let filename = format!(
            "{}/{}.json",
            args.output_dir.as_deref().unwrap_or("./output"),
            artist.username.to_lowercase()
        );

        write(&filename, json).unwrap_or_else(|err| error!("{}: {}", filename, err));
    });

    // watch file ./artists.toml changes
    // let mut inotify = Inotify::init().unwrap();
    // inotify.watches().add(artist_toml_path, WatchMask::MODIFY).unwrap();
}
