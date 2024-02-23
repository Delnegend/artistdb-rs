use shared::Artist;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// Deserialize a `Vec<u8>` into an `Artist` struct.
pub fn decode(data: Vec<u8>) -> Option<Artist> {
    Artist::from_bitcode(&data)
}

#[wasm_bindgen]
/// If the `Vec<u8>` is a valid alias, remove the `@` and return it as a `String`.
pub fn get_alias(data: Vec<u8>) -> Option<String> {
    let s = String::from_utf8(data).ok()?;

    s.strip_prefix('@')
        .map(|s| s.to_string())
        .map(|alias| alias.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::{Artist, Social};

    #[test]
    fn test_decode() {
        let mut socials: Vec<Social> = Vec::new();
        socials.push(Social {
            code: "code".to_string(),
            name: Some("Name".to_string()),
            desc: Some("Description".to_string()),
            link: Some("Link".to_string()),
        });

        let artist = Artist {
            alias: None,
            flag: None,
            name: Some("Name".to_string()),
            avatar: Some("avatar".to_string()),
            socials: Some(socials),
        };

        let encoded = artist.to_bitcode().unwrap();
        let decoded = decode(encoded).unwrap();

        assert_eq!(artist, decoded);
    }

    #[test]
    fn test_get_alias() {
        let alias = "@username".as_bytes().to_vec();
        let result = get_alias(alias).unwrap();

        assert_eq!("username", result);
    }
}
