use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Constants {
    pub unavatar_socials: HashMap<String, SupportedSocial>,
    pub extended_socials: HashMap<String, SupportedSocial>,
    pub unavatar_size: u16,
}

impl Constants {
    pub fn new() -> Self {
        Self {
            unavatar_socials: unavatar_socials(),
            extended_socials: extended_socials(),
            unavatar_size: 400,
        }
    }

    pub fn format_description(
        &self,
        social: &Option<SupportedSocial>,
        desc: &Option<String>,
    ) -> Option<String> {
        let result = match (social, desc) {
            (Some(social), Some(desc)) => format!("{} | {}", social.display, desc),
            (Some(social), None) => social.display.to_string(),
            (None, Some(desc)) => desc.to_string(),
            (None, None) => "".to_string(),
        };

        match result.is_empty() {
            true => None,
            false => Some(result),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SupportedSocial {
    pub code: String,
    pub display: String,
    pub url_template: Option<String>,
}

fn unavatar_socials() -> HashMap<String, SupportedSocial> {
    Vec::from([
        ("x", "𝕏", "twitter.com/<USERNAME>"),
        ("twitter", "𝕏", "twitter.com/<USERNAME>"),
        ("telegram", "Telegram", "t.me/<USERNAME>"),
        ("deviantart", "DeviantArt", "deviantart.com/<USERNAME>"),
        ("instagram", "Instagram", "instagram.com/<USERNAME>"),
        ("dribbble", "Dribbble", "dribbble.com/<USERNAME>"),
        ("duckduckgo", "DuckDuckGo", ""),
        ("reddit", "Reddit", "reddit.com/user/<USERNAME>"),
        ("youtube", "YouTube", "youtube.com/<USERNAME>"),
        ("github", "GitHub", "github.com/<USERNAME>"),
        ("google", "Google", ""),
        ("gravatar", "Gravatar", ""),
        ("microlink", "Microlink", ""),
        ("readcv", "ReadCV", "read.cv/<USERNAME>"),
        ("reddit", "Reddit", "reddit.com/user/<USERNAME>"),
        ("soundcloud", "SoundCloud", "soundcloud.com/<USERNAME>"),
        ("substack", "Substack", "<USERNAME>.substack.com/"),
        ("youtube", "YouTube", "youtube.com/@<USERNAME>"),
    ])
    .into_iter()
    .map(|(code, display, url)| {
        (
            code.to_string(),
            SupportedSocial {
                code: code.to_string(),
                display: display.to_string(),
                url_template: match url.is_empty() {
                    true => None,
                    false => Some(format!("https://{}", url)),
                },
            },
        )
    })
    .collect()
}

fn extended_socials() -> HashMap<String, SupportedSocial> {
    let extended = Vec::from([
        (
            "fa",
            "FurAffinity 🐾",
            "www.furaffinity.net/user/<USERNAME>/",
        ),
        ("itaku", "Itaku", "itaku.ee/profile/<USERNAME>"),
        ("bsky", "BlueSky", "bsky.app/profile/<USERNAME>"),
        ("bluesky", "BlueSky", "bsky.app/profile/<USERNAME>"),
        ("threads", "Threads", "www.threads.net/@<USERNAME>"),
        ("tumblr", "Tumblr", "<USERNAME>.tumblr.com"),
        ("pixiv", "Pixiv", "www.pixiv.net/en/users/<USERNAME>"),
        ("patreon", "Patreon", "www.patreon.com/<USERNAME>"),
        ("kofi", "Ko-fi", "ko-fi.com/<USERNAME>"),
        ("plurk", "Plurk", "plurk.com/<USERNAME>"),
    ])
    .into_iter()
    .map(|(code, display, url)| {
        (
            code.to_string(),
            SupportedSocial {
                code: code.to_string(),
                display: display.to_string(),
                url_template: match url.is_empty() {
                    true => None,
                    false => Some(format!("https://{}", url)),
                },
            },
        )
    })
    .collect::<HashMap<String, SupportedSocial>>();

    unavatar_socials().into_iter().chain(extended).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_format_description() {
        let constants = Constants::new();
        let social = Some(SupportedSocial {
            code: "twitter".to_string(),
            display: "𝕏".to_string(),
            url_template: None,
        });

        // social and desc present
        assert_eq!(
            constants.format_description(&social, &Some("foo".to_string())),
            Some("𝕏 | foo".to_string())
        );

        // social present, desc absent
        assert_eq!(
            constants.format_description(&social, &None),
            Some("𝕏".to_string())
        );

        // social absent, desc present
        assert_eq!(
            constants.format_description(&None, &Some("foo".to_string())),
            Some("foo".to_string())
        );

        // social and desc absent
        assert_eq!(constants.format_description(&None, &None), None);
    }
}
