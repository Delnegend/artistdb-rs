use std::{path::PathBuf, rc::Rc};

use crate::utils::{
    process_artists::{Artist, Artists},
    supported_socials::SupportedSocials,
};
use tracing::error;

pub struct Pipeline<'a> {
    pub in_file: &'a String,
    pub out_dir: &'a String,
    pub supported_socials: Rc<SupportedSocials>,
}

impl<'a> Pipeline<'a> {
    pub fn run(&self) {
        let artists = Artists::from_file(self.supported_socials.clone(), "artists.txt");

        // re-create output dir, write files
        self.recreate_out_dir()
            .unwrap_or_else(|err| error!("{}", err));

        artists.get_artists().iter().for_each(|artist| {
            self.write_to_out_dir(artist)
                .unwrap_or_else(|err| error!("{}", err));
        });
    }

    fn recreate_out_dir(&self) -> Result<(), String> {
        if std::path::Path::new(&self.out_dir).exists() {
            std::fs::remove_dir_all(self.out_dir)
                .map_err(|err| format!("can't remove output dir: {}", err))?
        }
        std::fs::create_dir_all(self.out_dir)
            .map_err(|err| format!("can't create output dir: {}", err))?;

        Ok(())
    }

    fn write_to_out_dir(&self, artist: &Artist) -> Result<(), String> {
        // Main files
        let path = PathBuf::from(format!("{}/{}", self.out_dir, artist.username));
        let contents = artist.serialize()?;

        std::fs::write(path, contents)
            .map_err(|err| format!("{}: can't write dist file: {}", artist.username, err))?;

        // Alias files, contents: "@" + username
        artist.alias.iter().for_each(|alias| {
            let content = format!("@{}", artist.username);
            let path = PathBuf::from(format!("{}/{}", self.out_dir, alias));
            std::fs::write(path, content)
                .unwrap_or_else(|err| error!("{}: can't write alias file: {}", alias, err));
        });

        Ok(())
    }
}
