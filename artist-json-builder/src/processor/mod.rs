pub mod avatar_parser;
pub mod processed_social;
pub mod social_parser;

use serde::Serialize;

use crate::{constants::Constants, toml_parser::ParsedArtist};

use self::{
    avatar_parser::avatar_parser, processed_social::ProcessedSocial, social_parser::SocialParser,
};

#[derive(Debug, Serialize)]
pub struct ProcessedArtist {
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alias: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub socials: Vec<ProcessedSocial>,
}

fn process_artist(constants: &Constants, parsed_artist: ParsedArtist) -> ProcessedArtist {
    let mut alias = parsed_artist.alias;
    let mut social_processor = SocialParser::from(constants, &mut alias);

    let socials: Vec<ProcessedSocial> = parsed_artist
        .socials
        .into_iter()
        .map(|(key, value)| social_processor.parse(&key, &value, &parsed_artist.username))
        .collect();

    let avatar = avatar_parser(
        constants,
        &socials,
        parsed_artist.avatar,
        &parsed_artist.username,
    );
    ProcessedArtist {
        username: parsed_artist.username,
        name: parsed_artist.name,
        flag: parsed_artist.flag,
        avatar,

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

pub fn is_url(url: &Option<String>) -> bool {
    match url {
        Some(url) => {
            let url = url.to_lowercase();
            url.starts_with("http://") || url.starts_with("https://")
        }
        None => false,
    }
}
