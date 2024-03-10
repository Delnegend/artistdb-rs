use bridge::Socials;
use tracing::warn;

use crate::utils::constants::Constants;

fn unavatar(code: &str, username: &str, size: u16, fallback: &str) -> String {
    format!(
        "https://unavatar.io/{}/{}?size={}&fallback={}",
        code, username, size, fallback
    )
}

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
            if http {
                warn!("{}: avatar URL is not secure", log_username)
            }
            http || https
        })
        .unwrap_or(false);
    let avatar_is_root = avatar
        .as_ref()
        .map(|avatar| avatar.starts_with('/'))
        .unwrap_or(false);

    let avatar = match (avatar_is_root, avatar_is_url, avatar) {
        (true, _, Some(avatar)) => return Some(format!("/avatars{}", &avatar)),
        (false, true, Some(avatar)) => {
            return Some(avatar.clone());
        }
        (false, false, Some(avatar)) => Some(avatar.clone()),
        (_, _, None) => None,
    };

    // Manually specify `username@social`
    if let Some(avatar) = avatar {
        match avatar.split('@').collect::<Vec<&str>>().as_slice() {
            [target_uname, social_code] => match constants.unavatar_socials.get(*social_code) {
                Some(_) => {
                    let code = match social_code {
                        &"x" => "twitter".to_string(),
                        _ => social_code.to_string(),
                    };
                    return Some(unavatar(
                        &code,
                        target_uname,
                        constants.unavatar_size,
                        &constants.fallback_avatar,
                    ));
                }
                None => warn!(
                    "{}: unavatar doesn't support {}",
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
        .filter_map(|social| match (&social.code, &social.name) {
            (Some(code), Some(name)) => Some((code, name)),
            _ => None,
        })
        .find_map(|(code, username)| {
            let code = if code == "x" {
                "twitter".to_string()
            } else {
                code.to_string()
            };
            constants.unavatar_socials.get(&code).map(|_| {
                unavatar(
                    &code,
                    username,
                    constants.unavatar_size,
                    &constants.fallback_avatar,
                )
            })
        })
}

#[cfg(test)]
mod test {
    use bridge::Social;

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
                    code: Some("x".to_string()),
                    name: Some("foo".to_string()),
                    ..Default::default()
                }]),
                &None,
                &empty_string
            ),
            Some(unavatar(
                "twitter",
                "foo",
                400,
                "https://artistdb.delnegend.com/avatar.svg"
            ))
        );

        // same as above, but override everything
        assert_eq!(
            avatar_parser(
                &constants,
                &Some(vec![Social {
                    code: Some("reddit".to_string()),
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
                    code: Some("example".to_string()),
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
                    code: Some("example".to_string()),
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
                    code: Some("reddit".to_string()),
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
            Some(unavatar(
                "reddit",
                "foo",
                400,
                "https://artistdb.delnegend.com/avatar.svg"
            ))
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
                    code: Some("reddit".to_string()),
                    name: Some("foo".to_string()),
                    ..Default::default()
                }]),
                &Some("foo@example".to_string()),
                &empty_string
            ),
            Some(unavatar(
                "reddit",
                "foo",
                400,
                "https://artistdb.delnegend.com/avatar.svg"
            ))
        );
    }
}
