use hyper::error::Error as HyperError;
use self::Error::*;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ConfigDownloadFailed,
    ConfigParseError,
    ServerListParseError,
    Hyper(HyperError),
}

impl From<HyperError> for Error {
    fn from(err: HyperError) -> Error {
        Hyper(err)
    }
}
