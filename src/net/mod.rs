pub mod http;
pub mod ws;

use http::HTTPClient;
use ws::WSClient;

enum Proto {
    WS,
    HTTP,
}

enum NetClient {
    HTTP(HTTPClient),
    WS(WSClient),
}

pub struct Client {}

impl Client {
    pub fn new(proto: Proto) -> Result<NetClient, crate::AnyError> {
        match proto {
            Proto::WS => Ok(NetClient::WS(WSClient::new())),
            Proto::HTTP => Ok(NetClient::HTTP(HTTPClient::new())),
        }
    }
}
