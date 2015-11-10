#[macro_use]
extern crate log;

extern crate hyper;
extern crate xml;
extern crate time;

pub mod speedtest;
pub mod distance;
pub mod error;

pub use self::error::{Result, Error};
