#![doc = include_str!("../README.md")]

mod api;

mod client;
mod parmeters;

///
pub mod prelude {
    use super::{api, client, parmeters};

    pub use api::*;
    pub use client::*;
    pub use parmeters::*;
}
