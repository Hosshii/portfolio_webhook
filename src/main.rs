use actix_web::{web, App, HttpServer};
use portfolio_webhook::handler::{self, WebHook};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = WebHook::new("a");
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
