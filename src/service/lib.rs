#![warn(clippy::all, clippy::pedantic)]

#[macro_use]
extern crate log;

pub use self::{poller::*, services::*};

mod poller;
mod services;
