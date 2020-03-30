use std::cmp::Ordering::Less;
use std::io::Read;
use std::path::Path;
use std::sync::mpsc::sync_channel;
use std::sync::{Arc, RwLock};
use std::thread;

use log::{debug, info};
use reqwest::header::{CONNECTION, CONTENT_TYPE, REFERER, USER_AGENT};
use reqwest::{Client, Response};
use time::{now, Duration};
use xml::reader::EventReader;
use xml::reader::XmlEvent::StartElement;

use crate::distance::{self, EarthLocation};
use crate::error::Error;

const ST_USER_AGENT: &str = concat!("reqwest/speedtest-rs ", env!("CARGO_PKG_VERSION"));

pub struct SpeedTestConfig {
    pub ip: String,
    location: EarthLocation,
    pub isp: String,
}

impl SpeedTestConfig {
    fn new<R: Read>(parser: EventReader<R>) -> Result<SpeedTestConfig, Error> {
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
                            "lat" => lat = attribute.value.parse::<f32>().ok(),
                            "lon" => lon = attribute.value.parse::<f32>().ok(),
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

#[derive(Clone, Debug)]
pub struct SpeedTestServer {
    pub country: String,
    pub host: String,
    pub id: u32,
    pub location: EarthLocation,
    pub distance: Option<f32>,
    pub name: String,
    pub sponsor: String,
    pub url: String,
}

pub struct SpeedTestServersConfig {
    servers: Vec<SpeedTestServer>,
}

impl SpeedTestServersConfig {
    fn new<R: Read>(parser: EventReader<R>) -> Result<SpeedTestServersConfig, Error> {
        SpeedTestServersConfig::with_config(parser, None)
    }

    fn with_config<R: Read>(
        parser: EventReader<R>,
        config: Option<&SpeedTestConfig>,
    ) -> Result<SpeedTestServersConfig, Error> {
        let mut servers: Vec<SpeedTestServer> = Vec::new();

        for event in parser {
            if let Ok(StartElement {
                ref name,
                ref attributes,
                ..
            }) = event
            {
                if name.local_name == "server" {
                    let mut country: Option<String> = None;
                    let mut host: Option<String> = None;
                    let mut id: Option<u32> = None;
                    let mut lat: Option<f32> = None;
                    let mut lon: Option<f32> = None;
                    let mut name: Option<String> = None;
                    let mut sponsor: Option<String> = None;
                    let mut url: Option<String> = None;
                    for attribute in attributes {
                        match attribute.name.local_name.as_ref() {
                            "country" => {
                                country = Some(attribute.value.clone());
                            }
                            "host" => {
                                host = Some(attribute.value.clone());
                            }
                            "id" => id = attribute.value.parse::<u32>().ok(),
                            "lat" => lat = attribute.value.parse::<f32>().ok(),
                            "lon" => lon = attribute.value.parse::<f32>().ok(),
                            "name" => {
                                name = Some(attribute.value.clone());
                            }
                            "sponsor" => {
                                sponsor = Some(attribute.value.clone());
                            }
                            "url" => {
                                url = Some(attribute.value.clone());
                            }
                            _ => {}
                        }
                    }
                    if let (
                        Some(country),
                        Some(host),
                        Some(id),
                        Some(lat),
                        Some(lon),
                        Some(name),
                        Some(sponsor),
                        Some(url),
                    ) = (country, host, id, lat, lon, name, sponsor, url)
                    {
                        let location = EarthLocation {
                            latitude: lat,
                            longitude: lon,
                        };
                        let distance = config
                            .map(|config| distance::compute_distance(&config.location, &location));
                        let server = SpeedTestServer {
                            country,
                            host,
                            id,
                            location,
                            distance,
                            name,
                            sponsor,
                            url,
                        };
                        servers.push(server);
                    }
                }
            }
        }
        Ok(SpeedTestServersConfig { servers })
    }

    pub fn servers_sorted_by_distance(&self, config: &SpeedTestConfig) -> Vec<SpeedTestServer> {
        let location = &config.location;
        let mut sorted_servers = self.servers.clone();
        sorted_servers.sort_by(|a, b| {
            let a_distance = distance::compute_distance(&location, &a.location);
            let b_distance = distance::compute_distance(&location, &b.location);
            a_distance.partial_cmp(&b_distance).unwrap_or(Less)
        });
        sorted_servers
    }
}

pub fn download_configuration() -> Result<Response, Error> {
    info!("Downloading Configuration from speedtest.net");

    #[cfg(not(test))]
    let url = "http://www.speedtest.net/speedtest-config.php";
    #[cfg(test)]
    let url = &format!("{}/speedtest-config.php", &mockito::server_url());

    let client = Client::new();
    // Creating an outgoing request.
    let res = client
        .get(url)
        .header(CONNECTION, "close")
        .header(USER_AGENT, ST_USER_AGENT.to_owned())
        .send()?;
    info!("Downloaded Configuration from speedtest.net");
    Ok(res)
}

pub fn get_configuration() -> Result<SpeedTestConfig, Error> {
    let config_body = download_configuration()?;
    info!("Parsing Configuration");
    let config_parser = EventReader::new(config_body);
    let spt_config = SpeedTestConfig::new(config_parser);
    info!("Parsed Configuration");
    spt_config
}

pub fn download_server_list() -> Result<Response, Error> {
    info!("Download Server List");
    #[cfg(not(test))]
    let url = "http://www.speedtest.net/speedtest-servers.php";
    #[cfg(test)]
    let url = &format!("{}/speedtest-servers.php", &mockito::server_url());

    let client = Client::new();
    let server_res = client
        .get(url)
        .header(CONNECTION, "close")
        .header(USER_AGENT, ST_USER_AGENT)
        .send()?;
    info!("Downloaded Server List");
    Ok(server_res)
}

pub fn get_server_list_with_config(
    config: Option<&SpeedTestConfig>,
) -> Result<SpeedTestServersConfig, Error> {
    let config_body = download_server_list()?;
    info!("Parsing Server List");
    let config_parser = EventReader::new(config_body);
    let spt_config = match config {
        Some(config) => SpeedTestServersConfig::with_config(config_parser, Some(config)),
        None => SpeedTestServersConfig::new(config_parser),
    };
    info!("Parsed Server List");
    spt_config
}

#[derive(Debug)]
pub struct SpeedTestLatencyTestResult<'a> {
    pub server: &'a SpeedTestServer,
    pub latency: Duration,
}

pub fn get_best_server_based_on_latency(
    servers: &[SpeedTestServer],
) -> Result<SpeedTestLatencyTestResult, Error> {
    info!("Testing for fastest server");
    let client = Client::new();
    let mut fastest_server = None;
    let mut fastest_latency = Duration::max_value();
    'server_loop: for server in servers {
        let path = Path::new(&server.url);
        let latency_path = format!(
            "{}/latency.txt",
            path.parent()
                .ok_or(Error::LatencyTestInvalidPath)?
                .display()
        );
        info!("Downloading: {:?}", latency_path);
        let mut latency_measurements = vec![];
        for _ in 0..3 {
            let start_time = now();
            let res = client
                .get(&latency_path)
                .header(CONNECTION, "close")
                .header(USER_AGENT, ST_USER_AGENT.to_owned())
                .send();
            if res.is_err() {
                continue 'server_loop;
            }
            res?.bytes().last();
            let latency_measurement = now() - start_time;
            info!("Sampled {} ms", latency_measurement.num_milliseconds());
            latency_measurements.push(latency_measurement);
        }
        // Divide by the double to get the non-RTT time but the trip time.
        // NOT PING or RTT
        // https://github.com/sivel/speedtest-cli/pull/199
        let latency = latency_measurements
            .iter()
            .fold(Duration::zero(), |a, &i| a + i)
            / ((latency_measurements.iter().count() as i32) * 2);
        info!("Trip calculated to {} ms", latency.num_milliseconds());

        if latency < fastest_latency {
            fastest_server = Some(server);
            fastest_latency = latency;
        }
    }
    info!(
        "Fastest Server @ {}ms : {:?}",
        fastest_latency.num_milliseconds(),
        fastest_server
    );
    Ok(SpeedTestLatencyTestResult {
        server: fastest_server.ok_or(Error::LatencyTestClosestError)?,
        latency: fastest_latency,
    })
}

