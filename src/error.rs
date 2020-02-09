#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Io(::std::io::Error),
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
