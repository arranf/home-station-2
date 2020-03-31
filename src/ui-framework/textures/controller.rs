use anyhow::Result;
use log::trace;

use std::collections::HashMap;
use std::path::Path;

use crate::vendor::{ImageId, ImageMap};
use crate::{Display, TextureLoader};

/// Stores the `Display` and `ImageMap`
pub struct TextureController<'sys> {
    display: &'sys Display,
    image_map: ImageMap,
    images: HashMap<String, ImageId>,
}

impl<'sys> TextureController<'sys> {
    /// Creates a new `TextureController`
    pub fn new(display: &'sys Display, image_map: ImageMap) -> Self {
        Self {
            display,
            image_map,
            images: HashMap::new(),
        }
    }

    /// Initializes the `TextureController` by loading a path of assets.
    pub fn initialize(&mut self, assets: &Path) -> Result<()> {
        for (texture_name, texture) in TextureLoader::load_dir(self.display, assets)? {
            let texture_id = self.image_map.insert(texture);

            trace!(
                "Texture `{}` is now available as `{:?}`",
                texture_name,
                texture_id
            );

            self.images.insert(texture_name, texture_id);
        }
        Ok(())
    }

    /// Gets an image id by its key
    pub fn get(&self, name: &str) -> Option<ImageId> {
        self.images.get(name).map(ToOwned::to_owned)
    }

    /// Get the extension of an image (?)
    pub fn get_ex(&self, name: &[&str]) -> Option<ImageId> {
        self.get(&name.join(":"))
    }

    /// Returns a reference to the internal `&ImageMap`
    pub fn image_map(&self) -> &ImageMap {
        &self.image_map
    }
}
