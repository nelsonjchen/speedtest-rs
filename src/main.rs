mod speedtest;
mod distance;

extern crate hyper;
extern crate xml;
extern crate time;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate clap;

#[allow(dead_code)]
fn main() {
    env_logger::init().unwrap();
    info!("Starting up...");
    speedtest::run_speedtest();
}