#[derive(Debug)]
pub struct SpeedMeasurement {
    pub size: usize,
    pub duration: Duration,
}

impl SpeedMeasurement {
    pub fn kbps(&self) -> u32 {
        (self.size as u32 * 8) / self.duration.num_milliseconds() as u32
    }

    pub fn bps_f64(&self) -> f64 {
        (self.size as f64 * 8.0) / (self.duration.num_milliseconds() as f64 / (1000.0))
    }
}

pub fn test_download_with_progress<F>(
    server: &SpeedTestServer,
    f: F,
) -> Result<SpeedMeasurement, Error>
where
    F: Fn() -> () + Send + Sync + 'static,
{
    info!("Testing Download speed");
    let root_path = Path::new(&server.url)
        .parent()
        .ok_or(Error::ConfigParseError)?;
    debug!("Root path is: {}", root_path.display());
    let start_time = Arc::new(now());
    let total_size;

    let sizes = [350, 500, 750, 1000, 1500, 2000, 2500, 3000, 3500, 4000];
    let times_to_run_each_file = 4;
    let len_sizes = sizes.len() * times_to_run_each_file;
    let complete = Arc::new(RwLock::new(vec![]));
    let (tx, rx) = sync_channel(6);
    let root_path = root_path.to_path_buf();
    let thread_start_time = start_time.clone();
    let farc = Arc::new(f);
    let prod_thread = thread::spawn(move || {
        for size in &sizes {
            for _ in 0..times_to_run_each_file {
                let size = *size;
                let root_path = root_path.clone();
                let start_time = thread_start_time.clone();
                let farc = farc.clone();
                let thread = thread::spawn(move || {
                    let path = root_path.join(format!("random{0}x{0}.jpg", size));
                    let f = farc.clone();
                    f();
                    if (now() - *start_time) > Duration::seconds(10) {
                        info!("Canceled Downloading {} of {}", size, path.display());
                        return 0;
                    }
                    let client = Client::new();
                    let mut res = client
                        .get(path.to_str().unwrap())
                        .header(CONNECTION, "close")
                        .header(USER_AGENT, ST_USER_AGENT.to_owned())
                        .send()
                        .unwrap();
                    let mut buffer = [0; 10240];
                    let mut size: usize = 0;
                    loop {
                        match res.read(&mut buffer) {
                            Ok(0) => {
                                break;
                            }
                            Ok(n) => size += n,
                            _ => panic!("Something has gone wrong."),
                        }
                    }
                    info!("Done {}, {}", path.display(), size);
                    size
                });
                tx.send(thread).unwrap();
            }
        }
    });

    let cons_complete = complete.clone();

    let cons_thread = thread::spawn(move || {
        while cons_complete.read().unwrap().len() < len_sizes {
            let thread = rx.recv().unwrap();
            let mut complete = (*cons_complete).write().unwrap();
            complete.push(thread.join().unwrap());
        }
    });
    prod_thread.join().unwrap();
    cons_thread.join().unwrap();
    total_size = (*complete).read().unwrap().iter().sum();
    Ok(SpeedMeasurement {
        size: total_size,
        duration: now() - *start_time,
    })
}

