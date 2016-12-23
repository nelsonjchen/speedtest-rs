#[macro_use]
extern crate log;

extern crate xml;
extern crate time;
extern crate crypto;
extern crate reqwest;
extern crate url;

pub mod speedtest;
pub mod distance;
pub mod error;

pub use self::error::{Result, Error};
