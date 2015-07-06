mod speedtest;
mod cheap_distance;

extern crate xml;
extern crate time;

#[allow(dead_code)]
fn main() {
    speedtest::run_speedtest();
}
