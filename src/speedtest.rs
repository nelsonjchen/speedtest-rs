// use xml::{Element, Parser, ElementBuilder};
use std::io::Read;
use xml::reader::EventReader;
use xml::reader::events::XmlEvent::*;

#[derive(Debug)]
pub struct ParseError(String);

pub struct SpeedTestConfig {
    ip: String,
    lat: String,
    lon: String,
    isp: String,
}

impl SpeedTestConfig {
    fn new<R: Read>(parser: &mut EventReader<R>) -> Result<SpeedTestConfig, ParseError> {
        let mut ip: Option<String> = None;
        let mut lat: Option<String> = None;
        let mut lon: Option<String> = None;
        let mut isp: Option<String> = None;
        for e in parser.events() {
            match e {
                StartElement { ref name, ref attributes, ..} => {
                    match name.local_name.as_ref() {
                        "client" => {
                            for item in attributes {
                                match item.name.local_name.as_ref() {
                                    "ip" => {
                                        ip = Some(item.value.clone());
                                    },
                                    "lat" => {
                                        lat = Some(item.value.clone());
                                    },
                                    "lon" => {
                                        lon = Some(item.value.clone());
                                    },
                                    "isp" => {
                                        isp = Some(item.value.clone());
                                    },
                                    _ => {},

                                }
                            }
                        break;
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
        match (ip, lat, lon, isp) {
            (Some(ip), Some(lat), Some(lon), Some(isp))=> {
            return Ok(SpeedTestConfig {
                ip: ip,
                lat: lat,
                lon: lon,
                isp: isp,
                })
            },
            _ => {
                return Err(ParseError("something is missing".to_string()));
            }
        }
    }
}

pub fn run_speedtest() {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use std::io::Cursor;
    use std::string::String;
    use std::fs::File;
    use xml::reader::EventReader;

    #[test]
    fn test_parse_config_xml() {
        let mut parser = EventReader::new(include_bytes!("../tests/data/speedtest-config.php.xml") as &[u8]);
        let config = SpeedTestConfig::new(&mut parser).unwrap();
        assert_eq!("174.79.12.26", config.ip);
    }
}
