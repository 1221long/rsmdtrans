
use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::prelude::*;

    pub fn hmacsha256(api_secret_key: String, data: String) -> Result<String, String> 
    {
        type HmacSha256 = Hmac<Sha256>;
        //println!("[key] {}", api_secret_key);
        //println!("[data] {}", data);
        let key_bytes = api_secret_key.into_bytes();
        let data_bytes = data.into_bytes();

        let mut mac= HmacSha256::new_from_slice(&key_bytes).expect("HMAC can take key of any size");
        mac.update(&data_bytes);
        let result = mac.finalize();
        //println!("[result] {:?}", result);
        let code_bytes = result.into_bytes();
        //println!("[code_bytes] {:?}", code_bytes);
        // https://docs.rs/hmac/0.12.1/hmac/
        let data = BASE64_STANDARD.encode(code_bytes);
        Ok(data)

        // match String::from_utf8(mac_bytes.to_vec()) {
        //     Ok(string) => Ok(string),
        //     Err(e) => Err("")
        // }
    }
