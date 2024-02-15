use serde::Serialize;
use serde_with::skip_serializing_none;
use tracing::warn;

use crate::constants::{Constants, SupportedSocial};

#[derive(Debug, PartialEq, Serialize)]
#[skip_serializing_none]
pub struct ProcessedSocial {
    #[serde(skip_serializing)]
    pub match_social: Option<SupportedSocial>,
    #[serde(skip_serializing)]
    pub social_specific_uname: Option<String>,

    pub code: Option<String>,
    pub description: Option<String>,
    pub profile_url: Option<String>,
}

#[derive(Debug, PartialEq)]
struct ProcessedKey {
    match_social: Option<SupportedSocial>,
    description: Option<String>,
}

#[derive(Debug, PartialEq)]
struct ProcessedValue {
    profile_url: Option<String>,
    uname_match_social: Option<String>,
}

#[derive(Debug)]
pub struct SocialsProcessor<'a> {
    constants: &'a Constants<'a>,
    alias: &'a mut Vec<String>,
}

impl<'a> SocialsProcessor<'a> {
    pub fn from(constants: &'a Constants, alias: &'a mut Vec<String>) -> Self {
        Self { constants, alias }
    }

    pub fn get_alias(&self) -> Vec<String> {
        self.alias
            .clone()
            .into_iter()
            .filter(|item| !item.is_empty())
            .map(|item| item.to_lowercase())
            .collect::<Vec<String>>()
    }

    /// Split the social key into SupportedSocial and description
    /// Example
    /// ```toml
    /// [username]
    /// "TELegram:Ipsum" = "..."
    /// "telegram"       = "..."
    /// "hello:world"    = "..."
    /// ```
    ///
    /// - Found in social list, have description: "TELegram:Ipsum" -> Some(SupportedSocial<"telegram">), Some("Ipsum")
    /// - Have no description:                    "telegram"       -> Some(SupportedSocial<"telegram">), None
    /// - Not found in social list:               "hello:world"    -> None, Some("hello:world")
    /// - Nothing at all:                         ""               -> None, None
    fn key(&self, original: &String) -> ProcessedKey {
        let split = original.splitn(2, ':').collect::<Vec<&str>>();

        let first_part = match split.get(0) {
            Some(first) => *first,
            None => {
                return ProcessedKey {
                    match_social: None,
                    description: None,
                }
            }
        };

        let match_social = match self.constants.unavatar_socials.get(first_part) {
            Some(social) => Some(social.clone()),
            None => None,
        };

        let description = match match_social {
            None => Some(original.to_string()),
            Some(_) => match split.get(1) {
                Some(second) => Some(second.to_string()),
                None => None,
            },
        }
        .filter(|item| !item.is_empty());

        ProcessedKey {
            match_social,
            description,
        }
    }

    /// Construct profile URL from username and SupportedSocial,
    /// and return the platform-specific username if raw isn't an URL for the avatar-finder.
    ///
    /// Example:
    /// - Some(Telegram) + "Ipsum"              -> "https://t.me/Ipsum", Some("Ipsum")
    /// - Some(Telegram) + "https://t.me/Ipsum" -> "https://t.me/Ipsum", None + warning to replace with username
    /// - None + "https://t.me/Ipsum"           -> "https://t.me/Ipsum", None
    /// - None + "Ipsum"                        -> "Ipsum", Some("Ipsum") + warning to replace with profile url
    fn value(
        &mut self,
        match_socials: &Option<SupportedSocial>,
        original: &String,
        original_key: &String, // for logging purpose
        artist_name: &String,  // for logging purpose
    ) -> ProcessedValue {
        let original_lower = original.to_lowercase();
        let original_is_url =
            original_lower.starts_with("http://") || original_lower.starts_with("https://");

        if !original_is_url {
            self.alias.push(original.clone());
        }

        let uname_match_social = match original_is_url {
            true => None,
            false => Some(original.clone()).filter(|item| !item.is_empty()),
        };

        match match_socials {
            Some(social) => match original_is_url {
                true => {
                    warn!("{}: replace {} with username", artist_name, original);
                    ProcessedValue {
                        profile_url: Some(original.clone()),
                        uname_match_social,
                    }
                }
                false => ProcessedValue {
                    profile_url: match social.profile_url.clone() {
                        Some(profile_url) => Some(profile_url.replace("<USERNAME>", &original)),
                        None => {
                            warn!("{}: social {} has no profile url", artist_name, social.code);
                            None
                        }
                    },
                    uname_match_social,
                },
            },

            None => match original_is_url {
                true => {
                    warn!("{}: add support for social {}", artist_name, original_key);
                    ProcessedValue {
                        profile_url: Some(original.clone()),
                        uname_match_social,
                    }
                }
                false => {
                    warn!(
                        "{}: add support for social {}, or replace value with profile url",
                        artist_name, original_key
                    );
                    ProcessedValue {
                        profile_url: None,
                        uname_match_social,
                    }
                }
            },
        }
    }

