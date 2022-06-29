use anyhow::Result;
use bevy::{log::LogPlugin, prelude::*};

fn main() -> Result<()> {
    meerkat_common::logging::init_logging()?;
    App::new()
        .add_plugins_with(DefaultPlugins, |group| group.disable::<LogPlugin>())
        .run();

    Ok(())
}
