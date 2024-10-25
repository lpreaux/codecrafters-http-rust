use crate::http::http_version::HttpVersion;
use crate::http::header::header_kind::HttpHeaderKind;

pub struct RequestBase {
    target: String,
    http: HttpVersion,
    headers: Vec<HttpHeaderKind>,
    body: String,
}