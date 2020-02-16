use serde::Serialize;

#[derive(Debug, Serialize)]
struct SpeedTestCsvResult<'a> {
    server_id: &'a str,
    sponsor: &'a str,
    server_name: &'a str,
    timestamp: &'a str,
    distance: &'a str,
    ping: &'a str,
    download: &'a str,
    upload: &'a str,
    share: &'a str,
    ip_address: &'a str,
}

#[cfg(test)]
mod tests {
    use csv::WriterBuilder;
    use std::error::Error;

    #[test]
    fn test_header_serialize() -> Result<(), Box<dyn Error>> {
        let original = "Server ID,Sponsor,Server Name,Timestamp,Distance,Ping,Download,Upload,Share,IP Address";

        let wtr = WriterBuilder::new().has_headers(true).from_writer(vec![]);

        let data = String::from_utf8(wtr.into_inner()?)?;
        assert_eq!(data, original);
        Ok(())
    }
}
