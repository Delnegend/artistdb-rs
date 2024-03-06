use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Constants {
    pub unavatar_socials: HashMap<String, SupportedSocial>,
    pub extended_socials: HashMap<String, SupportedSocial>,
    pub special_socials: HashSet<String>,
    pub unavatar_size: u16,
}

impl Default for Constants {
    fn default() -> Self {
        Self {
            unavatar_socials: unavatar_socials(),
            extended_socials: extended_socials(),
            special_socials: special_socials(),

            unavatar_size: 400,
        }
    }
}

impl Constants {
    pub fn format_description(&self, code: &String, desc: &Option<String>) -> Option<String> {
        let display = self
            .extended_socials
            .get(code)
            .map_or(code.clone(), |social| social.display.clone());

        match desc {
            Some(desc) => Some(format!("{} | {}", display, desc)),
            None => Some(display),
        }
    }

    pub fn name_code_to_link(&self, code: &String, username: &Option<String>) -> Option<String> {
        let url_template: Option<String> = self
            .extended_socials
            .get(code)
            .map(|social| social.url_template.clone())
            .unwrap_or(None);

        match (username, url_template) {
            (Some(username), Some(url_template)) => {
                Some(url_template.replace("<USERNAME>", username))
            }
            _ => None,
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
        ("subscribestar", "SubscribeStar", "subscribestar.adult/<USERNAME>"),
        ("facebook", "Facebook", "fb.com/<USERNAME>"),
        ("fb", "Facebook", "fb.com/<USERNAME>"),
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
        ("linktree", "Linktr.ee", "linktr.ee/<USERNAME>"),
        ("linktr.ee", "Linktr.ee", "linktr.ee/<USERNAME>"),
        ("carrd", "Carrd.co", "<USERNAME>.carrd.co"),
        ("carrd.co", "Carrd.co", "<USERNAME>.carrd.co"),
        ("booth", "Booth.pm", "<USERNAME>.booth.pm"),
        ("booth.pm", "Booth.pm", "<USERNAME>.booth.pm"),
        ("skeb", "Skeb.jp", "skeb.jp/@<USERNAME>"),
        ("skeb.jp", "Skeb.jp", "skeb.jp/@<USERNAME>"),
        ("fanbox", "PixivFanbox", "<USERNAME>.fanbox.cc"),
        ("picarto", "Picarto", "www.picarto.tv/<USERNAME>"),
        ("picarto.tv", "Picarto", "www.picarto.tv/<USERNAME>"),
        ("gumroad", "Gumroad", "<USERNAME>.gumroad.com"),
        ("twitch", "Twitch", "www.twitch.tv/<USERNAME>"),
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

fn special_socials() -> HashSet<String> {
    vec!["potofu.me", "carrd.co", "linktr.ee"]
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}
