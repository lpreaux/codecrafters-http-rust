
pub(crate) enum HttpVersion {
    HTTP1_0,
    HTTP1_1,
    HTTP2,
    HTTP3,
}

impl HttpVersion {
    fn as_str(&self) -> &str {
        match self {
            HttpVersion::HTTP1_0 => "HTTP/1.0",
            HttpVersion::HTTP1_1 => "HTTP/1.1",
            HttpVersion::HTTP2 => "HTTP/2",
            HttpVersion::HTTP3 => "HTTP/3",
        }
    }
}
