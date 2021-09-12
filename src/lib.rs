pub mod config {
    use r2d2_sqlite::SqliteConnectionManager;
    pub struct MyConfig {
        pub server_address: String,
        pub server_port: u16,
        pub sqlite_manager: SqliteConnectionManager,
    }
    impl MyConfig {
        pub fn new() -> Self {
            let mut settings = config::Config::default();
            settings.merge(config::File::with_name("Settings")).unwrap();
            let server_address: String = settings.get("server.address").unwrap();
            let server_port: u16 = settings.get("server.port").unwrap();
            let sqlite_path: String = settings.get("db.sqlite_path").unwrap();
            let sqlite_manager = SqliteConnectionManager::file(sqlite_path);
            Self {
                server_address,
                server_port,
                sqlite_manager,
            }
        }
    }
}

pub mod db {
    use r2d2::PooledConnection;
    use r2d2_sqlite::SqliteConnectionManager;
    use r2d2_sqlite::rusqlite::Error;
    pub fn useradd(conn: PooledConnection<SqliteConnectionManager>, id: u32, name: String) -> Result<usize,Error> {
        conn.execute(
            "INSERT INTO users (id, name) VALUES ($1, $2)",
            [id.to_string(), name],
        )
    }
}
pub mod headers {
    use actix_web::{web, HttpResponse};
    use r2d2::Pool;
    use r2d2_sqlite::SqliteConnectionManager;

    use crate::db;
    pub async fn index() -> HttpResponse {
        HttpResponse::Ok().body("Hello world!")
    }
    pub async fn testdb(pool: web::Data<Pool<SqliteConnectionManager>>) -> HttpResponse {
        let conn = pool.get().unwrap();
        web::block(|| {
            db::useradd(conn, 1, "hello".to_string())
        }).await.unwrap();
        HttpResponse::Ok().into()
    }
}
