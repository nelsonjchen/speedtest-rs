// use xml::{Element, Parser, ElementBuilder};
use std::io::Read;

#[derive(Debug)]
pub struct ParseError(String);

pub struct SpeedTestConfig {
    ip: String,
    lat: String,
    lon: String,
    isp: String,
}

impl SpeedTestConfig {
    fn new<R: Read(parser: &mut EventReader<R>) -> Result<SpeedTestConfig, ParseError> {
        unimplemented!();
    }
}

pub fn run_speedtest() {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use std::string::String;
    use std::fs::File;

    #[test]
    fn test_parse_config_xml() {
        unimplemented!();
        let config_xml = include_bytes!("../tests/data/speedtest-config.php.xml");
        let config = parse_config_xml(config_xml).unwrap();
        assert_eq!("174.79.12.26", config.ip);
    }
}
