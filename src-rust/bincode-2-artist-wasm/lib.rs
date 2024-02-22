use shared::Artist;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// Deserialize a `Vec<u8>` into an `Artist` struct.
pub fn decode(data: Vec<u8>) -> Option<Artist> {
    Artist::from_bincode(&data)
}

#[wasm_bindgen]
/// If the `Vec<u8>` is a valid alias, remove the `@` and return it as a `String`.
pub fn get_alias(data: Vec<u8>) -> Option<String> {
    let s = String::from_utf8(data).ok()?;
    if s.starts_with('@') {
        Some(s[1..].to_string())
    } else {
        None
    }
}
