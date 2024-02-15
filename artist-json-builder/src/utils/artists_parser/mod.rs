mod info_parser;

use crate::utils::artists_parser::info_parser::InfoParser;
use toml_edit::{Document, Table};
use tracing::warn;

#[derive(Debug)]
pub struct ParsedArtist {
    pub username: String,
    /// Render this over `username` in the website if present
    pub name: Option<String>,
    pub flag: Option<String>,
    pub avatar: Option<String>,
    pub alias: Vec<String>,
    pub socials: Vec<(String, String)>,
}

fn parse_artist_tables(document: &Document) -> Vec<(&str, &Table)> {
    document
        .iter()
        .filter_map(|(username, _)| {
            document
                .get(username)
                .and_then(|item| item.as_table())
                .ok_or_else(|| {
                    warn!("{}: must be a table", username);
                })
                .ok()
                .map(|item| (username, item))
        })
        .collect::<Vec<_>>()
}

pub fn from(path: &str) -> Vec<ParsedArtist> {
    let document = std::fs::read_to_string(path)
        .expect("failed to read artists.toml")
        .parse::<Document>()
        .expect("failed to parse artists.toml");

    parse_artist_tables(&document)
        .into_iter()
        .map(|(username, info)| {
            let info_parser = InfoParser::from(username, info);
            ParsedArtist {
                username: username.to_string(),
                name: info_parser.name(),
                flag: info_parser.flag(),
                avatar: info_parser.avatar(),
                alias: info_parser.alias(),
                socials: info_parser.socials(),
            }
        })
        .collect::<Vec<_>>()
}
