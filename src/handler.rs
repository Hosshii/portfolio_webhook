use actix_web::{web, HttpRequest, HttpResponse, Responder};

use github_webhook::event::{self, Event};
use hex::FromHex;
use ring::{constant_time::verify_slices_are_equal, hmac};
use serde_json;
use std::sync::Arc;

const X_GITHUB_EVENT: &str = "X-Github-Event";
const X_HUB_SIGNATURE: &str = "X-Hub-Signature-256";

pub async fn webhook(req: HttpRequest, data: web::Data<AppState>, body: String) -> impl Responder {
    let secret = data.secret.as_ref();
    let event = req.headers().get(X_GITHUB_EVENT).unwrap().to_str().unwrap();
    let signature = req
        .headers()
        .get(X_HUB_SIGNATURE)
        .unwrap()
        .to_str()
        .unwrap();

    if !verify_signature(secret, &body, signature) {
        return HttpResponse::Forbidden().body("Not authorized");
    }

    let payload = event::patch_payload_json(event, &body);
    let event = serde_json::from_str::<Event>(&payload);

    HttpResponse::Ok().body("Hey there!")
}

fn verify_signature(secret: &str, payload: &str, signature: &str) -> bool {
    println!("{}", payload);
    println!("{}", secret);
    println!("{}", signature);

    let secret = secret.as_bytes();
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

#[derive(Clone, Debug)]
pub struct AppState {
    secret: Arc<String>,
}

impl AppState {
    pub fn new(secret: impl Into<String>) -> Self {
        Self {
            secret: Arc::new(secret.into()),
        }
    }
}
