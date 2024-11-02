use std::env;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use xinghuo::contracts::{*};
mod xinghuo;

fn main() {
    xh_api();
    // fun1();
}

#[tokio::main]
async fn xh_api() {
    let url = xinghuo::functions::GetAuthUrlWs().unwrap();
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    let _header = Header { 
        app_id : xinghuo::settings::APP_ID.to_string(), 
        uid:  "12345".to_string(),
    };

    let _chat = Chat {
        domain: "lite".to_string(),
        temperature: 0.5,
        max_tokens: 1024,
    };

    let _parameter = Parameter {
        chat: _chat,
    };
    let mut _text = vec![Content{
        role: "user".to_string(),
        content:"你好呀!".to_string(),
    }];
    let mut _message = xinghuo::contracts::Message {
        text: _text
    };

    let mut _payload = Payload {
        message: _message,
    };

    let mut _request = JsonRequest {
        header: _header,
        parameter: _parameter,
        payload: _payload,
    };



}

fn fun1() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[0];
}
