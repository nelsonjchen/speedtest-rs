#![allow(dead_code)]

#[derive(Debug)]
pub enum SpeedTestError {
    Reqwest(reqwest::Error),
    Io(::std::io::Error),
    Csv(csv::Error),
    ParseFloatError(std::num::ParseFloatError),
    ParseIntError(std::num::ParseIntError),
    AddrParseError(std::net::AddrParseError),
    RoXmlTreeError(roxmltree::Error),
    ConfigParseError,
    ServerParseError,
    LatencyTestInvalidPath,
    LatencyTestNoServerError,
    LatencyTestClosestError,
    UrlParseError(url::ParseError),
    SystemTimeError(std::time::SystemTimeError),
    ParseShareUrlError,
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
