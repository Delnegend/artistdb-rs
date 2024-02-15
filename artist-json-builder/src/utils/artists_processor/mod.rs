mod avatar_finder;
mod social_process;

use serde::Serialize;

use crate::{constants::Constants, utils::artists_parser::ParsedArtist};

use self::social_process::{ProcessedSocial, SocialsProcessor};

#[derive(Debug, Serialize)]
pub struct ProcessedArtist {
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag: Option<String>,
    pub avatar: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alias: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub socials: Vec<ProcessedSocial>,
}

fn process_artist(constants: &Constants, parsed_artist: ParsedArtist) -> ProcessedArtist {
    let mut alias = parsed_artist.alias;
    let mut social_processor = SocialsProcessor::from(constants, &mut alias);

    let socials: Vec<ProcessedSocial> = parsed_artist
        .socials
        .into_iter()
        .map(|(key, value)| social_processor.parse(&key, &value, &parsed_artist.username))
        .collect();

    ProcessedArtist {
        username: parsed_artist.username,
        name: parsed_artist.name,
        flag: parsed_artist.flag,
        avatar: "".to_string(),
        alias: social_processor.get_alias(),
        socials,
    }
}

pub fn from(constants: &Constants, parsed_artists: Vec<ParsedArtist>) -> Vec<ProcessedArtist> {
    parsed_artists
        .into_iter()
        .map(|artist| process_artist(constants, artist))
        .collect()
}
