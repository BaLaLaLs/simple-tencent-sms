use hmac::digest::CtOutput;
use hmac::{Hmac, Mac};
use sha2::{Sha256};

type HmacSha256 = Hmac<Sha256>;
pub fn hmac_sha256(key: &[u8], data: &[u8]) -> CtOutput<Hmac<Sha256>> {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize()
}
