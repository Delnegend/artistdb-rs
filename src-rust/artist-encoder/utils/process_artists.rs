use std::{collections::HashSet, rc::Rc};
use tracing::warn;

use crate::utils::{process_socials::Social, supported_socials::SupportedSocials};

#[derive(Debug)]
pub struct Artist {
    pub username: String,
    pub display_name: Option<String>,
    pub avatar: Option<String>,
    pub alias: Vec<String>,
    pub socials: Vec<Social>,

    raw_social_lines: Vec<String>,
    formatted_info_line: String,
    pub(super) original_avatar: Option<String>,
    pub(super) supported_socials: Rc<SupportedSocials>,
}

impl Default for Artist {
    fn default() -> Artist {
        Artist {
            display_name: None,
            avatar: None,
            alias: vec![],
            socials: vec![],

            raw_social_lines: vec![],
            formatted_info_line: "".to_string(),
            original_avatar: None,
            username: "".to_string(),
            supported_socials: Rc::new(SupportedSocials::default()),
        }
    }
}

impl Artist {
    pub fn parse(supported_socials: Rc<SupportedSocials>, raw: &str) -> Result<Artist, String> {
        let mut new = Artist {
            supported_socials,
            ..Default::default()
        };

        let lines = raw.lines().collect::<Vec<&str>>();

        let first_line = lines.first().ok_or("empty line")?;

        new.parse_info(first_line)?;
        lines.iter().skip(1).for_each(|line| {
            new.raw_social_lines.push(line.to_string());
            let mut social = Social::new(Rc::clone(&new.supported_socials));
            match social.parse(line) {
                Ok(_) => new.socials.push(social),
                Err(err) => warn!("{}: failed to parse social: {}", &new.username, err),
            }
        });

        new.formatted_info_line = new.serialize_info_for_original()?;

        Ok(new)
    }

    pub fn serialize(&self) -> Result<String, String> {
        let mut lines: Vec<String> = vec![];
        lines.push(self.serialize_info()?);
        for social in &self.socials {
            lines.push(social.serialize()?);
        }
        Ok(lines.join("\n"))
    }
}

pub struct Artists(Vec<Artist>, Rc<SupportedSocials>);

impl Artists {
    pub fn from_file(supported_socials: Rc<SupportedSocials>, path: &str) -> Artists {
        let mut new = Artists(Vec::new(), supported_socials);

        let raw_data = match std::fs::read_to_string(path) {
            Ok(toml_string) => toml_string,
            Err(err) => {
                warn!("failed to read artists file: {}", err);
                return new;
            }
        };

        let raw_artists = raw_data
            .split("\n\n")
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();

        raw_artists.iter().for_each(|raw_artist| {
            let artist = match Artist::parse(new.1.clone(), raw_artist) {
                Ok(artist) => artist,
                Err(err) => {
                    warn!("failed to parse artist: {}", err);
                    return;
                }
            };
            new.0.push(artist);
        });

        new.lint_and_format();
        new
    }

    /// Warn duplicate users, remove alias duplicates
    pub fn lint_and_format(&mut self) {
        let mut all_username = HashSet::new();
        self.0.iter().for_each(|artist| {
            if all_username.contains(&artist.username) {
                warn!("duplicate username: {}", &artist.username);
            }
            all_username.insert(artist.username.clone());
        });

        let mut all_alias = self
            .0
            .iter()
            .flat_map(|artist| artist.alias.clone())
            .collect::<HashSet<String>>();

        self.0.iter_mut().for_each(|artist| {
            // Create a copy of the current artist's alias
            let current_artist_alias = artist.alias.clone();
            current_artist_alias.iter().for_each(|alias| {
                all_alias.remove(alias);
            });

            // Remove duplicates from the current
            artist
                .alias
                .retain(|alias| !all_alias.contains(alias) && !all_username.contains(alias));

            // Insert the current artist's alias back into the set of all alias
            current_artist_alias.into_iter().for_each(|alias| {
                all_alias.insert(alias);
            });
        });

        self.0.sort_by(|a, b| a.username.cmp(&b.username));
    }

    /// A Prettier for the original file
    pub fn to_original(&self) -> String {
        self.0
            .iter()
            .map(|artist| {
                let mut result = vec![artist.formatted_info_line.clone()];
                result.extend(artist.raw_social_lines.clone());
                result.join("\n")
            })
            .collect::<Vec<String>>()
            .join("\n\n")
    }

    pub fn get_artists(&self) -> &Vec<Artist> {
        &self.0
    }
}
