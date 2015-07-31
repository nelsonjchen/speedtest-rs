// use xml::{Element, Parser, ElementBuilder};
use std::io::Read;
use std::cmp::Ordering::Less;
use hyper::Client;
use hyper::header::{Connection, UserAgent};
use xml::reader::EventReader;
use xml::reader::events::XmlEvent::*;
use ::cheap_distance::{EarthLocation, compute_distance};

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
        for event in parser.events() {
            match event {
                StartElement { ref name, ref attributes, ..} => {
                    match name.local_name.as_ref() {
                        "client" => {
                            for attribute in attributes {
                                match attribute.name.local_name.as_ref() {
                                    "ip" => {
                                        ip = Some(attribute.value.clone());
                                    },
                                    "lat" => {
                                        match attribute.value.parse::<f32>() {
                                            Ok(_) => {
                                                lat = Some(attribute.value.clone());
                                            },
                                            _ => {
                                                lat = None;
                                            }
                                        }
                                    },
                                    "lon" => {
                                        match attribute.value.parse::<f32>() {
                                            Ok(_) => {
                                                lon = Some(attribute.value.clone());
                                            },
                                            _ => {
                                                lon = None;
                                            }
                                        }
                                    },
                                    "isp" => {
                                        isp = Some(attribute.value.clone());
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
                return Err(ParseError("Configuration is invalid".to_string()));
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct SpeedTestServer {
    pub country: String,
    host: String,
    id: String,
    lat: String,
    lon: String,
    name: String,
    sponsor: String,
    url: String,
    url2: String,
}

pub struct SpeedTestServersConfig {
    servers: Vec<SpeedTestServer>,
}


impl SpeedTestServersConfig {
    fn new<R: Read>(parser: &mut EventReader<R>) -> Result<SpeedTestServersConfig, ParseError> {
        let mut servers: Vec<SpeedTestServer> = Vec::new();

        for event in parser.events(){
            match event {
                StartElement { ref name, ref attributes, ..} => {
                    match name.local_name.as_ref() {
                        "server" => {
                            let mut country: Option<String> = None;
                            let mut host: Option<String> = None;
                            let mut id: Option<String> = None;
                            let mut lat: Option<String> = None;
                            let mut lon: Option<String> = None;
                            let mut name: Option<String> = None;
                            let mut sponsor: Option<String> = None;
                            let mut url: Option<String> = None;
                            let mut url2: Option<String> = None;
                            for attribute in attributes {
                                match attribute.name.local_name.as_ref() {
                                    "country" => {
                                        country = Some(attribute.value.clone());
                                    },
                                    "host" => {
                                        host = Some(attribute.value.clone());
                                    },
                                    "id" => {
                                        id = Some(attribute.value.clone());
                                    },
                                    "lat" => {
                                        lat = Some(attribute.value.clone());
                                    },
                                    "lon" => {
                                        lon = Some(attribute.value.clone());
                                    },
                                    "name" => {
                                        name = Some(attribute.value.clone());
                                    },
                                    "sponsor" => {
                                        sponsor = Some(attribute.value.clone());
                                    },
                                    "url" => {
                                        url = Some(attribute.value.clone());
                                    },
                                    "url2" => {
                                        url2 = Some(attribute.value.clone());
                                    },
                                    _ => {
                                        // eh?
                                    }
                                }
                            }
                            match (
                                country,
                                host,
                                id,
                                lat,
                                lon,
                                name,
                                sponsor,
                                url,
                                url2
                                ) {
                                    (
                                        Some(country),
                                        Some(host),
                                        Some(id),
                                        Some(lat),
                                        Some(lon),
                                        Some(name),
                                        Some(sponsor),
                                        Some(url),
                                        Some(url2)
                                        ) => {
                                            servers.push(
                                                SpeedTestServer{
                                                    country: country,
                                                    host: host,
                                                    id: id,
                                                    lat: lat,
                                                    lon: lon,
                                                    name: name,
                                                    sponsor: sponsor,
                                                    url: url,
                                                    url2: url2,
                                                }
                                            );
                                        }
                                _ => {
                                    // eh
                                }
                            }
                        }
                        _ => {
                            // I don't care about other tags.
                        }
                    }
                }
                _ => {
                    // not using other parts of the xml library right now.
                }
            }
        }
        return Ok(SpeedTestServersConfig{
            servers: servers
        })
    }

    pub fn closest_server(&self, config: &SpeedTestConfig) -> Option<&SpeedTestServer> {
        let location = EarthLocation{
            latitude: config.lat.parse::<f32>().unwrap(),
            longitude: config.lon.parse::<f32>().unwrap(),
        };
        self.servers.iter().fold((None, 0.0), |acc, server|
            {
                let server_location = EarthLocation {
                    latitude: server.lat.parse::<f32>().unwrap(),
                    longitude: server.lon.parse::<f32>().unwrap(),
                };
                match acc {
                    (None, _) => {
                        (Some(server), compute_distance(&location, &server_location))
                    },
                    (Some(last_server), last_distance) => {
                        let distance = compute_distance(&location, &server_location);
                        if last_distance.abs() > distance.abs() {
                            // This lastest server is closer
                            (Some(server), distance)
                        } else {
                            // Last server was still closer.
                            (Some(last_server), last_distance)
                        }
                    }
                }
            }
        ).0
    }

    pub fn servers_sorted_by_distance(&self, config: &SpeedTestConfig) -> Option<Vec<SpeedTestServer>> {
        let location = EarthLocation{
            latitude: config.lat.parse::<f32>().unwrap(),
            longitude: config.lon.parse::<f32>().unwrap(),
        };
        let mut sorted_servers = self.servers.clone();
        sorted_servers.sort_by(|a, b| {
                let a_location = EarthLocation {
                    latitude: a.lat.parse::<f32>().unwrap(),
                    longitude: a.lon.parse::<f32>().unwrap(),
                };
                let b_location = EarthLocation {
                    latitude: b.lat.parse::<f32>().unwrap(),
                    longitude: b.lon.parse::<f32>().unwrap(),
                };
                let a_distance = compute_distance(&location, &a_location);
                let b_distance = compute_distance(&location, &b_location);
                a_distance.partial_cmp(&b_distance).unwrap_or(Less)
        });
        Some(sorted_servers)
    }
}

pub fn run_speedtest() {
    info!("Downloading Configuration");
    let mut client = Client::new();
    // Creating an outgoing request.
    let mut config_res = client.get("http://www.speedtest.net/speedtest-config.php")
        // set a header
        .header(Connection::close())
        .header(UserAgent("hyper/speedtest-rust 0.01".to_owned()))
        // let 'er go!
        .send().unwrap();
    let mut config_body = String::new();
    config_res.read_to_string(&mut config_body).unwrap();
    info!("Downloaded Configuration");

    info!("Parsing Configuration");
    let mut config_parser = EventReader::new(config_body.as_bytes());
    let spt_config = SpeedTestConfig::new(&mut config_parser).unwrap();
    info!("Parsed Configuration");

    info!("Download Server List");
    let mut server_res = client.get("http://www.speedtest.net/speedtest-servers-static.php")
        // set a header
        .header(Connection::close())
        .header(UserAgent("hyper/speedtest-rust 0.01".to_owned()))
        // let 'er go!
        .send().unwrap();
    let mut server_body = String::new();
    server_res.read_to_string(&mut server_body).unwrap();
    info!("Downloaded Server List");

    info!("Parsing Server List");
    let mut server_parser = EventReader::new(
        include_bytes!("../tests/config/stripped-servers-static.php.xml") as &[u8]
    );
    let spt_server_config = SpeedTestServersConfig::new(&mut server_parser).unwrap();
    info!("Parsed Server List");

    info!("Determining Closest Server");
    let closest_server = spt_server_config.closest_server(&spt_config).unwrap();
    info!("Determined Closest Server is {:?}", closest_server);
    // Test against server
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
        let mut parser = EventReader::new(
            include_bytes!("../tests/config/config.php.xml") as &[u8]
        );
        let config = SpeedTestConfig::new(&mut parser).unwrap();
        assert_eq!("174.79.12.26", config.ip);
        assert_eq!("32.9954", config.lat);
        assert_eq!("-117.0753", config.lon);
        assert_eq!("Cox Communications", config.isp);
    }

    #[test]
    fn test_parse_speedtest_servers_xml() {
        let mut parser = EventReader::new(
            include_bytes!("../tests/config/stripped-servers-static.php.xml") as &[u8]
        );
        let spt_server_config = SpeedTestServersConfig::new(&mut parser).unwrap();
        assert!(spt_server_config.servers.len() > 5);
        let server = spt_server_config.servers.get(1).unwrap();
        assert!(server.url2.len() > 0);
        assert!(server.country.len() > 0);
    }

    #[test]
    fn test_fastest_server() {
        let spt_config = SpeedTestConfig{
            ip: "127.0.0.1".to_string(),
            lat: "32.9954".to_string(),
            lon: "-117.0753".to_string(),
            isp: "xxxfinity".to_string(),
        };
        let mut parser = EventReader::new(
            include_bytes!("../tests/config/geo-test-servers-static.php.xml") as &[u8]
        );
        let spt_server_config = SpeedTestServersConfig::new(&mut parser).unwrap();
        let closest_server = spt_server_config.closest_server(&spt_config).unwrap();
        assert_eq!("Los Angeles, CA", closest_server.name);
    }
}
