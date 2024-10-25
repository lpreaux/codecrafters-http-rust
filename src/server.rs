use std::net::TcpListener;
use anyhow::Result;

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
                Ok(_stream) => {
                    println!("Accepcted new connection!");
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