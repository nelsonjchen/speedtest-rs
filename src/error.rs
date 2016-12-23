use self::Error::*;
use reqwest::Error as ReqwestError;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ConfigDownloadFailed,
    ConfigParseError,
    ServerListParseError,
    LatencyTestInvalidPath,
    LatencyTestClosestError,
    Reqwest(ReqwestError),
}

impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Error {
        Reqwest(err)
    }
}
