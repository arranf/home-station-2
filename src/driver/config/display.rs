use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// The width the screen should be rendered in pixels
    pub width: u32,
    /// The height the screen should be rendered in pixels
    pub height: u32,
}