    pub fn parse(&mut self, key_: &String, value_: &String, username: &String) -> ProcessedSocial {
        let key = self.key(key_);
        let value = self.value(&key.match_social, value_, key_, username);
        ProcessedSocial {
            match_social: key.match_social.clone(),
            social_specific_uname: value.uname_match_social,
            code: key.match_social.map(|x| x.code),

            description: key.description,
            profile_url: value.profile_url,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_social_processor_key() {
        let constants = Constants::new();
        let mut alias: Vec<String> = vec![];
        let username = "".to_string(); // only for logging purpose

        let mut social_processor = SocialsProcessor::from(&constants, &mut alias);

        let telegram = SupportedSocial {
            display: "Telegram".to_string(),
            code: "telegram".to_string(),
            profile_url: Some("https://t.me/<USERNAME>".to_string()),
        };

        // key: social + desc | value: username (best case)
        let a = social_processor.parse(&"telegram:Main".to_string(), &"foo".to_string(), &username);
        assert_eq!(a.match_social, Some(telegram.clone()));
        assert_eq!(a.description, Some("Main".to_string()));
        assert_eq!(a.profile_url, Some("https://t.me/foo".to_string()));
        assert_eq!(a.social_specific_uname, Some("foo".to_string()));

        // key: social + desc | value: url
        let b = social_processor.parse(
            &"telegram:foo".to_string(),
            &"https://t.me/foo".to_string(),
            &username,
        );
        assert_eq!(b.match_social, Some(telegram.clone()));
        assert_eq!(b.description, Some("foo".to_string()));
        assert_eq!(b.profile_url, Some("https://t.me/foo".to_string()));
        assert_eq!(b.social_specific_uname, None);

        // key: desc | value: username
        let c = social_processor.parse(&"foo:bar".to_string(), &"foo".to_string(), &username);
        assert_eq!(c.match_social, None);
        assert_eq!(c.description, Some("foo:bar".to_string()));
        assert_eq!(c.profile_url, None);
        assert_eq!(c.social_specific_uname, Some("foo".to_string()));

        // key: desc | value: url
        let d = social_processor.parse(
            &"lorem:ipsum".to_string(),
            &"http://example.com/bar".to_string(),
            &username,
        );
        assert_eq!(d.match_social, None);
        assert_eq!(d.description, Some("lorem:ipsum".to_string()));
        assert_eq!(d.profile_url, Some("http://example.com/bar".to_string()));
        assert_eq!(d.social_specific_uname, None);

        // key: social | value: username
        let e = social_processor.parse(&"telegram".to_string(), &"foo".to_string(), &username);
        assert_eq!(e.match_social, Some(telegram.clone()));
        assert_eq!(e.description, None);
        assert_eq!(e.profile_url, Some("https://t.me/foo".to_string()));
        assert_eq!(e.social_specific_uname, Some("foo".to_string()));

        // key: social | value: url
        let f = social_processor.parse(
            &"telegram".to_string(),
            &"https://t.me/foo".to_string(),
            &username,
        );
        assert_eq!(f.match_social, Some(telegram.clone()));
        assert_eq!(f.description, None);
        assert_eq!(f.profile_url, Some("https://t.me/foo".to_string()));
        assert_eq!(f.social_specific_uname, None);

        // nothing
        let d = social_processor.parse(&"".to_string(), &"".to_string(), &username);
        assert_eq!(d.match_social, None);
        assert_eq!(d.description, None);
        assert_eq!(d.profile_url, None);
        assert_eq!(d.social_specific_uname, None);
    }
}
