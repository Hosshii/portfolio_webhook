use actix_web::{
    web::{self},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};

use anyhow::{bail, Context, Result};
use github_webhook::event::{self, Event};
use hex::FromHex;
use ring::{constant_time::verify_slices_are_equal, hmac};
use serde_json;
use std::sync::Arc;

const X_GITHUB_EVENT: &str = "X-Github-Event";
const X_HUB_SIGNATURE: &str = "X-Hub-Signature-256";

pub async fn webhook(
    mut req: HttpRequest,
    hook: web::Data<WebHook>,
    body: String,
) -> impl Responder {
    let result = hook.parse(&mut req, &body);

    match result {
        Ok(event) => {
            println!("{:?}", event);

            HttpResponse::Ok().body("correctly parsed")
        }
        Err(e) => {
            println!("{}", e);
            println!("{:?}", e);
            HttpResponse::BadRequest().body(e.to_string())
        }
    }
}

#[derive(Clone, Debug)]
pub struct WebHook {
    secret: Arc<String>,
}

impl WebHook {
    pub fn new(secret: impl Into<String>) -> Self {
        Self {
            secret: Arc::new(secret.into()),
        }
    }

    pub fn authenticate(&self, payload: &str, signature: &str) -> bool {
        let secret = self.secret.as_bytes();
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

    pub fn parse(&self, req: &mut HttpRequest, body: &String) -> Result<Event> {
        let event = req.headers().get(X_GITHUB_EVENT).unwrap().to_str().unwrap();
        let signature = req
            .headers()
            .get(X_HUB_SIGNATURE)
            .unwrap()
            .to_str()
            .unwrap();

        if !self.authenticate(&body, signature) {
            bail!("permission denied")
        }

        let payload = event::patch_payload_json(event, &body);
        let event = serde_json::from_str::<Event>(&payload);
        event.context("parse failed")
    }
}
