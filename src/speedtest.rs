pub struct SpeedTestConfig{
    ip: String,
    lat: String,
    lon: String,
    isp: String,
}


pub fn run_speedtest() {
    println!("TODO");
}

pub fn parse_config_xml(config_xml: String) -> SpeedTestConfig {
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
    use std::io::prelude::*;
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn test_parse_config_xml() {
        let mut f = try!(File::open("foo.txt"));
    }
}
