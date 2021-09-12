use actix_web::{middleware, web, App, HttpServer};
use forum_rust::config::MyConfig as Config;
use forum_rust::headers;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let config = Config::new();
    let pool = r2d2::Pool::new(config.sqlite_manager).unwrap();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(headers::index))
            .route("/testdb", web::get().to(headers::testdb))
    })
    .bind((config.server_address, config.server_port))?
    .run()
    .await
}
