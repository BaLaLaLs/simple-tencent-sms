use simple_tencent_sms::hmac_sha256;
use std::fmt::format;

#[test]
fn sha256_test() {
    let array = hmac_sha256("123456".as_bytes(), "123456".as_bytes());
    let hex_str: String = array
        .into_bytes()
        .into_iter()
        .map(|byte| format!("{:02x}", byte))
        .collect();
    assert_eq!(
        "b8ad08a3a547e35829b821b75370301dd8c4b06bdd7771f9b541a75914068718",
        hex_str
    );
}
