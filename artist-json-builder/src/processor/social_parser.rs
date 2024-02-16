use tracing::warn;

use crate::constants::{Constants, SupportedSocial};

use super::{is_url, processed_social::ProcessedSocial};

/// Internal use
#[derive(Debug, PartialEq)]
struct ProcessedKey {
    match_social: Option<SupportedSocial>,
    description: Option<String>,
}

/// Internal use
#[derive(Debug, PartialEq)]
struct ProcessedValue {
    profile_url: Option<String>,
    uname_match_social: Option<String>,
}

/// Use this to construct the Processor
#[derive(Debug)]
pub struct SocialParser<'a> {
    constants: &'a Constants,
    alias: &'a mut Vec<String>,
}

impl<'a> SocialParser<'a> {
    /// Construct a new SocialProcessor, given the Constants and a mutable
    /// reference to the alias list. New `value` will be added to the alias list
    /// if it's not an URL.
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

        let first_part = match split.first() {
            Some(first) => *first,
            None => {
                return ProcessedKey {
                    match_social: None,
                    description: None,
                }
            }
        };

        let match_social = self.constants.unavatar_socials.get(first_part).cloned();

        let description = match match_social {
            None => Some(original.to_string()),
            Some(_) => split.get(1).map(|second| second.to_string()),
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
        match_social: &Option<SupportedSocial>,
        original: &str,
        log_original_key: &String,
        log_username: &String,
    ) -> ProcessedValue {
        let original_lower = original.to_lowercase();
        let original_is_url = is_url(&Some(original_lower));

        if !original_is_url {
            self.alias.push(original.to_owned());
        }

        let uname_match_social = match original_is_url {
            true => None,
            false => Some(original.to_owned()).filter(|item| !item.is_empty()),
        };

        let profile_url = match_social
            .clone()
            // Pulling out the profile_url and code (for logging purpose)
            .and_then(|s| match s.url_template {
                Some(url_template) => Some(url_template),
                None => {
                    warn!(
                        "{}: social {} has no profile url",
                        log_username, log_original_key
                    );
                    None
                }
            })
            // Check if original string is empty
            .and_then(|url_template| match original.is_empty() {
                true => {
                    warn!(
                        "{}: social {} has no username",
                        log_username, log_original_key
                    );
                    None
                }
                false => Some(url_template),
            })
            // Replace <USERNAME> if not an URL
            .map(|url_tempalte| match original_is_url {
                true => {
                    warn!(
                        "{}: replace {} with username",
                        log_username, log_original_key
                    );
                    original.to_owned()
                }
                false => url_tempalte.replace("<USERNAME>", original),
            })
            // Last resort
            .or_else(|| match original_is_url {
                true => Some(original.to_owned()),
                false => {
                    warn!(
                        "{}: add support for {}, or use profile url",
                        log_username, log_original_key
                    );
                    None
                }
            });

        ProcessedValue {
            profile_url,
            uname_match_social,
        }
    }

