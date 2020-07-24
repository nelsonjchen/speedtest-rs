#[derive(Debug)]
pub enum Error {
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
    LatencyTestClosestError,
    UrlParseError(url::ParseError),
    SystemTimeError(std::time::SystemTimeError),
    ParseShareUrlError,
    ThreadPoolBuildError(rayon::ThreadPoolBuildError),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::Reqwest(err)
    }
}

impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<csv::Error> for Error {
    fn from(err: csv::Error) -> Error {
        Error::Csv(err)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Error {
        Error::ParseFloatError(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Error {
        Error::ParseIntError(err)
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(err: std::net::AddrParseError) -> Error {
        Error::AddrParseError(err)
    }
}

impl From<roxmltree::Error> for Error {
    fn from(err: roxmltree::Error) -> Error {
        Error::RoXmlTreeError(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::UrlParseError(err)
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(err: std::time::SystemTimeError) -> Error {
        Error::SystemTimeError(err)
    }
}

impl From<rayon::ThreadPoolBuildError> for Error {
    fn from(err: rayon::ThreadPoolBuildError) -> Error {
        Error::ThreadPoolBuildError(err)
    }
}
