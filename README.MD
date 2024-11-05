### 🚀 安装
```
cargo add simple-tencent-sms
```
### 💡 使用
secret 获取相关可查看腾讯云官网文档
```rust
    let tencent_sms = TencentSMS::new(
        "secret_id".into(),
        "secret_key".into(),
        "sms_app_id".into(),
    );
```
#### 📲 发送短信
```rust
    let response_json = tencent_sms
        .send_sms(
            Region::Beijing,
            "sign_name".into(),
            vec!["+86xxxxxxx"],
            "template_id".into(),
            vec!["xxxx"],
        )
        .await?;
```
response 和官方文档返回json一致
```json
{
    "Response": {
        "SendStatusSet": [
            {
                "SerialNo": "5000:1045710669157053657849499619",
                "PhoneNumber": "+8618501234444",
                "Fee": 1,
                "SessionContext": "outsid_1729495320_1011",
                "Code": "Ok",
                "Message": "send success",
                "IsoCode": "CN"
            }
        ],
        "RequestId": "a0aabda6-cf91-4f3e-a81f-9198114a2279"
    }
}
```