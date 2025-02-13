pub mod http;
pub mod ws;

use http::HTTPClient;
use ws::WSClient;

pub enum Proto {
    WS,
    HTTP,
}

pub enum NetClient {
    HTTP(HTTPClient),
    WS(WSClient),
}

pub struct Client {
    inner: NetClient,
}

impl Client {
    pub fn new(proto: Proto) -> Result<Self, crate::AnyError> {
        match proto {
            Proto::WS => Ok(Client {
                inner: NetClient::WS(WSClient::new()),
            }),
            Proto::HTTP => Ok(Client {
                inner: NetClient::HTTP(HTTPClient::new()),
            }),
        }
    }
}
