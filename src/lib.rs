#[macro_use]
extern crate log;

extern crate hyper;
extern crate xml;
extern crate time;
extern crate crypto;

pub mod speedtest;
pub mod distance;
pub mod error;

pub use self::error::{Result, Error};
