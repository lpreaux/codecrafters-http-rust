
pub struct OkResponse;

impl OkResponse {
    pub fn get_response() -> Vec<u8> {
        format!("HTTP/1.1 200 OK\r\n\r\n").into_bytes()
    }
}