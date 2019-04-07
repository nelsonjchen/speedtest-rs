use reqwest::Error as ReqwestError;
use failure::{Error as FailureError, Fail};

#[derive(Debug, Fail)]
pub enum SpeedtestError {
//    Legacy Errors
    #[fail(display = "Reqwest",)]
    Reqwest(#[fail(cause)] ReqwestError),
    #[fail(display = "Io",)]
    Io(#[fail(cause)] ::std::io::Error),
    #[fail(display = "Legacy Config Parse error",)]
    ConfigParseError,
    #[fail(display = "Legacy Latency Test Invalid Path",)]
    LatencyTestInvalidPath,
    #[fail(display = "Legacy LatencyTestClosestError",)]
    LatencyTestClosestError,
//    New Errors
    #[fail(display = "{}", message)]
    SpeedtestCLIError { message: String },
    #[fail(display = "Configuration XML is invalid")]
    SpeedtestConfigError {},
    #[fail(display = "Servers XML is invalid")]
    SpeedtestServersError {},
    #[fail(display = "HttpError",)]
    SpeedtestHttpError(#[fail(cause)] SpeedtestHttpError),
    #[fail(display = "Server ID used for filtering was not an integer")]
    InvalidServerIDType,
    #[fail(display = "No servers matched when filtering")]
    NoMatchedServers,
    #[fail(display = "Could not connect to speedtest.net API to POST results")]
    ShareResultsConnectFailure,
    #[fail(display = "Unable to successfully POST results to speedtest.net API after connection")]
    ShareResultsSubmitFailure,
    #[fail(display = "testlength configuration reached during upload")]
    SpeedtestUploadTimeout,
    #[fail(display = "Unable to determine best server")]
    SpeedtestBestServerFailure,
    #[fail(display = "get_best_server not called or not able to determine best server")]
    SpeedtestMissingBestServer,
}

#[derive(Debug, Fail)]
pub enum SpeedtestHttpError {
    #[fail(display = "Cannot retrieve speedtest configuration")]
    ConfigRetrievalError(#[fail(cause)] ReqwestError),
    #[fail(display = "Cannot retrieve speedtest server list")]
    ServersRetrievalError(#[fail(cause)] ReqwestError),
}