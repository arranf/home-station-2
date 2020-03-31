#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

#[macro_use]
extern crate conrod_core;

use anyhow::Result;

use lib_ui_framework::System;
pub use state::State;

use self::pages::*;

mod pages;
mod state;

/// Sets up the state, router, and initialises the first page.
pub fn start(mut system: System<State>, state: State) -> Result<()> {
    system.setup_state(move |sys_state| {
        *sys_state = Some(state);
    });

    system.setup_router(move |router| {
        router.add("home", HomeRoute);
    });

    system.start("home")?;
    Ok(())
}
