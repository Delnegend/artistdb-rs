use std::collections::{HashMap, HashSet};

type SocialCode = String;

#[derive(Debug)]
pub struct SupportedSocials {
    unavatar: HashMap<SocialCode, (Description, ProfileUrl)>,
    extended: HashMap<SocialCode, (Description, ProfileUrl)>,
    specials: HashSet<SocialCode>,
}

impl Default for SupportedSocials {
    fn default() -> SupportedSocials {
        let mut new = SupportedSocials {
            unavatar: HashMap::new(),
            extended: HashMap::new(),
            specials: HashSet::new(),
        };

        new.unavatar = vec![
            ("x", "𝕏", "twitter.com/<@>"),
            ("twitter", "𝕏", "twitter.com/<@>"),
            ("telegram", "Telegram", "t.me/<@>"),
            ("deviantart", "DeviantArt", "deviantart.com/<@>"),
            ("instagram", "Instagram", "instagram.com/<@>"),
            ("dribbble", "Dribbble", "dribbble.com/<@>"),
            ("duckduckgo", "DuckDuckGo", ""),
            ("reddit", "Reddit", "reddit.com/user/<@>"),
            ("youtube", "YouTube", "youtube.com/<@>"),
            ("github", "GitHub", "github.com/<@>"),
            ("google", "Google", ""),
            ("gravatar", "Gravatar", ""),
            ("microlink", "Microlink", ""),
            ("readcv", "ReadCV", "read.cv/<@>"),
            ("reddit", "Reddit", "reddit.com/user/<@>"),
            ("soundcloud", "SoundCloud", "soundcloud.com/<@>"),
            ("substack", "Substack", "<@>.substack.com/"),
            ("youtube", "YouTube", "youtube.com/@<@>"),
            ("subscribestar", "SubscribeStar", "subscribestar.adult/<@>"),
            ("facebook", "Facebook", "fb.com/<@>"),
            ("fb", "Facebook", "fb.com/<@>"),
        ]
        .into_iter()
        .map(|(code, name, url)| (code.to_string(), (name.to_string(), url.to_string())))
        .collect();

        new.extended = vec![
            ("fa", "FurAffinity 🐾", "www.furaffinity.net/user/<@>/"),
            ("itaku", "Itaku", "itaku.ee/profile/<@>"),
            ("bsky", "BlueSky", "bsky.app/profile/<@>"),
            ("bluesky", "BlueSky", "bsky.app/profile/<@>"),
            ("threads", "Threads", "www.threads.net/@<@>"),
            ("tumblr", "Tumblr", "<@>.tumblr.com"),
            ("pixiv", "Pixiv", "www.pixiv.net/en/users/<@>"),
            ("patreon", "Patreon", "www.patreon.com/<@>"),
            ("kofi", "Ko-fi 🍵", "ko-fi.com/<@>"),
            ("plurk", "Plurk", "plurk.com/<@>"),
            ("linktr.ee", "Linktr.ee 🌲", "linktr.ee/<@>"),
            ("carrd.co", "Carrd.co", "<@>.carrd.co"),
            ("booth", "Booth.pm", "<@>.booth.pm"),
            ("skeb", "Skeb.jp", "skeb.jp/@<@>"),
            ("fanbox", "PixivFanbox", "<@>.fanbox.cc"),
            ("picarto", "Picarto", "www.picarto.tv/<@>"),
            ("gumroad", "Gumroad", "<@>.gumroad.com"),
            ("twitch", "Twitch", "www.twitch.tv/<@>"),
            ("lit.link", "Lit.link", "lit.link/<@>"),
            ("potofu.me", "Potofu.me", "potofu.me/<@>"),
        ]
        .into_iter()
        .map(|(code, name, url)| (code.to_string(), (name.to_string(), url.to_string())))
        .collect();

        new.specials = vec!["potofu.me", "carrd.co", "linktr.ee", "lit.link"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        new
    }
}

type ProfileUrl = String;
type Description = String;

impl SupportedSocials {
    /// Returns the display name of the social and the formatted profile URL.
    /// The description will be use to override the display name.
    pub fn get(
        &self,
        social_username: &str,
        social_code: &str,
        description: &Option<String>,
    ) -> Result<(ProfileUrl, Description), String> {
        let mut found_social = self.unavatar.get(social_code);
        if found_social.is_none() {
            found_social = self.extended.get(social_code);
        }
        match (found_social, description) {
            (None, _) => Err("unsupported social code".to_string())?,
            (Some((social_name, template)), None) => Ok((
                format!("//{}", template.replace("<@>", social_username)),
                social_name.clone(),
            )),
            (Some((social_name, template)), Some(description)) => Ok((
                format!("//{}", template.replace("<@>", social_username)),
                format!("{} | {}", social_name, description.clone()),
            )),
        }
    }

    // Returns true if the social code is artists' own link-in-bio
    pub fn is_special(&self, social_code: &Option<String>) -> bool {
        match social_code {
            Some(code) => self.specials.contains(code),
            None => false,
        }
    }

    // Returns true if the social code is supported by unavatar
    pub fn is_unavatar_supported(&self, code: &str) -> bool {
        self.unavatar.contains_key(code)
    }

    pub fn is_supported(&self, code: &str) -> bool {
        if self.unavatar.contains_key(code) {
            return true;
        }
        self.extended.contains_key(code)
    }
}
