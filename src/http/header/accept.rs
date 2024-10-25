use crate::http::header::header_base::{HttpHeader, HttpHeaderBase};

const ACCEPT_HEADER_NAME: &str = "Accept";

pub struct AcceptHeader {
    base: HttpHeaderBase,
}

impl HttpHeader for AcceptHeader {
    fn get_header_name(&self) -> String {
        ACCEPT_HEADER_NAME.to_string()
    }
}