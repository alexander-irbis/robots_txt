#![cfg_attr(feature = "clippy", allow(items_after_statements))]
#![cfg_attr(feature = "clippy", allow(match_bool))]

#![cfg_attr(feature = "release", deny(warnings))]


//! # Standard
//!
//! [A Standard for Robot Exclusion](http://www.robotstxt.org/orig.html)
//!
//! * User-agent
//! * Disallow
//!
//! # Additions
//!
//! * Allow
//! * Crawl-delay
//! * Request-rate
//! * Sitemap
//! * Host


pub extern crate url;

pub mod prelude;

pub mod builder;
pub use builder::*;

pub mod matcher;
pub use matcher::*;

pub mod parts;
pub use parts::*;

pub mod parse;
pub use parse::*;

pub mod render;
pub use render::*;


