use std::env;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use xinghuo::contracts::{self, Header};
mod xinghuo;

fn main() {
    xh_api();
    // fun1();
}

#[tokio::main]
async fn xh_api() {
    let url = xinghuo::functions::GetAuthUrlWs().unwrap();
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    let _header = Header { app_id :  };

    let _request = JsonRequest{
        hearder:
    };

}

fn fun1() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[0];
}
