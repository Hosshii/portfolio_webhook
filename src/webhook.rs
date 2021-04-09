use actix_web::{HttpMessage, HttpRequest};

use crate::error::MyError;
use github_webhook::event::{self, Event};
use hex::{FromHex, ToHex};
use log::info;
use reqwest::header::HeaderMap;
use reqwest::Response;
use ring::{constant_time::verify_slices_are_equal, hmac};
use serde_json;
use std::sync::Arc;

const X_GITHUB_EVENT: &str = "X-Github-Event";
const X_HUB_SIGNATURE: &str = "X-Hub-Signature-256";

fn generate_signature<'a>(message: &'a str, secret: &'a str) -> Vec<u8> {
    let key = hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, secret.as_bytes());
    let tag = hmac::sign(&key, message.as_bytes());
    tag.as_ref().to_vec()
}

pub fn authenticate(github_secret: &str, payload: &str, signature: &str) -> bool {
    let secret = github_secret.as_bytes();
    let payload = payload.as_bytes();
    // let payload = payload[..payload.len() - 1].as_bytes();
    let prefix = signature[7..signature.len()].as_bytes();
    match Vec::from_hex(prefix) {
        Ok(sig_bytes) => {
            let key = hmac::Key::new(hmac::HMAC_SHA256, secret);
            let tag = hmac::sign(&key, payload);
            verify_slices_are_equal(tag.as_ref(), &sig_bytes).is_ok()
        }
        Err(_) => false,
    }
}

#[derive(Clone)]
pub struct WebHook {
    github_secret: Arc<String>,
    traq_secret: Arc<String>,
    traq_webhook_id: Arc<String>,
}

impl WebHook {
    pub fn new(
        github_secret: impl Into<String>,
        traq_secret: impl Into<String>,
        traq_webhook_id: impl Into<String>,
    ) -> Self {
        Self {
            github_secret: Arc::new(github_secret.into()),
            traq_secret: Arc::new(traq_secret.into()),
            traq_webhook_id: Arc::new(traq_webhook_id.into()),
        }
    }

    pub fn parse_and_authenticate(
        &self,
        req: &mut HttpRequest,
        body: &String,
    ) -> Result<Event, MyError> {
        let event = req.headers().get(X_GITHUB_EVENT).unwrap().to_str().unwrap();
        let signature = req
            .headers()
            .get(X_HUB_SIGNATURE)
            .unwrap()
            .to_str()
            .unwrap();

        if !authenticate(&self.github_secret, &body, signature) {
            return Err(MyError::UnAuthorized);
        }

        let payload = event::patch_payload_json(event, &body);
        let event = serde_json::from_str::<Event>(&payload)?;
        Ok(event)
    }

    pub async fn post_message(&self, message: impl Into<String>) -> Result<Response, MyError> {
        let message = message.into();
        let url = &format!("https://q.trap.jp/api/v3/webhooks/{}", self.traq_webhook_id);
        let client = reqwest::Client::new();

        let sig = generate_signature(&message, &self.traq_secret).encode_hex::<String>();
        let mut headers = HeaderMap::new();
        headers.insert("X-TRAQ_Signature", sig.parse().unwrap());
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "text/plain; charset=utf-8".parse().unwrap(),
        );

        let res = client
            .post(url)
            .headers(headers)
            .body(message.clone())
            .send()
            .await?;
        info!(
            "Message sent to {}, message: {}, response: {:?}",
            url, message, res
        );

        Ok(res)
    }
}
