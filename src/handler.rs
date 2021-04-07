use std::net;

use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

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

// async fn webhook(req: HttpRequest, body: String) -> impl Responder {
//     let event = req.headers().get(X_GITHUB_EVENT);
//     let signature = req.headers().get(X_HUB_SIGNATURE);
// }

type HmacSha256 = Hmac<Sha256>;
pub fn verify_signature(secret: &str, payload: &str, signature: &str) -> bool {
    let signature = signature[7..signature.len()].as_bytes();
    let mut mac = HmacSha256::new_varkey(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(payload.as_bytes());
    let result = mac.clone().finalize().into_bytes();
    println!("{:02x?}", result);
    println!("{:?}", mac.verify(signature));
    false
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
