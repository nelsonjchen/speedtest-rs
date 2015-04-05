struct SpeedTestConfig{
    ip: String,
    lat: String,
    lon: String,
    isp: String,
}


pub fn speedtest() {
    println!("Starting Speed Test");
    get_config();
}

fn get_config() -> SpeedTestConfig {
    SpeedTestConfig{
        ip: "127.0.0.1".to_string(),
        lat: "37.4192".to_string(),
        lon: "-122.0574".to_string(),
        isp: "Time Warner Cable".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_five_closest_servers() {
    }
}
