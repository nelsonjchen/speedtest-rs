use crate::{error::Error, distance::EarthLocation};
use std::io::Read;
use xml::EventReader;
use xml::reader::XmlEvent::StartElement;


pub struct SpeedTestConfig {
    pub ip: String,
    pub location: EarthLocation,
    pub isp: String,
}

impl SpeedTestConfig {
    pub fn new<R: Read>(parser: EventReader<R>) -> Result<SpeedTestConfig, Error> {
        let mut ip: Option<String> = None;
        let mut lat: Option<f32> = None;
        let mut lon: Option<f32> = None;
        let mut isp: Option<String> = None;
        for event in parser {
            if let Ok(StartElement {
                ref name,
                ref attributes,
                ..
            }) = event
            {
                if name.local_name == "client" {
                    for attribute in attributes {
                        match attribute.name.local_name.as_ref() {
                            "ip" => {
                                ip = Some(attribute.value.clone());
                            }
                            "lon" => lon = Some(attribute.value.parse::<f32>()?),
                            "lat" => lat = Some(attribute.value.parse::<f32>()?),
                            "isp" => {
                                isp = Some(attribute.value.clone());
                            }
                            _ => {}
                        }
                    }
                    break;
                }
            }
        }
        if let (Some(ip), Some(lat), Some(lon), Some(isp)) = (ip, lat, lon, isp) {
            Ok(SpeedTestConfig {
                ip,
                location: EarthLocation {
                    latitude: lat,
                    longitude: lon,
                },
                isp,
            })
        } else {
            Err(Error::ConfigParseError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xml::EventReader;
    #[test]
    fn test_parse_config_xml() {
        let parser = EventReader::new(include_bytes!("../tests/config/config.php.xml") as &[u8]);
        let config = SpeedTestConfig::new(parser).unwrap();
        assert_eq!("174.79.12.26", config.ip);
        assert_eq!(
            EarthLocation {
                latitude: 32.9954,
                longitude: -117.0753,
            },
            config.location
        );
        assert_eq!("Cox Communications", config.isp);
    }
}