use std::{
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use log::info;
use reqwest::blocking::{Client, Request, Response};
use reqwest::header::{HeaderValue, CONNECTION, CONTENT_TYPE, REFERER, USER_AGENT};
use reqwest::Url;
use time::{now, Duration};

use crate::distance::EarthLocation;
use crate::error::Error;
use crate::speedtest_config::SpeedTestConfig;
use crate::speedtest_servers_config::SpeedTestServersConfig;
use rayon::prelude::*;


const ST_USER_AGENT: &str = concat!("reqwest/speedtest-rs ", env!("CARGO_PKG_VERSION"));

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
    let mut config_body = download_configuration()?;
    info!("Parsing Configuration");
    let spt_config = SpeedTestConfig::parse(&(config_body.text()?));
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
    config: &SpeedTestConfig,
) -> Result<SpeedTestServersConfig, Error> {
    let mut config_body = download_server_list()?;
    info!("Parsing Server List");
    let server_config_string = config_body.text()?;
    let spt_config = SpeedTestServersConfig::parse_with_config(&server_config_string, config);
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
            res?.bytes()?.last();
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

pub fn test_download_with_progress_and_config<F>(
    server: &SpeedTestServer,
    f: F,
    config: &SpeedTestConfig,
) -> Result<SpeedMeasurement, Error>
where
    F: Fn() + Send + Sync + 'static,
{
    info!("Testing Download speed");
    let mut root_url = Url::parse(&server.url)?;
    let query_pairs = root_url.query_pairs_mut();

    let urls = vec![];
    for size in config.sizes.download {
        let mut download_with_size_url = root_url.clone();
        let path_segments_mut = download_with_size_url
            .path_segments_mut()
            .map_err(|_| Error::ServerParseError)?;
        path_segments_mut.push(&format!("random{}x{}.jpg", size, size));
        for cache_bump in 0..config.counts.download {
            // let mut cache_busting_url = download_with_size_url.clone();
            // let query_pairs_mut = cache_busting_url.query_pairs_mut();
            // query_pairs_mut.append_pair("x", &format!("{}", cache_bump));
            urls.push(download_with_size_url.clone());
        }
    }

    let request_count = urls.len();
    let requests = vec![];
    urls.iter()
        .enumerate()
        .map(|(i, url)| {
            let mut cache_busting_url = url.clone();
            cache_busting_url.query_pairs_mut().append_pair(
                "x",
                &format!(
                    "{}.{}",
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)?
                        .as_millis()
                        .to_string(),
                    i
                ),
            );
            let mut request = Request::new(reqwest::Method::GET, url.clone());
            request.headers_mut().insert(
                reqwest::header::CACHE_CONTROL,
                HeaderValue::from_static("no-cache"),
            );
            requests.push(request);
            Ok(())
        })
        .collect()?;

    let pool = rayon::ThreadPoolBuilder::new().num_threads(config.threads.download).build()?;
    let res = pool.install(|| {
        requests.par_iter().map(|request|
            request
        );
    });
}

pub fn test_upload_with_progress_and_config<F>(
    server: &SpeedTestServer,
    f: F,
    config: &SpeedTestConfig,
) -> Result<SpeedMeasurement, Error> {
    todo!()
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
    let encode_return = res?.text()?;
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

        let _server_list_config = get_server_list_with_config(&SpeedTestConfig::default());
    }
}
