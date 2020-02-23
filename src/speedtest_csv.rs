use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct SpeedTestCsvResult<'a> {
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

impl<'a> SpeedTestCsvResult<'a> {
    pub fn header_serialize(self) -> String {
        // Un-dynamic for now
        // Blocked on:
        // * https://github.com/BurntSushi/rust-csv/issues/161 being implemented or solved
        // * https://github.com/BurntSushi/rust-csv/pull/193/files, like in this?
        "Server ID,Sponsor,Server Name,Timestamp,Distance,Ping,Download,Upload,Share,IP Address".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_header_serialize() -> Result<(), Box<dyn Error>> {
        let original = "Server ID,Sponsor,Server Name,Timestamp,Distance,Ping,Download,Upload,Share,IP Address";

        let results = SpeedTestCsvResult::default();

        assert_eq!(results.header_serialize(), original);
        Ok(())
    }
}
