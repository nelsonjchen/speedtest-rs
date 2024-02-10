use thiserror::Error;
#[derive(Debug, Error)]
pub enum SpeedTestError {
    #[error("Reqwest error: {0}")]
    Reqwest(reqwest::Error),
    #[error("Io error: {0}")]
    Io(::std::io::Error),
    #[error("Csv error: {0}")]
    Csv(csv::Error),
    #[error("ParseFloatError error: {0}")]
    ParseFloatError(std::num::ParseFloatError),
    #[error("ParseIntError error: {0}")]
    ParseIntError(std::num::ParseIntError),
    #[error("AddrParseError error: {0}")]
    AddrParseError(std::net::AddrParseError),
    #[error("RoXmlTreeError error: {0}")]
    RoXmlTreeError(roxmltree::Error),
    #[error("ConfigParseError error")]
    ConfigParseError,
    #[error("ServerParseError error")]
    ServerParseError,
    #[error("LatencyTestInvalidPath error")]
    LatencyTestInvalidPath,
    #[error("LatencyTestClosestError error")]
    LatencyTestClosestError,
    #[error("UrlParseError error: {0}")]
    UrlParseError(url::ParseError),
    #[error("SystemTimeError error: {0}")]
    SystemTimeError(std::time::SystemTimeError),
    #[error("ParseShareUrlError error")]
    ParseShareUrlError,
    #[error("ThreadPoolBuildError error: {0}")]
    ThreadPoolBuildError(rayon::ThreadPoolBuildError),
}

impl From<reqwest::Error> for SpeedTestError {
    fn from(err: reqwest::Error) -> SpeedTestError {
        SpeedTestError::Reqwest(err)
    }
}

impl From<::std::io::Error> for SpeedTestError {
    fn from(err: ::std::io::Error) -> SpeedTestError {
        SpeedTestError::Io(err)
    }
}

impl From<csv::Error> for SpeedTestError {
    fn from(err: csv::Error) -> SpeedTestError {
        SpeedTestError::Csv(err)
    }
}

impl From<std::num::ParseFloatError> for SpeedTestError {
    fn from(err: std::num::ParseFloatError) -> SpeedTestError {
        SpeedTestError::ParseFloatError(err)
    }
}

impl From<std::num::ParseIntError> for SpeedTestError {
    fn from(err: std::num::ParseIntError) -> SpeedTestError {
        SpeedTestError::ParseIntError(err)
    }
}

impl From<std::net::AddrParseError> for SpeedTestError {
    fn from(err: std::net::AddrParseError) -> SpeedTestError {
        SpeedTestError::AddrParseError(err)
    }
}

impl From<roxmltree::Error> for SpeedTestError {
    fn from(err: roxmltree::Error) -> SpeedTestError {
        SpeedTestError::RoXmlTreeError(err)
    }
}

impl From<url::ParseError> for SpeedTestError {
    fn from(err: url::ParseError) -> SpeedTestError {
        SpeedTestError::UrlParseError(err)
    }
}

impl From<std::time::SystemTimeError> for SpeedTestError {
    fn from(err: std::time::SystemTimeError) -> SpeedTestError {
        SpeedTestError::SystemTimeError(err)
    }
}

impl From<rayon::ThreadPoolBuildError> for SpeedTestError {
    fn from(err: rayon::ThreadPoolBuildError) -> SpeedTestError {
        SpeedTestError::ThreadPoolBuildError(err)
    }
}
