#![doc(
    html_logo_url = "https://github.com/THeK3nger/movingai-rust/blob/37ad04b72a2e9e8fb7f794c7f1be176fee99b67e/assets/ma.png",
    html_favicon_url = "https://github.com/THeK3nger/movingai-rust/blob/37ad04b72a2e9e8fb7f794c7f1be176fee99b67e/assets/ma.png"
)]
#![deny(missing_docs)]

//!
//! The MovingAI Benchmark Parser
//!
//! # Overview
//!
//! Things.

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

/// Contains all the parser functions.
pub mod parser;

mod map2d;

/// Contains data structure for 2D MovingAI maps.
pub use map2d::*;
