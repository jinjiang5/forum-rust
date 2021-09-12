use actix_web::{App, HttpServer, middleware, web};
use forum_rust::headers;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(headers::index))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
