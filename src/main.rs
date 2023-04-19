use kanwar_server::approach_1_server::Approach1Server;
use kanwar_server::common::ServerTrait;

use std::net::TcpListener;
fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8181").unwrap();
    let approach_1_server = Approach1Server::new(4);
    approach_1_server.start_listening(listener)?;
    Ok(())
}
