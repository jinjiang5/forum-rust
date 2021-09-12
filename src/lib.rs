pub mod config {
    pub struct MyConfig {
        pub server_address:String,
        pub server_port:u16,
    }
    impl MyConfig {
        pub fn new() -> Self {
            let mut settings = config::Config::default();
            settings.merge(config::File::with_name("Settings")).unwrap();
            let server_address: String = settings.get("server.address").unwrap();
            let server_port: u16 = settings.get("server.port").unwrap();
            Self {
                server_address,
                server_port
            }
        }
    }
}
pub mod headers {
    use actix_web::HttpResponse;
    pub fn index() -> HttpResponse {
        HttpResponse::Ok().body("Hello world!")
    }
}
