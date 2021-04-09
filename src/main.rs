use std::env;

use actix_web::{web, App, HttpServer};
use portfolio_webhook::handler;
use portfolio_webhook::webhook::WebHook;

#[macro_use]
extern crate log;

const ENV_TRAQ_WEBHOOK_ID: &str = "TRAQ_WEBHOOK_ID";
const ENV_TRAQ_WEBHOOK_SECRET: &str = "TRAQ_WEBHOOK_SECRET";
const ENV_GITHUB_WEBHOOK_SECRET: &str = "GITHUB_WEBHOOK_SECRET";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let traq_webhook_id = env::var(ENV_TRAQ_WEBHOOK_ID)
        .expect(&format!("{} is must not be empty", ENV_TRAQ_WEBHOOK_ID));
    let traq_webhook_secret = env::var(ENV_TRAQ_WEBHOOK_SECRET)
        .expect(&format!("{} is must not be empty", ENV_TRAQ_WEBHOOK_SECRET));
    let github_webhook_secret = env::var(ENV_GITHUB_WEBHOOK_SECRET).expect(&format!(
        "{} is must not be empty",
        ENV_GITHUB_WEBHOOK_SECRET
    ));

    let data = WebHook::new(github_webhook_secret, traq_webhook_secret, traq_webhook_id);

    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            // .service(handler::webhook)
            .route("/webhook", web::post().to(handler::webhook))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
