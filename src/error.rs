use failure::Error;
use reqwest::Error;

error_chain!{
    foreign_links {
        Reqwest(ReqwestError);
        Io(::std::io::Error);
    }

    errors {
        ConfigParseError {}
        LatencyTestInvalidPath {}
        LatencyTestClosestError {}
    }
}

#[derive(Debug, Fail)]
enum SpeedtestError {
    SpeedtestCLIError { message: string },
    #[fail(display = "Configuration XML is invalid")]
    SpeedtestConfigError {},
    #[fail(display = "Servers XML is invalid")]
    SpeedtestServersError {},
    #[fail(display = "Servers XML is invalid", {})]
    HttpError(#[fail(cause)] SpeedtestHttpError),
}

#[derive(Debug, Fail)]
enum SpeedtestHttpError {
    #[fail(display = "Cannot retrieve speedtest configuration")]
    ConfigRetrievalError(#[fail(cause)] reqwest::Error),
    #[fail(display = "Cannot retrieve speedtest server list")]
    ServersRetrievalError(#[fail(cause)] reqwest::Error),
}