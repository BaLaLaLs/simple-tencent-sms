use simple_tencent_sms::{Region, TencentSMS};

#[tokio::test]
async fn send_sms_test() -> Result<(), anyhow::Error> {
    let tencent_sms = TencentSMS::new(
        "secret_id".into(),
        "secret_key".into(),
        "sms_app_id".into(),
    );
    let response_json = tencent_sms
        .send_sms(
            Region::Beijing,
            "sign_name".into(),
            vec!["+86xxxxxxx"],
            "template_id".into(),
            vec!["xxxx"],
        )
        .await?;
    println!("{:?}", response_json);
    Ok(())
}