pub fn test_upload_with_progress<F>(
    server: &SpeedTestServer,
    f: F,
) -> Result<SpeedMeasurement, Error>
where
    F: Fn() -> () + Send + Sync + 'static,
{
    info!("Testing Upload");
    let upload_path = Path::new(&server.url).to_path_buf();
    let total_size: usize;
    let start_time = Arc::new(now());
    let small_sizes = [250_000; 25];
    let large_sizes = [500_000; 25];
    let sizes = small_sizes
        .iter()
        .chain(large_sizes.iter())
        .cloned()
        .collect::<Vec<usize>>();
    let len_sizes = sizes.len();
    let complete = Arc::new(RwLock::new(vec![]));
    let (tx, rx) = sync_channel(6);

    let thread_start_time = start_time.clone();
    let farc = Arc::new(f);
    let prod_thread = thread::spawn(move || {
        for size in &sizes {
            let size = *size;
            let path = upload_path.to_path_buf().clone();
            let start_time = thread_start_time.clone();
            let farc = farc.clone();
            let thread = thread::spawn(move || {
                info!("Uploading {} to {}", size, path.display());
                let f = farc.clone();
                f();
                if (now() - *start_time) > Duration::seconds(10) {
                    info!("Canceled Uploading {} of {}", size, path.display());
                    return 0;
                }
                let body_loop = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().cycle();
                let client = Client::new();
                let body = format!("content1={}", body_loop.take(size).collect::<String>());
                let mut res = client
                    .post(path.to_str().unwrap())
                    .body(body)
                    .header(CONNECTION, "close")
                    .header(USER_AGENT, ST_USER_AGENT.to_owned())
                    .send()
                    .unwrap();
                let mut buffer = [0; 10240];
                loop {
                    match res.read(&mut buffer) {
                        Ok(0) => {
                            break;
                        }
                        Ok(_) => {}
                        _ => panic!("Something has gone wrong."),
                    }
                }
                info!("Done {}, {}", path.display(), size);
                size
            });
            tx.send(thread).unwrap();
        }
    });

    let cons_complete = complete.clone();

    let cons_thread = thread::spawn(move || {
        while cons_complete.read().unwrap().len() < len_sizes {
            let thread = rx.recv().unwrap();
            let mut complete = (*cons_complete).write().unwrap();
            complete.push(thread.join().unwrap());
        }
    });

    prod_thread.join().unwrap();
    cons_thread.join().unwrap();
    total_size = (*complete).read().unwrap().iter().sum();
    let latency = now() - *start_time;
    info!(
        "It took {} ms to upload {} bytes",
        latency.num_milliseconds(),
        total_size
    );
    info!(
        "{} bytes per second",
        total_size as i64 / (latency.num_milliseconds() / 1000)
    );
    Ok(SpeedMeasurement {
        size: total_size,
        duration: now() - *start_time,
    })
}

