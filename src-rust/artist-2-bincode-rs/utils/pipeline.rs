use std::path::PathBuf;

use crate::{
    utils::{artists_serde::ArtistsSerde, constants::Constants},
    Args,
};
use shared::Artist;
use tracing::{error, info};

pub struct Pipeline<'a> {
    args: &'a Args,
    constants: &'a Constants,
    last_content_hash: u128,
}

impl<'a> Pipeline<'a> {
    pub fn new(constants: &'a Constants, args: &'a Args) -> Self {
        Self {
            args,
            constants,
            last_content_hash: 0,
        }
    }

    /// Read toml -> preprocess -> format -> write .toml and bincode files
    pub fn run(&mut self) {
        let artists_serde = ArtistsSerde::from(&self.args.input_file, self.constants);

        let something_changed = 'scoped: {
            let curr_content_hash = super::artists_hasher(&artists_serde.artists);

            let content_hash_changed =
                curr_content_hash != self.last_content_hash && curr_content_hash != 0;

            if content_hash_changed {
                info!("content hash changed");
                self.last_content_hash = curr_content_hash;
                break 'scoped true;
            }

            if artists_serde.content_change_after_post_proc {
                info!("content changed after post processing");
                break 'scoped true;
            }
            false
        };

        if !something_changed {
            return;
        }

        // re-create output dir, write files
        self.re_create_output_dir()
            .unwrap_or_else(|err| error!("{}", err));

        artists_serde
            .pre_bincode_ser()
            .into_iter()
            .for_each(|(username, artist)| {
                self.write_bincode(&username, artist)
                    .unwrap_or_else(|err| error!("{}: {}", username, err));
            });

        std::thread::sleep(std::time::Duration::from_millis(
            self.args.toml_save_delay_ms,
        ));

        // write back to toml
        std::fs::write(
            &self.args.input_file,
            artists_serde.to_toml_string().unwrap_or_default(),
        )
        .map_err(|err| error!("can't write toml file: {}", err))
        .ok();
    }

    fn re_create_output_dir(&self) -> Result<(), String> {
        if std::path::Path::new(&self.args.output_dir).exists() {
            std::fs::remove_dir_all(&self.args.output_dir)
                .map_err(|err| format!("can't remove output dir: {}", err))?
        }
        std::fs::create_dir_all(&self.args.output_dir)
            .map_err(|err| format!("can't create output dir: {}", err))?;

        Ok(())
    }

    fn write_bincode(&mut self, username: &String, artist: Artist) -> Result<(), String> {
        // Main files
        let bincode_string = artist.to_bitcode()?;
        let path = PathBuf::from(format!(
            "{}/{}",
            self.args.output_dir,
            username.to_lowercase()
        ));

        std::fs::write(path, bincode_string)
            .map_err(|err| format!("{}: can't write bincode file: {}", username, err))?;

        // Alias files
        let alias = match &artist.alias {
            Some(alias) => alias,
            None => return Ok(()),
        };

        // Content: "@" + username
        alias.iter().for_each(|alias| {
            let content = format!("@{}", username);
            let path = PathBuf::from(format!("{}/{}", self.args.output_dir, alias.to_lowercase()));
            std::fs::write(path, content)
                .unwrap_or_else(|err| error!("{}: can't write alias file: {}", alias, err));
        });

        Ok(())
    }
}
