use serde::Serialize;
use crate::speedtest::SpeedTestResult;

#[derive(Debug, Serialize, Default)]
pub struct SpeedTestJsonResult<'a> {
    pub download: f64,
    pub upload: f64,
    pub ping: String,
    pub server: SpeedTestJsonServerResult<'a>,
    pub timestamp: &'a str,
    pub bytes_sent: &'a str,
    pub bytes_received: &'a str,
    pub share: &'a str,
    pub client: SpeedTestJsonClientResult<'a>,
}

impl<'a> SpeedTestJsonResult<'a> {
    fn new(result: &SpeedTestResult) -> SpeedTestJsonResult<'a> {
        SpeedTestJsonResult {
            download: result.download_measurement.bps_f64(),
            upload: result.upload_measurement.bps_f64(),
            ping: format!(
                "{:.2}",
                result.latency_measurement.latency.num_microseconds().unwrap_or(0) as f64/ 1000.0
            ),
            server: Default::default(),
            timestamp: "",
            bytes_sent: "",
            bytes_received: "",
            share: "",
            client: Default::default()
        }
    }
}

#[derive(Debug, Serialize, Default)]
pub struct SpeedTestJsonServerResult<'a> {
    url: &'a str,
    lat: f32,
    lon: f32,
    name: &'a str,
    country: &'a str,
    cc: &'a str,
    sponsor: &'a str,
    id: u32,
    url2: &'a str,
    host: &'a str,
    d: f32,
    latency: &'a str,
}

impl<'a> SpeedTestJsonServerResult<'a> {
    fn new(result: &SpeedTestResult<'a>) -> SpeedTestJsonServerResult<'a> {
        SpeedTestJsonServerResult {
            url: &result.server.url,
            lat: result.server.location.latitude,
            lon: result.server.location.longitude,
            name: &result.server.name,
            country: &result.server.country,
            cc: &result.server.cc,
            sponsor: &result.server.sponsor,
            id: result.server.id,
            url2: &result.server.url2,
            host: &result.server.host,
            d: result.server.distance.unwrap_or(0.0),
            latency: ""
        }
    }
}

#[derive(Debug, Serialize, Default)]
pub struct SpeedTestJsonClientResult<'a> {
    ip: &'a str,
    lat: &'a str,
    lon: &'a str,
    isp: &'a str,
    isprating: &'a str,
    rating: &'a str,
    ispdlavg: &'a str,
    ispulavg: &'a str,
    loggedin: &'a str,
    country: &'a str,
}

mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_make_json_result() -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
