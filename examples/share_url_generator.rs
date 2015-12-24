// For every fake measurement generated, do a few real ones with sharing on.
extern crate speedtest_rs;

#[macro_use]
extern crate log;
extern crate env_logger;


#[macro_use]
extern crate clap;

extern crate time;

use time::Duration;
use clap::{App, Arg};
use speedtest_rs::speedtest;
use speedtest_rs::distance;

#[allow(dead_code)]
fn main() {
    env_logger::init().unwrap();
    let matches = App::new("share_url_generator")
                      .version(&crate_version!()[..])
                      .about("Generate share URLs. WARNING: Use realistic inputs. Don't \
                              contaminate.")
                      .arg(Arg::with_name("download")
                               .help("Download speed in kilobytes per second to report")
                               .index(1)
                               .required(true))
                      .arg(Arg::with_name("upload")
                               .help("Upload speed in kilobytes per second to report")
                               .index(2)
                               .required(true))
                      .arg(Arg::with_name("server")
                               .index(3)
                               .help("Server numeric id to report.")
                               .required(true))
                      .arg(Arg::with_name("ping")
                               .index(4)
                               .help("Ping time in milliseconds to report.")
                               .required(true))
                      .get_matches();
    let download_kb = value_t!(matches.value_of("download"), u32).unwrap();
    let download_measurement = speedtest::SpeedMeasurement {
        size: (download_kb * 1000) as usize,
        duration: Duration::seconds(1),
    };
    println!("Download: {:?}", download_measurement);
    let upload_kb = value_t!(matches.value_of("upload"), u32).unwrap();
    let upload_measurement = speedtest::SpeedMeasurement {
        size: (upload_kb * 1000) as usize,
        duration: Duration::seconds(1),
    };
    println!("Upload: {:?}", upload_measurement);
    let server_id = value_t!(matches.value_of("server"), u32).unwrap();
    let server = speedtest::SpeedTestServer {
        country: "".to_owned(),
        host: "".to_owned(),
        id: server_id,
        location: distance::EarthLocation {
            latitude: 0.0,
            longitude: 0.0,
        },
        distance: None,
        name: "".to_owned(),
        sponsor: "".to_owned(),
        url: "".to_owned(),
    };
    println!("Server: {:?}", server);
    let ping = value_t!(matches.value_of("ping"), u32).unwrap() as i64;
    let latency_measurement = speedtest::SpeedTestLatencyTestResult {
        server: &server,
        latency: Duration::milliseconds(ping),
    };
    println!("Latency: {:?}", latency_measurement);
    let request = speedtest::ShareUrlRequest {
        download_measurement: &download_measurement,
        upload_measurement: &upload_measurement,
        server: &server,
        latency_measurement: &latency_measurement,
    };
    let url = speedtest::get_share_url(&request);
    println!("URL: {}", url);
}
