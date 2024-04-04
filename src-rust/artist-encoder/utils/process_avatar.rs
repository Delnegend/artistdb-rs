use tracing::warn;

use crate::utils::process_artists::Artist;

fn unavatar(social_username: &str, social_code: &str) -> String {
    format!("{}/{}", social_code, social_username)
}

/// Transforming the Artist's avatar in raw form into a proper URL for the frontend
impl Artist {
    pub fn serialize_avatar(&self) -> Result<String, String> {
        let supported_socials = self.supported_socials.as_ref();

        let avatar = match &self.avatar {
            // URL
            Some(avatar) if avatar.starts_with("//") => return Ok(avatar.clone()),
            // From root
            Some(avatar) if avatar.starts_with('/') && !avatar.starts_with("//") => {
                return Ok(format!("/avatars{}", &avatar));
            }
            // Auto inferred
            Some(avatar) if avatar != "_" => Some(avatar.clone()),
            _ => None,
        };

        // Manually specify `username@social`
        if let Some(avatar) = avatar {
            let components = avatar.split('@').collect::<Vec<&str>>();
            if components.len() != 2 {
                warn!("{}: Invalid avatar format: {}", self.username, &avatar);
                return Ok("_".to_string());
            }
            let (social_username, social_code) = (components[0], components[1]);
            let social_code = match social_code {
                "x" => "twitter".to_string(),
                _ => social_code.to_string(),
            };
            if supported_socials.is_unavatar_supported(&social_code) {
                return Ok(unavatar(social_username, &social_code));
            }
        }

        let result: Option<(String, String)> =
            self.socials
                .iter()
                .find_map(|social| match (social.get_code(), social.get_name()) {
                    (Some(code), Some(name)) if !name.is_empty() => {
                        if !supported_socials.is_unavatar_supported(&code) {
                            return None;
                        }
                        if code == "x" {
                            return Some((name, "twitter".to_string()));
                        }
                        Some((name, code))
                    }
                    _ => None,
                });

        if let Some((username, code)) = result {
            return Ok(unavatar(&username, &code));
        }
        warn!("{}: no supported socials found", self.username);

        Ok("_".to_string())
    }
}

#[cfg(test)]
mod test {

    use std::rc::Rc;

    use crate::utils::{process_socials::Social, supported_socials::SupportedSocials};

    use super::*;

    #[test]
    fn serialize_valid() {
        let supported_socials = Rc::from(SupportedSocials::default());
        let example_url = "//example.com/avatar.png".to_string();

        // social & name present
        let mut artist = Artist::default();
        let social = Social::new(supported_socials.clone());
        artist.socials = vec![social.parse_into("foo@twitter").unwrap()];
        assert_eq!(artist.serialize_avatar().unwrap(), "twitter/foo");

        // only avatar override
        let mut artist = Artist::default();
        artist.avatar = Some(example_url.clone());
        assert_eq!(artist.serialize_avatar().unwrap(), example_url);

        // valid provided avatar
        let mut artist = Artist::default();
        artist.avatar = Some("foo@twitter".to_string());
        assert_eq!(artist.serialize_avatar().unwrap(), "twitter/foo")
    }

    #[test]
    fn serialize_fallback() {
        let supported_socials = Rc::from(SupportedSocials::default());

        // unavatar support, but no username
        let mut artist = Artist::default();
        let social = Social::new(supported_socials.clone());
        artist.socials = vec![social.parse_into("@twitter").unwrap()];
        assert_eq!(artist.serialize_avatar().unwrap(), "_");

        // invalid provided avatar
        let mut artist = Artist::default();
        artist.avatar = Some("foo@example".to_string());
        assert_eq!(artist.serialize_avatar().unwrap(), "_");

        // invalid provided avatar, auto match from a valid social
        let mut artist = Artist::default();
        let social = Social::new(supported_socials.clone());
        artist.avatar = Some("foo@example".to_string());
        artist.socials = vec![social.parse_into("foo@reddit").unwrap()];
        assert_eq!(artist.serialize_avatar().unwrap(), "reddit/foo");

        // empty
        let artist = Artist::default();
        assert_eq!(artist.serialize_avatar().unwrap(), "_");
    }

    #[test]
    fn serialize_auto() {
        let supported_socials = Rc::from(SupportedSocials::default());

        let mut artist = Artist::default();
        let social = Social::new(supported_socials.clone());
        artist.avatar = Some("_".to_string());
        artist.socials = vec![social.parse_into("foo@twitter").unwrap()];
        assert_eq!(artist.serialize_avatar().unwrap(), "twitter/foo");
    }
}
