use codecrafters_http_server::server::Server;
use anyhow::Result;

fn main() -> Result<()> {
    let mut server = Server::new("127.0.0.1", 4221);
    server.init()
}