use actix_web::{App, HttpServer, middleware, web};
use forum_rust::headers;
use forum_rust::config::MyConfig as Config;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let config = Config::new();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(headers::index))
    })
    .bind((config.server_address, config.server_port))?
    .run()
    .await
}
