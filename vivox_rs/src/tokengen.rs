use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct TokenRequest {
  iss: String,
  exp: u64,
  vxa: String,
  vxi: u64,
  f: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  t: Option<String>,
}

#[derive(Debug, Copy, Clone)]
pub struct TokenGenerator {
  req_index: u64,
}

impl TokenGenerator {
  pub const fn init() -> Self {
    Self {
      req_index: 0,
    }
  }

  pub fn generate(
    mut self,
    key: &str,
    issuer: &str,
    exp: u64,
    vxa: &str,
    f: &str,
    t: Option<String>,
  ) -> String {
    use data_encoding::BASE64URL_NOPAD;
    use hmac::{Hmac, Mac, NewMac};
    use sha2::Sha256;
    // Header is static - base64url encoded {}
    let header = BASE64URL_NOPAD.encode(b"{}");
  
    // Create payload and base64 encode
    let tr = TokenRequest {
      iss: issuer.to_string(),
      exp,
      vxa: vxa.to_string(),
      vxi: self.req_index,
      f: f.to_string(),
      t,
    };
    let mut obj = serde_json::to_string(&tr)
      .expect("Unable to serialize object!");
  
    obj = BASE64URL_NOPAD.encode(obj.as_bytes());
  
    // Join segments to prepare for signing
    let to_sign = format!("{}.{}", header, obj);
  
    // Create alias for HMAC-SHA256
    type HmacSha256 = Hmac<Sha256>;
  
    let mut mac = HmacSha256::new_varkey(key.as_bytes())
      .expect("HMAC can take key of any size");
  
    mac.update(to_sign.as_bytes());
  
    let res = mac.finalize();
  
    // Sign token with key and HMACSHA256, then base64 encode
    let signed_payload = BASE64URL_NOPAD.encode(&res.into_bytes());

    // Token has been generated. Increment counter

    self.req_index += 1;
  
    // Combine header and payload with signature
    format!("{}.{}", to_sign, signed_payload)
  }  
}
