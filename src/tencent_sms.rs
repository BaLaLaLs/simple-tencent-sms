use crate::region::Region;
use crate::utils::hmac_sha256;
use anyhow::{anyhow, Error, Result};
use chrono::prelude::*;
use reqwest::header;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};

pub struct TencentSMS {
    secret_id: String,
    secret_key: String,
    sms_app_id: String,
}
const HOST: &str = "sms.tencentcloudapi.com";
const VERSION: &str = "2021-01-11";
const SERVICE: &str = "sms";
const CONTENT_TYPE: &str = "content-type:application/json; charset=utf-8";

impl TencentSMS {
    pub fn new(secret_id: String, secret_key: String, sms_app_id: String) -> Self {
        TencentSMS {
            secret_id,
            secret_key,
            sms_app_id,
        }
    }

    pub async fn send_sms(
        &self,
        region: Region,
        sign_name: &str,
        phone_numbers: Vec<&str>,
        template_id: String,
        template_param: Vec<&str>,
    ) -> Result<ResponseJson<SendSmsResponse>> {
        let action = "SendSms";
        // ************* 步骤 1：拼接规范请求串 *************
        let req_json = json!({
            "PhoneNumberSet": phone_numbers,
            "SmsSdkAppId": self.sms_app_id,
            "SignName": sign_name,
            "TemplateId": template_id,
            "TemplateParamSet": template_param
        });
        let mut hasher = Sha256::default();
        hasher.update(req_json.to_string().as_bytes());
        let hashed_request_payload = hasher.finalize();
        let sign = format!(
            "{}\n/\n\n{}\nhost:{}\nx-tc-action:{}\n\ncontent-type;host;x-tc-action\n{}",
            "POST",
            CONTENT_TYPE,
            HOST,
            action.to_lowercase(),
            format!("{:x}", hashed_request_payload).to_lowercase()
        );
        // ************* 步骤 2：拼接待签名字符串 *************
        let time = Local::now();
        let time_date = time.format("%Y-%m-%d").to_string();
        let mut hasher = Sha256::default();
        hasher.update(sign.as_bytes());
        let hashed_canonical_request = hasher.finalize();
        let string_to_sign = format!(
            "TC3-HMAC-SHA256\n{}\n{}/{}/tc3_request\n{}",
            time.timestamp(),
            time_date,
            SERVICE,
            format!("{:x}", hashed_canonical_request).to_lowercase()
        );
        // ************* 步骤 3：计算签名 *************
        let signature_str = self.signature(time_date, string_to_sign);
        // ************* 步骤 4：拼接 Authorization *************
        let headers = self.builder_headers(region, action, time, signature_str)?;
        let request = Self::create_request();
        let response = request
            .post(format!("https://{}/", HOST))
            .headers(headers)
            .body(req_json.to_string())
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(anyhow!(
                "request error response status: {}",
                response.status()
            ));
        }
        let resp_json = response.json::<ResponseJson<SendSmsResponse>>().await?;
        Ok(resp_json)
    }

    fn signature(&self, time_date: String, string_to_sign: String) -> String {
        let secret_date = hmac_sha256(
            format!("TC3{}", self.secret_key).as_bytes(),
            time_date.as_bytes(),
        );
        let secret_service = hmac_sha256(&secret_date.into_bytes(), SERVICE.as_bytes());
        let secret_signing = hmac_sha256(&secret_service.into_bytes(), b"tc3_request");
        let signature = hmac_sha256(&secret_signing.into_bytes(), string_to_sign.as_bytes());
        let signature_str = hex::encode(signature.into_bytes());
        signature_str
    }

    fn builder_headers(&self, region: Region, action: &str, time: DateTime<Local>, signature_str: String) -> Result<HeaderMap, Error> {
        let authorization = format!(
            "TC3-HMAC-SHA256 Credential={}/{}/{}/tc3_request, SignedHeaders=content-type;host;x-tc-action, Signature={}",
            self.secret_id,
            time.format("%Y-%m-%d"),
            SERVICE,
            signature_str
        );
        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, authorization.parse()?);
        headers.insert(
            header::CONTENT_TYPE,
            "application/json; charset=utf-8".parse()?,
        );
        headers.insert(header::HOST, HOST.parse()?);
        headers.insert("X-TC-Action", action.parse()?);
        headers.insert("X-TC-Timestamp", time.timestamp().to_string().parse()?);
        headers.insert("X-TC-Version", VERSION.parse()?);
        headers.insert("X-TC-Region", region.get_region().parse()?);
        Ok(headers)
    }

    fn create_request() -> reqwest::Client {
        let client_builder = reqwest::Client::builder();
        let client = client_builder.build();
        client.unwrap()
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseJson<T> {
    #[serde(alias = "Response")]
    pub response: T,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SendSmsResponse {
    #[serde(alias = "RequestId")]
    pub request_id: String,
    #[serde(alias = "SendStatusSet")]
    pub send_status_set: Vec<SendStatus>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SendStatus {
    #[serde(alias = "SerialNo")]
    pub serial_no: String,
    #[serde(alias = "PhoneNumber")]
    pub phone_number: String,
    #[serde(alias = "Fee")]
    pub fee: u32,
    #[serde(alias = "SessionContext")]
    pub session_context: String,
    #[serde(alias = "Code")]
    pub code: String,
    #[serde(alias = "Message")]
    pub message: String,
    #[serde(alias = "IsoCode")]
    pub iso_code: String,
}
