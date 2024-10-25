use std::io::Write;
use std::net::TcpListener;
use anyhow::Result;
use crate::http::response::ok::OkResponse;

pub struct Server {
    host: String,
    port: u16,
    listener: Option<TcpListener>,
}

impl Server {
    pub fn new(host: &str, port: u16) -> Server {
        Server {
            host: host.to_owned(),
            port,
            listener: None,
        }
    }

    pub fn init(&mut self) -> Result<()> {
        let listener = TcpListener::bind(self.addr())?;
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    println!("Accepcted new connection!");
                    stream.write(&OkResponse::get_response())?;
                },
                Err(e) => {
                    println!("error: {}", e)
                }
            }
        }
        Ok(())
    }

    fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}