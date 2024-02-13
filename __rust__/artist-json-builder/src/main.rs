use inotify::{EventMask, Inotify, WatchMask};
use murmur3::murmur3_x64_128 as hash;
use serde::Serialize;
use serde_json::Result;
use serde_with::skip_serializing_none;
use toml_edit::Document;
use tracing::{debug, error, info, warn};

mod parser;

#[skip_serializing_none]
#[derive(Serialize)]
struct DecodedArtist {
    name: String,
    flag: Option<String>,
    avatar: Option<String>,
    alias: Option<String>,
    socials: Option<Vec<(String, String)>>,
}

#[derive(Default)]
struct Parser {
    doc: Document,
}

impl Parser {
    fn parse_string(&self, key: &str) -> Option<String> {
        self.doc
            .get(key)
            .and_then(|value| value.as_str().map(|s| s.to_string()))
    }

    fn from(path: &str) -> Self {
        let mut new = Self::default();

        let toml = match std::fs::read_to_string(path) {
            Ok(toml) => toml,
            Err(e) => {
                error!("failed to read: {}", e);
                return new;
            }
        };

        new.doc = match toml.parse::<Document>() {
            Ok(doc) => doc,
            Err(e) => {
                error!("failed to parse: {}", e);
                return new;
            }
        };

        // 0: artist name
        // 1: table
        let artists = new
            .doc
            .iter()
            .map(|(k, _)| {
                let item = match new.doc.get(k) {
                    Some(item) => item,
                    None => {
                        error!("{} not found, this should not happen", k);
                        return None;
                    }
                };
                match item.as_table() {
                    Some(table) => Some((k, table)),
                    None => {
                        warn!("{} is not a table", k);
                        None
                    }
                }
            })
            .filter_map(|x| x)
            .collect::<Vec<_>>();

        artists.into_iter().for_each(|(name, info)| {
            // let avatar = info
            //     .get("avatar")
            //     .map(|name| match name.as_str() {
            //         Some(content) => content.to_string(),
            //         None => {
            //             warn!("avatar of {} is not a string, defaulting to empty", name);
            //             String::new()
            //         }
            //     });
            let avatar = match info.get("avatar").unwrap_or(None) {
                Some(content) => Some(content),
                None => {
                    warn!("avatar of {} is not a string", name);
                    None
                }
            };

            let flag = info
                .get("flag")
                .map(|flag| flag.as_str().unwrap().to_string());

            if let Some(alias) = info.get("alias") {
                let parsed = alias
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|a| a.as_str().unwrap())
                    .collect::<Vec<_>>();
                debug!("alias: {:?}", parsed);
            }

            let artist = DecodedArtist {
                name: name.to_string(),
                flag,
                avatar,
                alias: None,
                socials: None,
            };

            println!("{:?}", serde_json::to_string(&artist).unwrap());
            ()
        });

        new
    }
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
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let artist_toml_path = "./artists.toml";
    let artists = Parser::from(&artist_toml_path);

    // watch file ./artists.toml changes
    // let mut inotify = Inotify::init().unwrap();
    // inotify.watches().add(artist_toml_path, WatchMask::MODIFY).unwrap();
}
