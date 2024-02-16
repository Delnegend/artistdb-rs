use serde::Serialize;

use crate::constants::SupportedSocial;

#[derive(Debug, PartialEq, Serialize, Default)]
pub struct ProcessedSocial {
    #[serde(skip_serializing)]
    pub match_social: Option<SupportedSocial>,

    #[serde(skip_serializing)]
    pub specific_uname: Option<String>,

    /// only for testing
    #[cfg(debug_assertions)]
    #[serde(skip_serializing)]
    pub desc_raw: Option<String>,

    /// NOTE: use the `code` field in match_social,
    /// this one is only for the JSON
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "url")]
    pub profile_url: Option<String>,
}
