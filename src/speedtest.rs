struct SpeedTestConfig;


pub fn speedtest() {
    println!("Starting Speed Test");
    get_config();
}

fn get_config() -> SpeedTestConfig {
    SpeedTestConfig
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_five_closest_servers() {
    }
}
