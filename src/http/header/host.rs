use crate::http::header::header_base::{HttpHeader, HttpHeaderBase};

const HOST_HEADER_NAME: &str = "host";

pub struct HostHeader {
    base: HttpHeaderBase,
}

impl HttpHeader for HostHeader {
    fn get_header_name(&self) -> String {
        HOST_HEADER_NAME.to_string()
    }
}