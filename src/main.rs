use anyhow::Result;

fn main() -> Result<()> {
    Ok(lib_driver::init()?)
}
