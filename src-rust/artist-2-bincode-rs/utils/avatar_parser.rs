use shared::Socials;
use tracing::warn;

use crate::utils::constants::Constants;

/// Given a list of Socials, return the avatar URL of the first supported social.
pub fn avatar_parser(
    constants: &Constants,
    socials: &Option<Socials>,
    avatar: &Option<String>,
    log_username: &String,
) -> Option<String> {
    let avatar_is_url = avatar
        .as_ref()
        .map(|avatar| {
            let http = avatar.starts_with("http://");
            let https = avatar.starts_with("https://");
            http || https
        })
        .unwrap_or(false);

    let avatar = match (avatar_is_url, avatar) {
        (true, Some(avatar)) => {
            return Some(avatar.clone());
        }
        (false, Some(avatar)) => Some(avatar),
        (_, None) => None,
    };

    // Manually specify `username@social`
    if let Some(avatar) = avatar {
        match avatar.split('@').collect::<Vec<&str>>().as_slice() {
            [target_uname, social_code] => match constants.unavatar_socials.get(*social_code) {
                Some(_) => {
                    return Some(format!(
                        "https://unavatar.io/{}/{}?size={}",
                        &social_code, &target_uname, &constants.unavatar_size
                    ));
                }
                None => warn!(
                    "{}: unavatar not support for {}",
                    log_username, &social_code
                ),
            },
            _ => warn!("{}: invalid avatar format", log_username),
        }
    }

    let socials = match socials {
        Some(socials) => socials,
        None => {
            return None;
        }
    };

    socials
        .iter()
        .filter_map(|social| {
            let social_specific_uname = social.name.as_ref();
            social_specific_uname.map(|name| (&social.code, name))
        })
        .find_map(|(code, username)| {
            constants.unavatar_socials.get(code).map(|_| {
                format!(
                    "https://unavatar.io/{}/{}?size={}",
                    code, username, constants.unavatar_size
                )
            })
        })
}

#[cfg(test)]
mod test {
    use shared::Social;

    use crate::utils::constants::Constants;

    use super::*;

    #[test]
    fn test_get_avatar_url() {
        let constants = Constants::default();
        let empty_string = "".to_string();
        let example_url = Some("https://example.com/avatar.png".to_string());

        // code supported, name present (best case)
        assert_eq!(
            avatar_parser(
                &constants,
                &Some(vec![Social {
                    code: "reddit".to_string(),
                    name: Some("foo".to_string()),
                    ..Default::default()
                }]),
                &None,
                &empty_string
            ),
            Some("https://unavatar.io/reddit/foo?size=400".to_string())
        );

        // same as above, but override everything
        assert_eq!(
            avatar_parser(
                &constants,
                &Some(vec![Social {
                    code: "reddit".to_string(),
                    name: Some("foo".to_string()),
                    ..Default::default()
                }]),
                &example_url,
                &empty_string
            ),
            example_url
        );

        // provide nothing, only avatar override
        assert_eq!(
            avatar_parser(&constants, &None, &example_url, &empty_string),
            Some("https://example.com/avatar.png".to_string())
        );

        // provide everything, no avatar support, but override
        assert_eq!(
            avatar_parser(
                &constants,
                &Some(vec![Social {
                    code: "example".to_string(),
                    name: Some("foo".to_string()),
                    ..Default::default()
                }]),
                &example_url,
                &empty_string
            ),
            example_url
        );

        // None cases

        // provide everything, unavatar not support
        assert_eq!(
            avatar_parser(
                &constants,
                &Some(vec![Social {
                    code: "example".to_string(),
                    name: Some("foo".to_string()),
                    ..Default::default()
                }]),
                &None,
                &empty_string
            ),
            None
        );

        // unavatar support, but no username
        assert_eq!(
            avatar_parser(
                &constants,
                &Some(vec![Social {
                    code: "reddit".to_string(),
                    name: None,
                    ..Default::default()
                }]),
                &None,
                &empty_string
            ),
            None
        );

        // manually provide username@social
        assert_eq!(
            avatar_parser(
                &constants,
                &None,
                &Some("foo@reddit".to_string()),
                &empty_string
            ),
            Some("https://unavatar.io/reddit/foo?size=400".to_string())
        );

        // ... but when not found
        assert_eq!(
            avatar_parser(
                &constants,
                &None,
                &Some("foo@example".to_string()),
                &empty_string
            ),
            None
        );

        // ... or auto-detect to another match
        assert_eq!(
            avatar_parser(
                &constants,
                &Some(vec![Social {
                    code: "reddit".to_string(),
                    name: Some("foo".to_string()),
                    ..Default::default()
                }]),
                &Some("foo@example".to_string()),
                &empty_string
            ),
            Some("https://unavatar.io/reddit/foo?size=400".to_string())
        );
    }
}
