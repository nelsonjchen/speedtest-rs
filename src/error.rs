use reqwest::Error as ReqwestError;

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
