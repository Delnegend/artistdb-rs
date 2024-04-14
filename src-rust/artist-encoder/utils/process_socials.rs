use std::rc::Rc;

use crate::utils::{split_components, supported_socials::SupportedSocials};

#[derive(Debug, Default)]
pub struct Social {
    social_code: Option<String>,
    social_username: Option<String>,
    profile_url: Option<String>,
    description: Option<String>,
    is_special: bool,

    supported_socials: Rc<SupportedSocials>,
}

impl Social {
    pub fn new(supported_socials: Rc<SupportedSocials>) -> Self {
        Social {
            supported_socials,
            ..Default::default()
        }
    }

    /// Parse <*? username@social_code || //link>[,<description>] to Social
    pub fn parse(&mut self, raw: &str) -> Result<(), String> {
        let supported_socials = self.supported_socials.as_ref();

        let components = split_components(raw)?;

        self.description = components.get(1).map(|s| s.to_string());

        let mut first = match components.first() {
            Some(s) => s.to_string(),
            None => return Err("invalid social format".to_string()),
        };

        if first.starts_with('*') {
            first = first.trim_start_matches('*').to_string();
            self.is_special = true;
        } else {
            self.is_special = false;
        }

        if first.starts_with("//") {
            self.profile_url = Some(first.to_string());
            return Ok(());
        }

        // Using username@social_code
        let mut parts = first.splitn(2, '@');
        if let Some(social_username) = parts.next() {
            self.social_username = Some(social_username.to_string());
        }
        if let Some(social_code) = parts.next() {
            let social_code = social_code.to_ascii_lowercase();
            if supported_socials.is_supported(&social_code) {
                self.social_code = Some(social_code);
                return Ok(());
            }
            return Err("unsupported social code".to_string());
        }

        Err("missing social code".to_string())
    }

    /// Same as parse(), but return the Social instead of mutating it.
    /// Use for tests only
    #[cfg(test)]
    pub fn parse_into(self, raw: &str) -> Result<Self, String> {
        let mut social = self;
        social.parse(raw)?;
        Ok(social)
    }

    /// Serialize the Social to a string [*]<social link><social description>
    pub fn serialize(&self) -> Result<String, String> {
        let supported_socials = self.supported_socials.as_ref();

        let (profile_url, description) =
            match (&self.profile_url, &self.social_username, &self.social_code) {
                (Some(profile_url), _, _) if profile_url.starts_with("//") => Ok((
                    profile_url.to_string(),
                    self.description
                        .clone()
                        .filter(|s| !s.is_empty())
                        .ok_or_else(|| "missing description".to_string())?,
                )),
                (_, Some(social_username), Some(social_code)) => {
                    supported_socials.get(social_username, social_code, &self.description)
                }
                _ => Err("missing social code or name".to_string()),
            }?;
        if self.is_special || supported_socials.is_special(&self.social_code) {
            return Ok(format!("*{},{}", profile_url, description));
        }
        Ok(format!("{},{}", profile_url, description))
    }

    pub fn get_code(&self) -> Option<String> {
        self.social_code.clone()
    }
    pub fn get_name(&self) -> Option<String> {
        self.social_username.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_normal_cases() {
        let supported_socials = Rc::from(SupportedSocials::default());

        let mut social = Social::new(supported_socials.clone());
        social.parse("username@fb").unwrap();
        assert_eq!(social.social_username, Some("username".to_string()));
        assert_eq!(social.social_code, Some("fb".to_string()));
        assert_eq!(social.is_special, false);

        let mut social = Social::new(supported_socials.clone());
        social.parse("//link").unwrap();
        assert_eq!(social.profile_url, Some("//link".to_string()));
        assert_eq!(social.social_username, None);
        assert_eq!(social.is_special, false);

        let mut social = Social::new(supported_socials.clone());
        social.parse("//link,description").unwrap();
        assert_eq!(social.profile_url, Some("//link".to_string()));
        assert_eq!(social.description, Some("description".to_string()));
        assert_eq!(social.social_username, None);
        assert_eq!(social.is_special, false);
    }

    #[test]
    fn parse_valid_special_cases() {
        let supported_socials = Rc::from(SupportedSocials::default());

        let mut social = Social::new(supported_socials.clone());
        social.parse("*username@linktr.ee").unwrap();
        assert_eq!(social.social_username, Some("username".to_string()));
        assert_eq!(social.social_code, Some("linktr.ee".to_string()));
        assert_eq!(social.profile_url, None);
        assert_eq!(social.is_special, true);

        let mut social = Social::new(supported_socials.clone());
        social.parse("*//link").unwrap();
        assert_eq!(social.profile_url, Some("//link".to_string()));
        assert_eq!(social.social_username, None);
        assert_eq!(social.is_special, true);

        let mut social = Social::new(supported_socials.clone());
        social.parse("*//link,description").unwrap();
        assert_eq!(social.profile_url, Some("//link".to_string()));
        assert_eq!(social.description, Some("description".to_string()));
        assert_eq!(social.social_username, None);
        assert_eq!(social.is_special, true);
    }

    #[test]
    fn parse_invalid_cases() {
        let mut social = Social::new(Rc::from(SupportedSocials::default()));

        assert!(social.parse("username").is_err());
        assert!(social.parse("username@invalid_code").is_err());
        assert!(social.parse("@invalid_code").is_err());
    }

    #[test]
    fn serialize_valid() {
        let supported = Rc::new(SupportedSocials::default());

        assert_eq!(
            Social {
                social_username: Some("username".to_string()),
                social_code: Some("fb".to_string()),
                supported_socials: supported.clone(),
                ..Default::default()
            }
            .serialize()
            .unwrap(),
            "//fb.com/username,Facebook"
        );
        assert_eq!(
            Social {
                social_username: Some("username".to_string()),
                social_code: Some("x".to_string()),
                supported_socials: supported.clone(),
                ..Default::default()
            }
            .serialize()
            .unwrap(),
            "//twitter.com/username,𝕏"
        );
        assert_eq!(
            Social {
                social_username: Some("username".to_string()),
                social_code: Some("instagram".to_string()),
                description: Some("hi".to_string()),
                supported_socials: supported.clone(),
                ..Default::default()
            }
            .serialize()
            .unwrap(),
            "//instagram.com/username,Instagram | hi"
        );

        assert_eq!(
            Social {
                profile_url: Some("//link".to_string()),
                description: Some("hi".to_string()),
                supported_socials: supported.clone(),
                ..Default::default()
            }
            .serialize()
            .unwrap(),
            "//link,hi"
        );
    }

    #[test]
    fn serialize_invalid() {
        let supported = Rc::new(SupportedSocials::default());

        assert!(Social::default().serialize().is_err());
        assert!(Social {
            social_username: Some("username".to_string()),
            supported_socials: supported.clone(),
            ..Default::default()
        }
        .serialize()
        .is_err());

        assert!(Social {
            social_code: Some("x".to_string()),
            supported_socials: supported.clone(),
            ..Default::default()
        }
        .serialize()
        .is_err());

        assert!(Social {
            profile_url: Some("//link".to_string()),
            description: Some("".to_string()),
            supported_socials: supported.clone(),
            ..Default::default()
        }
        .serialize()
        .is_err(),);

        assert!(Social {
            profile_url: Some("//link".to_string()),
            supported_socials: supported.clone(),
            ..Default::default()
        }
        .serialize()
        .is_err());
    }
}
