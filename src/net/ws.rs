use crate::AnyError;
use std::net::TcpStream;
pub use tungstenite::Message::{Binary, Close};
use tungstenite::{client, WebSocket};
use url::Url;

pub struct WSClient {}

impl WSClient {
    pub fn new() -> Self {
        WSClient {}
    }

    pub fn connect(url: &str) -> Result<WebSocket<TcpStream>, AnyError> {
        let url = Url::parse(url)?;

        let host = url.host_str().ok_or("Incorrect hostname")?;
        let port = url.port_or_known_default().unwrap_or(80);
        let socket_addr = format!("{}:{}", host, port);

        let stream = TcpStream::connect(socket_addr)?;

        let (ws_stream, _) = client(url.as_str(), stream)?;

        Ok(ws_stream)
    }
}
