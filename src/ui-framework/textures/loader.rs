use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::{Component, Path, PathBuf};

use anyhow::{anyhow, Result};
use glium::texture::RawImage2d;
use log::debug;

use crate::vendor::{Texture2d, TextureName};
use crate::Display;

pub struct TextureLoader;

impl TextureLoader {
    pub fn name(base: &Path, texture_dir: &Path, texture_file: &Path) -> Result<String> {
        let mut name: Vec<_> = texture_dir
            .strip_prefix(base)?
            .components()
            .filter_map(|component| match component {
                Component::Normal(str) => Some(str.to_string_lossy().to_string()),

                _ => None,
            })
            .collect();

        name.push(
            texture_file
                .file_stem()
                .ok_or_else(|| anyhow!("File did not have file stem {:?}", texture_file))?
                .to_str()
                .ok_or_else(|| anyhow!("Error converting file stem to string {:?}", texture_file))?
                .to_owned(),
        );

        Ok(name.join(":"))
    }

    /// Loads all the textures in the asset directory, keyed based on their file name
    pub fn load_dir(display: &Display, path: &Path) -> Result<HashMap<TextureName, Texture2d>> {
        debug!("Loading textures from directory: {}", path.display());

        // @todo replace with walkdir
        let mut textures: HashMap<TextureName, Texture2d> = HashMap::new();

        let mut pending_dirs = vec![PathBuf::from(path)];

        while let Some(dir) = pending_dirs.pop() {
            for entry in fs::read_dir(&dir)? {
                let entry = entry?;
                let entry_path = entry.path();
                let entry_meta = fs::metadata(&entry_path)?;

                if entry_meta.is_dir() {
                    pending_dirs.push(entry.path());
                    continue;
                }

                if let Some(texture) = Self::load(display, &entry_path) {
                    let texture_name = Self::name(path, &dir, &entry_path)?;
                    textures.insert(texture_name, texture);
                }
            }
        }

        Ok(textures)
    }

    /// Loads an asset at a specific path
    fn load(display: &Display, texture: &Path) -> Option<Texture2d> {
        if texture.extension() == Some(OsStr::new("png")) {
            Some(Self::load_png(display, texture))
        } else {
            None
        }
    }

    fn load_png(display: &Display, texture: &Path) -> Texture2d {
        let image = image::open(texture).unwrap().to_rgba();

        let image_dim = image.dimensions();
        let image_data = image.into_raw();

        let texture = RawImage2d::from_raw_rgba_reversed(&image_data, image_dim);

        Texture2d::new(&display.0, texture).unwrap()
    }
}
