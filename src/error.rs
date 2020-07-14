#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Io(::std::io::Error),
    Csv(csv::Error),
    ParseFloatError(std::num::ParseFloatError),
    ConfigParseError,
    LatencyTestInvalidPath,
    LatencyTestClosestError,
    ParseShareUrlError,
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

