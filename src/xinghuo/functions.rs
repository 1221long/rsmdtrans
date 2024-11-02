
    use std::mem::replace;

    use chrono::Utc;
    use url::Url;
    use base64::prelude::*;

    pub fn GetAuthUrl() -> Result<String, String>{

        // Fri, 01 Nov 2024 07:42:05 GMT
        let date_str = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();

        let _url = Url::parse(super::settings::HOST_URL).unwrap();
        let sign_verification = format!("host: {}\ndate: {}\nGET {} HTTP/1.1",_url.host_str().unwrap(),date_str,_url.path());
        let sha = super::utils::HMACsha256(super::settings::API_SECRET.to_string(), sign_verification)?;
        let authorization = format!("api_key=\"{0}\", algorithm=\"{1}\", headers=\"{2}\", signature=\"{3}\"", super::settings::API_KEY.to_string(), "hmac-sha256", "host date request-line", sha);
        let mut new_url = format!("https://{}{}",_url.host_str().unwrap(),_url.path());

        let path1 = format!("authorization={}", BASE64_STANDARD.encode(authorization.as_bytes()));
        let path2 = format!("date={}",date_str.replace(" ", "%20").replace(":", "%3A").replace(",", "%2C"));
        let path3 = format!("host={}",_url.host_str().unwrap());

        new_url = format!("{}?{}&{}&{}", new_url, path1, path2, path3);

        Ok(new_url)
    }

    pub fn GetAuthUrlWs() -> Result<String, String> {
        let url = GetAuthUrl()?;
        Ok(url.replace("http://", "ws://").replace("https://", "wss://"))
    }
