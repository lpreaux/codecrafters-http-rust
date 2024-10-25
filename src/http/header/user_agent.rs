use crate::http::header::header_base::{HttpHeader, HttpHeaderBase};

const USER_AGENT_HEADER_NAME: &str = "User-Agent";

pub struct UserAgentHeader {
    base: HttpHeaderBase
}

impl HttpHeader for UserAgentHeader {
    fn get_header_name(&self) -> String {
        USER_AGENT_HEADER_NAME.to_string()
    }
}