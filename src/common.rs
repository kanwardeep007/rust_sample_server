use std::net::TcpListener;

pub trait ServerTrait {
    fn start_listening(&self, listener: TcpListener) -> anyhow::Result<()>;
}
