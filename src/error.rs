use hyper::error::Error as HyperError;

pub type Result<T> = ::std::result::Result<T, Error>;

use self::SpeedTestError::*;

#[derive(Debug)]
pub enum SpeedTestError {
    ConfigDownloadFailed,
    Hyper(HyperError),
}

impl From<HyperError> for SpeedTestError {
    fn from(err: HyperError) -> SpeedTestError {
        SpeedTestError::Hyper(err)
    }
}
