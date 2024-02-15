mod constants;
mod utils;

use constants::Constants;
use inotify::{EventMask, Inotify, WatchMask};
use murmur3::murmur3_x64_128 as hash;
use tracing::{debug, error, info, warn};

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

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let parsed_artists = utils::artists_parser::from("./artists.toml");
    let processed_artists = utils::artists_processor::from(&constants, parsed_artists);

    processed_artists.into_iter().for_each(|artist| {
        // println!("{:?}\n", artist);
        let json = serde_json::to_string_pretty(&artist).unwrap();
        // write to ./output/
        let username = artist.username.to_lowercase();
        let filename = format!("./output/{}.json", username);
        std::fs::write(&filename, json).unwrap_or_else(|err| error!("{}: {}", filename, err));
    });

    // parsed_artists.into_iter().for_each(|artist| {
    //     println!("{:?}", artist);
    // });

    // watch file ./artists.toml changes
    // let mut inotify = Inotify::init().unwrap();
    // inotify.watches().add(artist_toml_path, WatchMask::MODIFY).unwrap();
}
