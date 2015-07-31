mod speedtest;
mod cheap_distance;

extern crate hyper;
extern crate xml;
extern crate time;

#[macro_use]
extern crate log;
extern crate env_logger;

#[allow(dead_code)]
fn main() {
    speedtest::run_speedtest();
}