#[derive(Debug)]
pub struct SpeedTestResult<'a, 'b, 'c> {
    pub download_measurement: Option<&'a SpeedMeasurement>,
    pub upload_measurement: Option<&'b SpeedMeasurement>,
    pub server: &'c SpeedTestServer,
    pub latency_measurement: &'c SpeedTestLatencyTestResult<'c>,
}

impl<'a, 'b, 'c> SpeedTestResult<'a, 'b, 'c> {
    pub fn hash(&self) -> String {
        let hashed_str = format!(
            "{}-{}-{}-{}",
            self.latency_measurement.latency.num_milliseconds(),
            if let Some(upload_measurement) = self.upload_measurement {
                upload_measurement.kbps()
            } else {
                0
            },
            if let Some(download_measurement) = self.download_measurement {
                download_measurement.kbps()
            } else {
                0
            },
            "297aae72"
        );

        format!("{:x}", md5::compute(hashed_str))
    }
}

pub fn get_share_url(speedtest_result: &SpeedTestResult) -> Result<String, Error> {
    info!("Generating share URL");
    let download = if let Some(download_measurement) = speedtest_result.download_measurement {
        download_measurement.kbps()
    } else {
        0
    };
    info!("Download parameter is {:?}", download);
    let upload = if let Some(upload_measurement) = speedtest_result.upload_measurement {
        upload_measurement.kbps()
    } else {
        0
    };
    info!("Upload parameter is {:?}", upload);
    let server = speedtest_result.server.id;
    info!("Server parameter is {:?}", server);
    let ping = speedtest_result.latency_measurement.latency;
    info!("Ping parameter is {:?}", ping);

    let pairs = [
        (
            "download",
            format!(
                "{}",
                if let Some(download_measurement) = speedtest_result.download_measurement {
                    download_measurement.kbps()
                } else {
                    0
                }
            ),
        ),
        ("ping", format!("{}", ping.num_milliseconds())),
        (
            "upload",
            format!(
                "{}",
                if let Some(upload_measurement) = speedtest_result.upload_measurement {
                    upload_measurement.kbps()
                } else {
                    0
                }
            ),
        ),
        ("promo", format!("")),
        ("startmode", "pingselect".to_string()),
        ("recommendedserverid", format!("{}", server)),
        ("accuracy", "1".to_string()),
        ("serverid", format!("{}", server)),
        ("hash", speedtest_result.hash()),
    ];

    let body = url::form_urlencoded::Serializer::new(String::new())
        .extend_pairs(pairs.iter())
        .finish();

    info!("Share Body Request: {:?}", body);

    let client = Client::new();
    let res = client
        .post("http://www.speedtest.net/api/api.php")
        .header(CONNECTION, "close")
        .header(REFERER, "http://c.speedtest.net/flash/speedtest.swf")
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send();
    let mut encode_return = String::new();
    res?.read_to_string(&mut encode_return)?;
    let response_id = parse_share_request_response_id(encode_return.as_bytes())?;
    Ok(format!(
        "http://www.speedtest.net/result/{}.png",
        response_id
    ))
}

