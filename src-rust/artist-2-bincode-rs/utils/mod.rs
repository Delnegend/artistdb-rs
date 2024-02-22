use murmur3::murmur3_x64_128;
use shared::Artists;
use tracing::warn;

pub mod artists_serde;
pub mod avatar_parser;
pub mod constants;
pub mod pipeline;

pub fn artists_hasher(artists: &Artists) -> u128 {
    if artists.is_empty() {
        return 0;
    }

    let content: Vec<u8> = match bincode::serialize(&artists)
        .map_err(|err| warn!("failed to serialize artists: {}", err))
    {
        Ok(content) => content,
        Err(_) => return 0,
    };

    murmur3_x64_128(&mut content.as_slice(), 0).unwrap_or_default()
}
