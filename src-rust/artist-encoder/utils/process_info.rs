use crate::utils::process_artists::Artist;

fn cleanup_name(raw: &str) -> String {
    raw.chars()
        .map(|c| match c {
            'a'..='z' | '0'..='9' | '-' | '_' | ' ' => c,
            'A'..='Z' => c.to_ascii_lowercase(),
            _ => '_',
        })
        .collect()
}

impl Artist {
    /// Parsing username, display name, avatar and aliases into Artist
    pub fn parse_info(&mut self, raw: &str) -> Result<(), String> {
        let components = super::split_components(raw)?;

        if components.is_empty() {
            return Err("missing fields to parse the info".to_string());
        }

        let username = components
            .first()
            .map(|username| cleanup_name(username))
            .ok_or("missing username")?;

        self.display_name = match components.get(1) {
            Some(name) if name == &"_".to_string() => None,
            Some(name) => Some(name.clone()),
            _ => None,
        };

        if let Some(avatar) = components.get(2) {
            self.original_avatar = Some(avatar.clone());
            if avatar != "_" {
                self.avatar = Some(avatar.to_string());
            }
        }

        self.alias = components
            .iter()
            .skip(3)
            .map(|alias| cleanup_name(alias))
            .collect();

        self.username = username.to_string();
        Ok(())
    }

    pub fn serialize_info_for_original(&self) -> Result<String, String> {
        let display_name = match &self.display_name {
            Some(display_name) => display_name.to_string(),
            None => "_".to_string(),
        };

        let avatar = match &self.original_avatar {
            Some(avatar) => avatar.to_string(),
            None => "_".to_string(),
        };

        let alias = match &self.alias {
            alias if !alias.is_empty() => format!(",{}", alias.join(",")),
            _ => "".to_string(),
        };

        Ok(format!(
            "{},{},{}{}",
            &self.username, display_name, avatar, alias
        ))
    }
    pub fn serialize_info(&self) -> Result<String, String> {
        let display_name = match &self.display_name {
            Some(display_name) => display_name.clone(),
            None => self.username.clone(),
        };
        Ok(format!("{},{}", display_name, self.serialize_avatar()?))
    }
}
