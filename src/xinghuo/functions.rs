//use std::mem::replace;

use super::contracts::request;
use base64::prelude::*;
use chrono::Utc;
use tokio_tungstenite::tungstenite::http::Request;
use url::Url;

pub fn get_auth_url(with_tls: Option<bool>) -> Result<String, String> {
    // Fri, 01 Nov 2024 07:42:05 GMT
    let date_str = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();

    let _url = Url::parse(super::settings::HOST_URL).unwrap();
    let sign_verification = format!(
        "host: {}\ndate: {}\nGET {} HTTP/1.1",
        _url.host_str().unwrap(),
        date_str,
        _url.path()
    );
    let sha = super::utils::hmacsha256(super::settings::API_SECRET.to_string(), sign_verification)?;
    let authorization = format!(
        "api_key=\"{0}\", algorithm=\"{1}\", headers=\"{2}\", signature=\"{3}\"",
        super::settings::API_KEY.to_string(),
        "hmac-sha256",
        "host date request-line",
        sha
    );
    let subfix = match with_tls {
        Some(true) => "s",
        Some(false) => "",
        None => "",
    };
    let mut new_url = format!(
        "http{}://{}{}",
        subfix,
        _url.host_str().unwrap(),
        _url.path()
    );

    let path1 = format!(
        "authorization={}",
        BASE64_STANDARD.encode(authorization.as_bytes())
    );
    let path2 = format!(
        "date={}",
        date_str
            .replace(" ", "%20")
            .replace(":", "%3A")
            .replace(",", "%2C")
    );
    let path3 = format!("host={}", _url.host_str().unwrap());

    new_url = format!("{}?{}&{}&{}", new_url, path1, path2, path3);

    Ok(new_url)
}

pub fn get_auth_url_ws(with_tls: Option<bool>) -> Result<String, String> {
    let url = get_auth_url(with_tls)?;
    Ok(url
        .replace("http://", "ws://")
        .replace("https://", "wss://"))
}

pub fn get_default_request() -> Result<request::Body, String> {
    let _header = request::Header {
        app_id: super::settings::APP_ID.to_string(),
        uid: "12345".to_string(),
    };

    let _chat = request::Chat {
        domain: "lite".to_string(),
        temperature: 0.5,
        max_tokens: 1024,
    };

    let _parameter = request::Parameter { chat: _chat };
    let mut _text = vec![request::Content {
        role: "user".to_string(),
        content: "今天天气怎么样？".to_string(),
    }];

    let mut _message = request::Message { text: _text };

    let mut _payload = request::Payload { message: _message };

    let mut _request = request::Body {
        header: _header,
        parameter: _parameter,
        payload: _payload,
    };

    Ok(_request)
}

pub fn add_new_content_to_request(_request: &mut request::Body, _context: String) {
    let _role = super::contracts::ROLE_USER.to_string();
    _request.payload.message.text.push(request::Content {
        role: _role,
        content: _context,
    });
}
