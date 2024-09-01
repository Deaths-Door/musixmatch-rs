#![doc = include_str!("../README.md")]

#[cfg_attr(not(test), forbid(unused_mut))]
mod api;
mod client;
pub mod prelude {
    use super::{api, client};

    pub use api::*;
    pub use client::*;
}
