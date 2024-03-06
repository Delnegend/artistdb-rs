use std::collections::BTreeMap;

use bitcode::{decode, encode, Decode, Encode};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Encode, Decode)]
pub struct Social {
    pub code: Option<String>,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub link: Option<String>,
    pub special: Option<bool>,
}

pub type Socials = Vec<Social>;
pub type Alias = Vec<String>;

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Encode, Decode)]
pub struct Artist {
    pub flag: Option<String>,
    pub avatar: Option<String>,
    pub name: Option<String>,
    pub alias: Option<Alias>,
    pub socials: Option<Socials>,
}

#[wasm_bindgen]
impl Artist {
    pub fn to_bitcode(&self) -> Result<Vec<u8>, String> {
        encode(&self).map_err(|err| format!("Failed to serialize artist: {}", err))
    }

    /// wasm-pack intepreted this as `Artist | undefined`, using Result is unnecessary
    pub fn from_bitcode(bytes: &[u8]) -> Option<Artist> {
        decode(bytes).ok()
    }
}

#[wasm_bindgen]
/// If the `Vec<u8>` is a valid alias, remove the `@` and return it as a `String`.
pub fn get_alias(data: Vec<u8>) -> Option<String> {
    let s = String::from_utf8(data).ok()?;

    s.strip_prefix('@')
        .map(|s| s.to_string())
        .map(|alias| alias.to_string())
}

pub type Artists = BTreeMap<String, Artist>;

pub fn to_bitcode<T>(value: &T) -> Result<Vec<u8>, String>
where
    T: ?Sized + Serialize + Encode,
{
    encode(value).map_err(|err| format!("Failed to serialize: {}", err))
}

#[wasm_bindgen]
pub fn is_special(link: &str) -> bool {
    ["linktr.ee", "carrd.co", "potofu.me"]
        .iter()
        .any(|special| link.contains(special))
}