    /// Given a raw key-value pair parsed from the toml file
    /// and the username for logging purporse, return a ProcessedSocial
    pub fn parse(&mut self, key_: &String, value_: &str, log_username: &String) -> ProcessedSocial {
        let key = self.key(key_);
        let value = self.value(&key.match_social, value_, key_, log_username);
        ProcessedSocial {
            match_social: key.match_social.clone(),
            specific_uname: value.uname_match_social,

            desc: self
                .constants
                .format_description(&key.match_social, &key.description),

            #[cfg(debug_assertions)]
            desc_raw: key.description,

            code: key.match_social.map(|s| s.code),
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

        let mut social_processor = SocialParser::from(&constants, &mut alias);

        let telegram = SupportedSocial {
            display: "Telegram".to_string(),
            code: "telegram".to_string(),
            url_template: Some("https://t.me/<USERNAME>".to_string()),
        };

        // key: social + desc | value: username (best case)
        let a = social_processor.parse(&"telegram:Main".to_string(), &"foo".to_string(), &username);
        assert_eq!(a.match_social, Some(telegram.clone()));
        assert_eq!(a.desc_raw, Some("Main".to_string()));
        assert_eq!(a.profile_url, Some("https://t.me/foo".to_string()));
        assert_eq!(a.specific_uname, Some("foo".to_string()));

        // key: social + desc | value: url
        let a = social_processor.parse(
            &"telegram:foo".to_string(),
            &"https://t.me/foo".to_string(),
            &username,
        );
        assert_eq!(a.match_social, Some(telegram.clone()));
        assert_eq!(a.desc_raw, Some("foo".to_string()));
        assert_eq!(a.profile_url, Some("https://t.me/foo".to_string()));
        assert_eq!(a.specific_uname, None);

        // key: social + desc | value: none
        let a = social_processor.parse(&"telegram:foo".to_string(), &"".to_string(), &username);
        assert_eq!(a.match_social, Some(telegram.clone()));
        assert_eq!(a.desc_raw, Some("foo".to_string()));
        assert_eq!(a.profile_url, None);
        assert_eq!(a.specific_uname, None);

        // key: desc | value: username
        let a = social_processor.parse(&"foo:bar".to_string(), &"foo".to_string(), &username);
        assert_eq!(a.match_social, None);
        assert_eq!(a.desc_raw, Some("foo:bar".to_string()));
        assert_eq!(a.profile_url, None);
        assert_eq!(a.specific_uname, Some("foo".to_string()));

        // key: desc | value: url
        let a = social_processor.parse(
            &"lorem:ipsum".to_string(),
            &"http://example.com/bar".to_string(),
            &username,
        );
        assert_eq!(a.match_social, None);
        assert_eq!(a.desc_raw, Some("lorem:ipsum".to_string()));
        assert_eq!(a.profile_url, Some("http://example.com/bar".to_string()));
        assert_eq!(a.specific_uname, None);

        // key: social | value: username
        let a = social_processor.parse(&"telegram".to_string(), &"foo".to_string(), &username);
        assert_eq!(a.match_social, Some(telegram.clone()));
        assert_eq!(a.desc_raw, None);
        assert_eq!(a.profile_url, Some("https://t.me/foo".to_string()));
        assert_eq!(a.specific_uname, Some("foo".to_string()));

        // key: social | value: url
        let a = social_processor.parse(
            &"telegram".to_string(),
            &"https://t.me/foo".to_string(),
            &username,
        );
        assert_eq!(a.match_social, Some(telegram.clone()));
        assert_eq!(a.desc_raw, None);
        assert_eq!(a.profile_url, Some("https://t.me/foo".to_string()));
        assert_eq!(a.specific_uname, None);

        // key: social | value: none
        let a = social_processor.parse(&"telegram".to_string(), &"".to_string(), &username);
        assert_eq!(a.match_social, Some(telegram.clone()));
        assert_eq!(a.desc_raw, None);
        assert_eq!(a.profile_url, None);
        assert_eq!(a.specific_uname, None);

        // key: none | value: username
        let a = social_processor.parse(&"".to_string(), &"foo".to_string(), &username);
        assert_eq!(a.match_social, None);
        assert_eq!(a.desc_raw, None);
        assert_eq!(a.profile_url, None);
        assert_eq!(a.specific_uname, Some("foo".to_string()));

        // key: none | value: url
        let a = social_processor.parse(&"".to_string(), &"https://t.me/foo".to_string(), &username);
        assert_eq!(a.match_social, None);
        assert_eq!(a.desc_raw, None);
        assert_eq!(a.profile_url, Some("https://t.me/foo".to_string()));
        assert_eq!(a.specific_uname, None);

        // nothing
        let a = social_processor.parse(&"".to_string(), &"".to_string(), &username);
        assert_eq!(a.match_social, None);
        assert_eq!(a.desc_raw, None);
        assert_eq!(a.profile_url, None);
        assert_eq!(a.specific_uname, None);
    }
}
