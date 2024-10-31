
use sha2::Sha256;
use hmac::{Hmac, Mac};

pub fn HMACsha256(api_secret_key: String, data: String) -> Result<String, String> {
    type HmacSha256 = Hmac<Sha256>;
}

