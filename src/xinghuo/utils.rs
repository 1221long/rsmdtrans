
    use hmac_sha256::HMAC;

    pub fn hmacsha256(api_secret_key: String, data: String) -> Result<String, String> 
    {        
        let key_bytes = api_secret_key.into_bytes();
        let data_bytes = data.into_bytes();

        let mac_bytes = HMAC::mac(&data_bytes, &key_bytes);
        let data = String::from_utf8(mac_bytes.to_vec()).unwrap();
        Ok(data)

        // match String::from_utf8(mac_bytes.to_vec()) {
        //     Ok(string) => Ok(string),
        //     Err(e) => Err("")
        // }
    }