pub fn parse_share_request_response_id(input: &[u8]) -> Result<String, Error> {
    let pairs = url::form_urlencoded::parse(input);
    for pair in pairs {
        if pair.0 == "resultid" {
            return Ok(pair.1.into_owned());
        }
    }
    Err(Error::ParseShareUrlError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_share_request_response_id() {
        let resp = "resultid=4932415710&date=12%2F21%2F2015&time=5%3A10+AM&rating=0".as_bytes();
        assert_eq!(parse_share_request_response_id(resp).unwrap(), "4932415710");
    }

    #[test]
    fn test_share_url_hash() {
        let download_measurement = SpeedMeasurement {
            size: (6096 * 100) as usize,
            duration: Duration::seconds(1),
        };
        println!("Download: {:?}", download_measurement);
        let upload_measurement = SpeedMeasurement {
            size: (1861 * 100) as usize,
            duration: Duration::seconds(1),
        };
        println!("Upload: {:?}", upload_measurement);
        let server = SpeedTestServer {
            country: "".to_owned(),
            host: "".to_owned(),
            id: 5116,
            location: EarthLocation {
                latitude: 0.0,
                longitude: 0.0,
            },
            distance: None,
            name: "".to_owned(),
            sponsor: "".to_owned(),
            url: "".to_owned(),
        };
        println!("Server: {:?}", server);
        let latency_measurement = SpeedTestLatencyTestResult {
            server: &server,
            latency: Duration::milliseconds(26),
        };
        println!("Latency: {:?}", latency_measurement);
        let request = SpeedTestResult {
            download_measurement: Some(&download_measurement),
            upload_measurement: Some(&upload_measurement),
            server: &server,
            latency_measurement: &latency_measurement,
        };
        assert_eq!(request.hash(), "f10eb3dd8d3c38a221e823d859680045");
    }

    #[test]
    fn test_construct_share_form() {}

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

    #[test]
    fn test_parse_speedtest_servers_xml() {
        let parser = EventReader::new(include_bytes!(
            "../tests/confi\
             g/stripped-ser\
             vers-static.\
             php.xml"
        ) as &[u8]);
        let spt_server_config = SpeedTestServersConfig::new(parser).unwrap();
        assert!(spt_server_config.servers.len() > 5);
        let server = spt_server_config.servers.get(1).unwrap();
        assert!(!server.url.is_empty());
        assert!(!server.country.is_empty());
    }

    #[test]
    fn test_fastest_server() {
        let spt_config = SpeedTestConfig {
            ip: "127.0.0.1".to_string(),
            location: EarthLocation {
                latitude: 32.9954,
                longitude: -117.0753,
            },
            isp: "xxxfinity".to_string(),
        };
        let parser = EventReader::new(include_bytes!(
            "../tests/confi\
             g/geo-test-ser\
             vers-static.\
             php.xml"
        ) as &[u8]);
        let config = SpeedTestServersConfig::new(parser).unwrap();
        let closest_server = &config.servers_sorted_by_distance(&spt_config)[0];
        assert_eq!("Los Angeles, CA", closest_server.name);
    }

    #[test]
    fn test_get_configuration() {
        use mockito::mock;

        let _m = mock("GET", "/speedtest-config.php")
            .with_status(200)
            .with_body_from_file("tests/config/stripped-config.php.xml")
            .create();
        let _config = get_configuration();
    }

    #[test]
    fn test_get_server_list_with_config() {
        use mockito::mock;

        let _m = mock("GET", "/speedtest-config.php")
            .with_status(200)
            .with_body_from_file("tests/config/servers-static.php.xml")
            .create();

        let _server_list_config = get_server_list_with_config(None);
    }
}
