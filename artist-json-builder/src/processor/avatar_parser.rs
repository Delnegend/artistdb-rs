use tracing::{debug, warn};

use crate::{constants::Constants, processor::is_url};

use super::processed_social::ProcessedSocial;

/// Given a list of Socials, return the avatar URL of the first supported social.
pub fn avatar_parser(
    constants: &Constants,
    processed_socials: &[ProcessedSocial],
    avatar_value: Option<String>,
    log_username: &String,
) -> Option<String> {
    let unavatar_override = match (is_url(&avatar_value), avatar_value) {
        (true, Some(avatar)) => {
            debug!("Avatar is a URL: {}", &avatar);
            return Some(avatar);
        }
        (false, Some(avatar)) => {
            debug!("Avatar is not a URL: {}", avatar);
            Some(avatar)
        }
        (_, None) => None,
    };

    // Manually specify `username@social`
    if let Some(unavatar_override) = unavatar_override {
        match unavatar_override
            .split('@')
            .collect::<Vec<&str>>()
            .as_slice()
        {
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

    processed_socials
        .iter()
        .filter_map(|social| {
            if let Some(match_social) = &social.match_social {
                match (&match_social.code, &social.specific_uname) {
                    (code, Some(specific_uname)) => Some((code, specific_uname)),
                    _ => None,
                }
            } else {
                None
            }
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
    use crate::constants::{Constants, SupportedSocial};

    use super::*;

    #[test]
    fn test_get_avatar_url() {
        let constants = Constants::new();
        let binding = "".to_string();

        // code supported, username present (best case)
        assert_eq!(
            avatar_parser(
                &constants,
                &vec![ProcessedSocial {
                    match_social: Some(SupportedSocial {
                        code: "reddit".to_string(),
                        ..Default::default()
                    }),
                    specific_uname: Some("foo".to_string()),
                    code: Some("reddit".to_string()),
                    ..Default::default()
                }],
                None,
                &binding
            ),
            Some("https://unavatar.io/reddit/foo?size=400".to_string())
        );

        // same as above, but override everything
        assert_eq!(
            avatar_parser(
                &constants,
                &vec![ProcessedSocial {
                    match_social: Some(SupportedSocial {
                        code: "reddit".to_string(),
                        ..Default::default()
                    }),
                    specific_uname: Some("foo".to_string()),
                    code: Some("reddit".to_string()),
                    ..Default::default()
                }],
                Some("https://example.com/avatar.png".to_string()),
                &binding
            ),
            Some("https://example.com/avatar.png".to_string())
        );

        // provide nothing, only avatar override
        assert_eq!(
            avatar_parser(
                &constants,
                &vec![],
                Some("https://example.com/avatar.png".to_string()),
                &binding
            ),
            Some("https://example.com/avatar.png".to_string())
        );

        // provide everything, no avatar support, but override
        assert_eq!(
            avatar_parser(
                &constants,
                &vec![ProcessedSocial {
                    match_social: Some(SupportedSocial {
                        code: "example".to_string(),
                        ..Default::default()
                    }),
                    specific_uname: Some("foo".to_string()),
                    code: Some("example".to_string()),
                    ..Default::default()
                }],
                Some("https://example.com/avatar.png".to_string()),
                &binding
            ),
            Some("https://example.com/avatar.png".to_string())
        );

        // None cases

        // provide everything, unavatar not support
        assert_eq!(
            avatar_parser(
                &constants,
                &vec![ProcessedSocial {
                    match_social: Some(SupportedSocial {
                        code: "example".to_string(),
                        ..Default::default()
                    }),
                    specific_uname: Some("foo".to_string()),
                    code: Some("example".to_string()),
                    ..Default::default()
                }],
                None,
                &binding
            ),
            None
        );

        // unavatar support, but no username
        assert_eq!(
            avatar_parser(
                &constants,
                &vec![ProcessedSocial {
                    match_social: Some(SupportedSocial {
                        code: "reddit".to_string(),
                        ..Default::default()
                    }),
                    specific_uname: None,
                    code: Some("reddit".to_string()),
                    ..Default::default()
                }],
                None,
                &binding
            ),
            None
        );

        // manually provide username@social
        assert_eq!(
            avatar_parser(
                &constants,
                &vec![],
                Some("foo@reddit".to_string()),
                &binding
            ),
            Some("https://unavatar.io/reddit/foo?size=400".to_string())
        );

        // ... but when not found
        assert_eq!(
            avatar_parser(
                &constants,
                &vec![],
                Some("foo@example".to_string()),
                &binding
            ),
            None
        );

        // ... or auto-detect to another match
        assert_eq!(
            avatar_parser(
                &constants,
                &vec![ProcessedSocial {
                    match_social: Some(SupportedSocial {
                        code: "reddit".to_string(),
                        ..Default::default()
                    }),
                    specific_uname: Some("foo".to_string()),
                    code: Some("reddit".to_string()),
                    ..Default::default()
                }],
                Some("foo@example".to_string()),
                &binding
            ),
            Some("https://unavatar.io/reddit/foo?size=400".to_string())
        );
    }
}
