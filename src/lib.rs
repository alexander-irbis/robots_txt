#![allow(clippy::items_after_statements)]
#![allow(clippy::match_bool)]
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

pub mod builder;
pub mod matcher;
pub mod parse;
pub mod parts;
pub mod render;

pub use crate::{builder::*, matcher::*, parse::*, parts::*, render::*};
