use toml_edit::Table;
use tracing::warn;

static FLAG: &str = "__flag__";
static ALIAS: &str = "__alias__";
static AVATAR: &str = "__avatar__";
static NAME: &str = "__name__";

pub struct InfoParser<'a> {
    pub username: &'a str,
    pub info: &'a Table,
}

impl<'a> InfoParser<'a> {
    pub fn from(username: &'a str, info: &'a Table) -> Self {
        Self { username, info }
    }

    fn string_parser(&self, key: &str) -> Option<String> {
        self.info
            .get(key)
            .and_then(|item| {
                item.as_str()
                    .ok_or_else(|| {
                        warn!("{}: {} must be a string", self.username, key);
                    })
                    .ok()
            })
            .filter(|item| !item.is_empty())
            .map(|item| item.to_string())
    }

    pub fn flag(&self) -> Option<String> {
        self.string_parser(FLAG)
    }

    pub fn avatar(&self) -> Option<String> {
        self.string_parser(AVATAR)
    }

    pub fn name(&self) -> Option<String> {
        self.string_parser(NAME)
    }

    pub fn alias(&self) -> Vec<String> {
        self.info
            .get(ALIAS)
            .and_then(|alias| {
                alias
                    .as_array()
                    .ok_or_else(|| {
                        warn!("{}: alias must be an array", self.username);
                    })
                    .ok()
            })
            .map(|alias| {
                alias
                    .iter()
                    .filter_map(|item| {
                        item.as_str()
                            .filter(|item| !item.is_empty())
                            .map(|item| item.to_string())
                            .ok_or_else(|| {
                                warn!(
                                    "{}: elements of alias must be a non-empty string",
                                    self.username
                                )
                            })
                            .ok()
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    pub fn socials(&self) -> Vec<(String, String)> {
        self.info
            .into_iter()
            .filter(|(key, _)| *key != ALIAS && *key != AVATAR && *key != FLAG && *key != NAME)
            .filter_map(|(key, value)| {
                value
                    .as_str()
                    .ok_or_else(|| warn!("{}: {} must be a string", self.username, key))
                    .ok()
                    .filter(|value| !value.is_empty())
                    .ok_or_else(|| {
                        warn!("{}: {} is empty", self.username, key);
                    })
                    .ok()
                    .map(|value| (key.to_string(), value.to_string()))
            })
            .collect::<Vec<_>>()
    }
}
