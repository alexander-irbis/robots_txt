#![cfg_attr(feature = "clippy", allow(items_after_statements))]
#![cfg_attr(feature = "clippy", allow(match_bool))]

#![deny(warnings)]


pub extern crate url;

pub mod prelude;

pub mod parts;
pub use parts::*;

pub mod render;
pub use render::*;
