use reqwest::Error as ReqwestError;

error_chain!{
    foreign_links {
        Reqwest(ReqwestError);
    }

    errors {
        ConfigDownloadFailed {}
        ConfigParseError {}
        ServerListParseError {}
        LatencyTestInvalidPath {}
        LatencyTestClosestError {}
    }
}
