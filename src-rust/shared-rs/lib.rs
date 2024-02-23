use std::collections::BTreeMap;

use bitcode::{decode, encode, Decode, Encode};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Encode, Decode)]
pub struct Social {
    pub code: String,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub link: Option<String>,
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

impl Artist {
    pub fn to_bitcode(&self) -> Result<Vec<u8>, String> {
        encode(&self).map_err(|err| format!("Failed to serialize artist: {}", err))
    }

    /// wasm-pack intepreted this as `Artist | undefined`, using Result is unnecessary
    pub fn from_bitcode(bytes: &[u8]) -> Option<Self> {
        decode(bytes).ok()
    }
}

pub type Artists = BTreeMap<String, Artist>;

pub fn to_bitcode<T>(value: &T) -> Result<Vec<u8>, String>
where
    T: ?Sized + Serialize + Encode,
{
    encode(value).map_err(|err| format!("Failed to serialize: {}", err))
}
