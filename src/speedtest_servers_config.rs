use crate::distance::{self, EarthLocation};
use crate::{error::SpeedTestError, speedtest::SpeedTestServer, speedtest_config::SpeedTestConfig};
use std::cmp::Ordering::Less;

pub struct SpeedTestServersConfig {
    pub servers: Vec<SpeedTestServer>,
}

impl SpeedTestServersConfig {
    pub fn parse_with_config(
        server_config_xml: &str,
        config: &SpeedTestConfig,
    ) -> Result<SpeedTestServersConfig, SpeedTestError> {
        let document = roxmltree::Document::parse(server_config_xml)?;
        let servers = document
            .descendants()
            .filter(|node| node.tag_name().name() == "server")
            .map::<Result<_, SpeedTestError>, _>(|n| {
                let location = EarthLocation {
                    latitude: n
                        .attribute("lat")
                        .ok_or(SpeedTestError::ServerParseError)?
                        .parse()?,
                    longitude: n
                        .attribute("lon")
                        .ok_or(SpeedTestError::ServerParseError)?
                        .parse()?,
                };
                Ok(SpeedTestServer {
                    country: n
                        .attribute("country")
                        .ok_or(SpeedTestError::ServerParseError)?
                        .to_string(),
                    host: n
                        .attribute("host")
                        .ok_or(SpeedTestError::ServerParseError)?
                        .to_string(),
                    id: n
                        .attribute("id")
                        .ok_or(SpeedTestError::ServerParseError)?
                        .parse()?,
                    location: location.clone(),
                    distance: Some(distance::compute_distance(&config.location, &location)),
                    name: n
                        .attribute("name")
                        .ok_or(SpeedTestError::ServerParseError)?
                        .to_string(),
                    sponsor: n
                        .attribute("sponsor")
                        .ok_or(SpeedTestError::ServerParseError)?
                        .to_string(),
                    url: n
                        .attribute("url")
                        .ok_or(SpeedTestError::ServerParseError)?
                        .to_string(),
                })
            })
            .filter_map(Result::ok)
            .filter(|server| !config.ignore_servers.contains(&server.id))
            .collect();
        Ok(SpeedTestServersConfig { servers })
    }

    pub fn servers_sorted_by_distance(&self, config: &SpeedTestConfig) -> Vec<SpeedTestServer> {
        let location = &config.location;
        let mut sorted_servers = self.servers.clone();
        sorted_servers.sort_by(|a, b| {
            let a_distance = distance::compute_distance(location, &a.location);
            let b_distance = distance::compute_distance(location, &b.location);
            a_distance.partial_cmp(&b_distance).unwrap_or(Less)
        });
        sorted_servers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::speedtest_config::*;

    fn sample_spt_config() -> SpeedTestConfig {
        // Somewhere in Los Angeles
        SpeedTestConfig {
            location: EarthLocation {
                latitude: 32.9954,
                longitude: -117.0753,
            },
            ..SpeedTestConfig::default()
        }
    }

    #[test]
    fn test_parse_speedtest_servers_xml() {
        let spt_config = sample_spt_config();
        let config_str = include_str!("../tests/config/geo-test-servers-static.php.xml");

        let server_config =
            SpeedTestServersConfig::parse_with_config(config_str, &spt_config).unwrap();
        assert!(server_config.servers.len() > 5);
        let server = server_config.servers.get(1).unwrap();
        assert!(!server.url.is_empty());
        assert!(!server.country.is_empty());
    }

    #[test]
    fn test_parse_speedtest_servers_xml_with_ignore_servers() {
        let spt_config = SpeedTestConfig {
            ignore_servers: vec![5905],
            ..sample_spt_config()
        };
        let config_str = include_str!("../tests/config/geo-test-servers-static.php.xml");

        let server_config =
            SpeedTestServersConfig::parse_with_config(config_str, &spt_config).unwrap();
        assert!(server_config.servers.iter().all(|s| s.id != 5905));
        assert!(server_config.servers.len() > 5);
        let server = server_config.servers.get(1).unwrap();
        assert!(!server.url.is_empty());
        assert!(!server.country.is_empty());
    }

    #[test]
    fn test_fastest_server() {
        let spt_config = sample_spt_config();
        let config_str = include_str!("../tests/config/geo-test-servers-static.php.xml");

        let config = SpeedTestServersConfig::parse_with_config(config_str, &spt_config).unwrap();
        let closest_server = &config.servers_sorted_by_distance(&spt_config)[0];
        assert_eq!("Los Angeles, CA", closest_server.name);
    }
}
