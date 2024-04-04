// use murmur3::murmur3_x64_128;
// use tracing::warn;

// use crate::utils::parse_artists::Artists;

pub mod pipeline;
pub mod process_artists;
pub mod process_avatar;
pub mod process_info;
pub mod process_socials;
pub mod supported_socials;

// pub fn artists_hasher(artists: &Artists) -> u128 {
//     if artists.0.is_empty() {
//         return 0;
//     }

//     let content: Vec<u8> = match bridge::to_bitcode(&artists)
//         .map_err(|err| warn!("failed to serialize artists: {}", err))
//     {
//         Ok(content) => content,
//         Err(_) => return 0,
//     };

//     murmur3_x64_128(&mut content.as_slice(), 0).unwrap_or_default()
// }

/// Split by comma, but if an element ends with a `\`, join with the next
/// element with a `,`
pub fn split_components(raw: &str) -> Result<Vec<String>, String> {
    let mut components: Vec<String> = raw.split(',').map(|s| s.trim().to_string()).collect();

    for i in 0..components.len() {
        if components[i].ends_with('\\') {
            if i + 1 >= components.len() {
                return Err("invalid escape".to_string());
            }
            components[i] = components[i][..components[i].len() - 1].to_string();
            components[i] = format!("{},{}", components[i], components[i + 1]);
            components.remove(i + 1);
        }
    }

    Ok(components)
}
