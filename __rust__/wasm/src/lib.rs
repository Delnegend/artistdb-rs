use wasm_bindgen::prelude::*;

struct ProvidedSocial {
    social: String,
    description: String,
    username: Option<String>,
    user_url: Option<String>,
}

struct SupportedSocial {
    code: String,
    display: String,
    profile_url_template: Option<String>,
}

fn unavatar_socials() -> Vec<SupportedSocial> {
    Vec::from([
        ("twitter", "Twitter", "https://twitter.com/{}"),
        ("telegram", "Telegram", "https://t.me/{}"),
        ("deviantart", "DeviantArt", "https://deviantart.com/{}"),
        ("instagram", "Instagram", "https://instagram.com/{}"),
        ("dribbble", "Dribbble", "https://dribbble.com/{}"),
        ("duckduckgo", "DuckDuckGo", ""),
        ("reddit", "Reddit", "https://reddit.com/user/{}"),
        ("youtube", "YouTube", "https://youtube.com/{}"),
        ("github", "GitHub", "https://github.com/{}"),
        ("google", "Google", ""),
        ("gravatar", "Gravatar", ""),
        ("microlink", "Microlink", ""),
        ("readcv", "ReadCV", ""),
        ("reddit", "Reddit", "https://reddit.com/user/{}"),
        ("soundcloud", "SoundCloud", "https://soundcloud.com/{}"),
        ("substack", "Substack", "https://{}.substack.com/"),
        ("youtube", "YouTube", "https://youtube.com/@{}"),
    ])
    .into_iter()
    .map(|(code, display, profile)| SupportedSocial {
        code: code.to_string(),
        display: display.to_string(),
        profile_url_template: profile.is_empty().then(|| profile.to_string()),
    })
    .collect::<Vec<SupportedSocial>>()
}

fn extended_socials() -> Vec<SupportedSocial> {
    let extended = Vec::from([
        ("fa", "FurAffinity", "https://www.furaffinity.net/user/{}/"),
        ("itaku", "Itaku", "https://itaku.ee/profile/{}"),
        ("bsky", "BlueSky", "https://bsky.app/profile/{}"),
        ("bluesky", "BlueSky", "https://bsky.app/profile/{}"),
        ("threads", "Threads", "https://www.threads.net/@{}"),
        ("tumblr", "Tumblr", "https://{}.tumblr.com")
    ])
    .into_iter()
    .map(|(code, display, profile)| SupportedSocial {
        code: code.to_string(),
        display: display.to_string(),
        profile_url_template: profile.is_empty().then(|| profile.to_string()),
    });

    unavatar_socials().into_iter().chain(extended).collect()
}

/// Get the avatar URL given a list of artist socials.
/// This function will normalize to lowercase and search using `.contains()`,
/// so no need to match the exact social name.
///
/// `param` provided_socials: Vec<String> - a list of `<social>\n<username>`
///
/// `returns` Option<String> - The URL of the avatar or None if not found
#[wasm_bindgen]
pub fn get_avatar_url(provided_socials: Vec<String>) -> Option<String> {
    // Internal note, for consistency:
    // - `social` = social name part, not including the username (INSTAgram 2)
    // - `username` = username part (foobar)
    // - `provided` = (<`social`>, <`username`>)
    // As mentioned in the description of supported_socials(),
    // - `code` = social code (instagram)
    // - `display` = social display name (Instagram, notice the capital I)

    let provideds = provided_socials
        .into_iter()
        .filter_map(|item| {
            item.split_once("\n")
                .map(|(social, username)| (social.to_string(), username.to_string()))
        })
        .filter(|(social, username)| !social.is_empty() && !username.is_empty())
        .collect::<Vec<(String, String)>>();

    let codes: Vec<String> = unavatar_socials()
        .into_iter()
        .map(|social| social.code)
        .collect();

    provideds
        .into_iter()
        .find_map(|(social, username)| {
            codes.iter().find_map(
                |supported| match social.to_lowercase().contains(supported) {
                    true => Some(format!(
                        "https://unavatar.io/{}/{}?size=400",
                        supported, username
                    )),
                    false => None,
                },
            )
        })
}

/// Format the socials to correct social names, keeping everything else intact
///
/// ```rust
/// let socials = vec!["iNstAgram:alt\nfoobar", "twitter\nfoobar1"]
///     .into_iter()
///     .map(|s| s.to_string())
///     .collect();
/// let formatted = format_social_name(socials);
/// assert_eq!(formatted, vec!["Instagram:alt\nhttps://instagram.com/foobar", "Twitter\nhttps://twitter.com/foobar1"]);
/// ```
#[wasm_bindgen]
pub fn format_socials(provided_socials: Vec<String>) -> Vec<String> {
    provided_socials
        .into_iter()
        .map(|provided| format_social(provided, " | "))
        .collect()
}

fn format_social(mut provided: String, delimiter: &str) -> String {
    // let (display, replace_range) = supported_socials()
    //     .into_iter()
    //     .find_map(|(code, display)| {
    //         let start = provided.to_lowercase().find(&code);
    //         match start {
    //             Some(start) => {
    //                 let end = start + code.len();
    //                 Some((display, (start, end)))
    //             }
    //             None => None,
    //         }
    //     })
    //     .into_iter()
    //     .next()
    //     .unwrap_or((String::from(""), (0, 0)));

    if display.is_empty() {
        return provided;
    }

    let (start, end) = (replace_range.0, replace_range.1);
    provided.replace_range(start..end, &display);
    provided
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_socials() {
        let socials = unavatar_socials();
        assert_eq!(socials.len(), 15);
    }

    #[test]
    fn test_get_avatar_url() {
        let socials = vec!["twitte\nfoobar1", "telegram\nfoobar2", "youtube\nfoobar3"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        let url = get_avatar_url(socials);

        assert_eq!(
            url,
            Some("https://unavatar.io/telegram/foobar2?size=400".to_string())
        );
    }

    #[test]
    fn test_format_social_name() {
        let socials = vec![
            "iNstAgram alt\nfoobar",
            "twitte\nfoobar1",
            "youtube\nfoobar3",
            "duckduckgo\nfoobar4",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

        let formatted = format_socials(socials);

        assert_eq!(
            formatted,
            vec![
                "Instagram alt\nfoobar",
                "twitte\nfoobar1",
                "YouTube\nfoobar3",
                "DuckDuckGo\nfoobar4"
            ]
        );
    }
}
