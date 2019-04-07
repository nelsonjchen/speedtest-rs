use crate::speedtest::{SpeedTestConfig, SpeedTestServer};
use time::Duration;
use std::sync::{Arc, Mutex, Condvar};

#[derive(Default)]
pub struct Speedtest {
    pub config: Option<SpeedTestConfig>,
    source_addr: Option<std::net::IpAddr>,
    timeout: Option<Duration>,
    secure: bool,
    shutdown_condvar: Option<Mutex<Condvar>>,
    servers: Vec<SpeedTestServer>,
    client: Option<>
}

impl Speedtest {
    pub fn new() -> Self
    {
        Self {
            timeout: Some(Duration::seconds(10)),
            ..Self::default()
        }
    }

    pub fn use_config(self, config: SpeedTestConfig) -> Self {
        Self {
            config: Some(config),
            ..self
        }
    }

    pub fn get_config(self) -> Self {
        Self {
            config: None,
            ..self
        }
    }

    pub fn execute(self) -> Self {
        Self {
            self
        }
    }

}


#[cfg(test)]
mod tests {
    use crate::speedtest::faithful::Speedtest;

    #[test]
    fn test_test() {
        let speedtest = Speedtest::new().get_config();
    }

}