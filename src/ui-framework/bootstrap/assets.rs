use anyhow::Result;
use log::debug;

use std::path::PathBuf;

/// Finds and returns the assets folder
pub fn locate_assets() -> Result<PathBuf> {
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets")?;

    debug!("Assets located at: {}", assets.display());

    Ok(assets)
}
