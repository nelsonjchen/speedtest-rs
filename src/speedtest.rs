// use xml::{Element, Parser, ElementBuilder};

pub struct SpeedTestConfig{
    ip: String,
    lat: String,
    lon: String,
    isp: String,
}

#[derive(Debug)]
pub struct ParseError;

pub fn run_speedtest() {
    unimplemented!();
}

pub fn parse_config_xml(config_xml: String) -> Result<SpeedTestConfig, ParseError> {
    // let elem: Element = config_xml.parse().unwrap();
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
        let config = parse_config_xml(include_bytes!("data/speedtest-config.php.xml")).unwrap();
        assert_eq!("174.79.12.26", config.ip);
    }
}
