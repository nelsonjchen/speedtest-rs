use hyper::error::Error as HyperError;
use self::SpeedTestError::*;

pub type Result<T> = ::std::result::Result<T, SpeedTestError>;

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
