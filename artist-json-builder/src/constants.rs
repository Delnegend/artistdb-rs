use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Constants<'a> {
    pub unavatar_socials: HashMap<String, SupportedSocial>,
    pub extended_socials: HashMap<String, SupportedSocial>,
    pub social_desc_format: &'a str,
}

impl<'a> Constants<'a> {
    pub fn new() -> Self {
        Self {
            unavatar_socials: unavatar_socials(),
            extended_socials: extended_socials(),
            social_desc_format: "{} | {}",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SupportedSocial {
    pub code: String,
    pub display: String,
    pub profile_url: Option<String>,
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
        ("readcv", "ReadCV", ""),
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
                profile_url: match url.is_empty() {
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
                profile_url: match url.is_empty() {
                    true => None,
                    false => Some(format!("https://{}", url)),
                },
            },
        )
    })
    .collect::<HashMap<String, SupportedSocial>>();

    unavatar_socials().into_iter().chain(extended).collect()
}
