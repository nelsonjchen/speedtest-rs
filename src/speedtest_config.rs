#![allow(dead_code)]

use crate::{distance::EarthLocation, error::SpeedTestError};
use std::{net::Ipv4Addr, time::Duration};

pub struct SpeedTestClientConfig {
    pub ip: Ipv4Addr,
    pub isp: String,
}

impl Default for SpeedTestClientConfig {
    fn default() -> Self {
        SpeedTestClientConfig {
            ip: Ipv4Addr::new(127, 0, 0, 1),
            isp: String::default(),
        }
    }
}

#[derive(Default)]
pub struct SpeedTestSizeConfig {
    pub upload: Vec<usize>,
    pub download: Vec<usize>,
}

#[derive(Default)]
pub struct SpeedTestCountsConfig {
    pub upload: usize,
    pub download: usize,
}

#[derive(Default)]
pub struct SpeedTestThreadsConfig {
    pub upload: usize,
    pub download: usize,
}

pub struct SpeedTestLengthConfig {
    pub upload: Duration,
    pub download: Duration,
}

impl Default for SpeedTestLengthConfig {
    fn default() -> Self {
        SpeedTestLengthConfig {
            upload: Duration::from_secs(10),
            download: Duration::from_secs(10),
        }
    }
}

#[derive(Default)]
pub struct SpeedTestConfig {
    pub client: SpeedTestClientConfig,
    pub ignore_servers: Vec<u32>,
    pub sizes: SpeedTestSizeConfig,
    pub counts: SpeedTestCountsConfig,
    pub threads: SpeedTestThreadsConfig,
    pub length: SpeedTestLengthConfig,
    pub upload_max: usize,
    pub location: EarthLocation,
}

impl SpeedTestConfig {
    pub fn parse(config_xml: &str) -> Result<SpeedTestConfig, SpeedTestError> {
        let document = roxmltree::Document::parse(config_xml)?;

        let server_config_node = document
            .descendants()
            .find(|n| n.has_tag_name("server-config"))
            .ok_or(SpeedTestError::ConfigParseError)?;
        let download_node = document
            .descendants()
            .find(|n| n.has_tag_name("download"))
            .ok_or(SpeedTestError::ConfigParseError)?;
        let upload_node = document
            .descendants()
            .find(|n| n.has_tag_name("upload"))
            .ok_or(SpeedTestError::ConfigParseError)?;
        let client_node = document
            .descendants()
            .find(|n| n.has_tag_name("client"))
            .ok_or(SpeedTestError::ConfigParseError)?;

        let ignore_servers: Vec<u32> = server_config_node
            .attribute("ignoreids")
            .ok_or(SpeedTestError::ConfigParseError)?
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()?;

        let ratio = upload_node
            .attribute("ratio")
            .ok_or(SpeedTestError::ConfigParseError)?
            .parse::<usize>()?;

        let upload_max = upload_node
            .attribute("maxchunkcount")
            .ok_or(SpeedTestError::ConfigParseError)?
            .parse::<usize>()?;

        let up_sizes = [32768usize, 65536, 131072, 262144, 524288, 1048576, 7340032];

        let sizes = SpeedTestSizeConfig {
            upload: up_sizes
                .get(ratio - 1..)
                .ok_or(SpeedTestError::ConfigParseError)?
                .to_vec(),
            download: vec![350usize, 500, 750, 1000, 1500, 2000, 2500, 3000, 3500, 4000],
        };

        let size_count = sizes.upload.len();

        let upload_count = (upload_max as f32 / size_count as f32).ceil() as usize;

        let counts = SpeedTestCountsConfig {
            upload: upload_count,
            download: download_node
                .attribute("threadsperurl")
                .ok_or(SpeedTestError::ConfigParseError)?
                .parse::<usize>()?,
        };

        let threads = SpeedTestThreadsConfig {
            upload: upload_node
                .attribute("threads")
                .ok_or(SpeedTestError::ConfigParseError)?
                .parse::<usize>()?,
            download: server_config_node
                .attribute("threadcount")
                .ok_or(SpeedTestError::ConfigParseError)?
                .parse::<usize>()?
                * 2,
        };

        let length = SpeedTestLengthConfig {
            upload: upload_node
                .attribute("testlength")
                .ok_or(SpeedTestError::ConfigParseError)?
                .parse::<u64>()
                .map(Duration::from_secs)?,
            download: download_node
                .attribute("testlength")
                .ok_or(SpeedTestError::ConfigParseError)?
                .parse::<u64>()
                .map(Duration::from_secs)?,
        };

        let client = SpeedTestClientConfig {
            ip: client_node
                .attribute("ip")
                .ok_or(SpeedTestError::ConfigParseError)?
                .parse()?,
            isp: client_node
                .attribute("isp")
                .ok_or(SpeedTestError::ConfigParseError)?
                .to_string(),
        };

        Ok(SpeedTestConfig {
            client,
            ignore_servers,
            sizes,
            counts,
            threads,
            length,
            upload_max,
            location: EarthLocation {
                latitude: client_node
                    .attribute("lat")
                    .ok_or(SpeedTestError::ConfigParseError)?
                    .parse()?,
                longitude: client_node
                    .attribute("lon")
                    .ok_or(SpeedTestError::ConfigParseError)?
                    .parse()?,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config_xml() {
        let config =
            SpeedTestConfig::parse(include_str!("../tests/config/config.php.xml")).unwrap();
        assert_eq!("174.79.12.26", config.client.ip.to_string());
        assert_eq!(
            EarthLocation {
                latitude: 32.9954,
                longitude: -117.0753,
            },
            config.location
        );
        assert_eq!("Cox Communications", config.client.isp);
    }

    #[test]
    fn test_parse_config_xml_83() {
        let config =
            SpeedTestConfig::parse(include_str!("../tests/config/2021-07-speedtest-config.xml"))
                .unwrap();
        assert_eq!("Cox Communications", config.client.isp);
    }
}
