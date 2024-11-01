use sha2::Sha256;
use hmac::{Hmac, Mac};

mod utils {


    pub fn HMACsha256(api_secret_key: String, data: String) -> Result<String, String> 
    {
        type HmacSha256 = Hmac<Sha256>;
        
        let mut mac = HmacSha256::new_from_slice(api_secret_key).except();

        Ok("Ok".to_string())
    }
}