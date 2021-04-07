use std::{future::Future, pin::Pin};

use actix_web::{dev::Payload, error::ErrorUnauthorized, web::BytesMut, HttpMessage};
use actix_web::{
    get, post, web, App, Error, FromRequest, HttpRequest, HttpResponse, HttpServer, Responder,
};
// use futures_util::StreamExt;
use hex::FromHex;
use ring::{constant_time::verify_slices_are_equal, hmac};
use std::iter::Iterator;

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

const X_GITHUB_EVENT: &str = "X-Github-Event";
const X_HUB_SIGNATURE: &str = "X-Hub-Signature-256";
const SECRET: &str = &std::env::args().nth(1).unwrap();

// async fn webhook(req: HttpRequest, body: String) -> impl Responder {
//     let event = req.headers().get(X_GITHUB_EVENT);
//     let signature = req.headers().get(X_HUB_SIGNATURE);
// }
struct Authorized;

impl FromRequest for Authorized {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let signature = &req
            .headers()
            .get(X_HUB_SIGNATURE)
            .unwrap()
            .to_str()
            .unwrap();
        // let mut bytes = web::BytesMut::new();
        // while let Some(item) = body.next() {
        //     bytes.extend_from_slice(&item?);
        // }

        // if verify_signature(SECRET, &payload, signature) {
        //     ok(Authorized)
        // } else {
        //     err(ErrorUnauthorized("not authorized"))
        // }
        Box::pin(async move {
            let mut body = BytesMut::new();
            // let mut stream = req.take_payload();
            while let Some(chunk) = payload.next().await {
                body.extend_from_slice(&chunk?);
            }
            Ok(Authorized)
        })
    }
}

pub fn verify_signature(secret: &str, payload: &str, signature: &str) -> bool {
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
// pub async fn init<A>(addr: A) -> std::io::Result<()>
// where
//     A: net::ToSocketAddrs,
// {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(echo)
//             .route("/hey", web::get().to(manual_hello))
//     })
//     .bind(addr)?
//     .run()
//     .await
// }
