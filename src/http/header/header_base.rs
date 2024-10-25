
pub struct HttpHeaderBase {
    value: String,
}


pub trait HttpHeader {
    fn get_header_name(&self) -> String;
}