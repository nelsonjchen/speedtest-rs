// use xml::{Element, Parser, ElementBuilder};

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
    // let elem: Element = config_xml.parse().unwrap();
    SpeedTestConfig{
        ip: "127.0.0.1".to_string(),
        lat: "0.0".to_string(),
        lon: "0.0".to_string(),
        isp: "Nipple Massage Cable".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use std::string::String;
    use std::fs::File;

    #[test]
    fn test_parse_config_xml() {
        let mut config_xml_string = String::new();
        let _ = File::open("data/speedtest-config.php.xml").
            unwrap().read_to_string(&mut config_xml_string);
        parse_config_xml(config_xml_string);
    }
}
