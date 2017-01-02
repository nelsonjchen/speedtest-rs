use reqwest::Error as ReqwestError;

error_chain!{
    foreign_links {
        Reqwest(ReqwestError);
    }

    errors {
        ConfigParseError {}
        LatencyTestInvalidPath {}
        LatencyTestClosestError {}
    }
}
