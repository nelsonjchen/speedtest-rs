use crate::speedtest::{SpeedTestConfig, SpeedTestServer};
use std::time::Duration;
use std::sync::{Arc, Mutex, Condvar};
use typed_builder::TypedBuilder;
use crate::distance::EarthLocation;

#[derive(TypedBuilder)]
pub struct Speedtest {
    #[builder(default)]
    pub config: Option<SpeedTestConfig>,
    #[builder(default)]
    source_addr: Option<std::net::IpAddr>,
    #[builder(default_code = "Duration::from_secs(10)")]
    timeout: Duration,
    #[builder(default = false)]
    secure: bool,
    #[builder(default)]
    shutdown_condvar: Option<Mutex<Condvar>>,
    #[builder(default)]
    pub servers: Vec<SpeedTestServer>,
    #[builder(default)]
    pub location: Option<EarthLocation>,
}

pub struct SpeedtestBuilder {

}



#[cfg(test)]
mod tests {
    use crate::speedtest::faithful::Speedtest;

    #[test]
    fn test_test() {
        let speedtest = Speedtest::builder().build();
    }
}