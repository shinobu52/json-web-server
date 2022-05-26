use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use std::env;

#[macro_use]
extern crate diesel;
use diesel::{pg::PgConnection, r2d2::{ConnectionManager, Pool}};

mod schema;
mod handlers;
mod model;
mod db;

// アプリケーションで持ち回る状態
#[derive(Clone)]
pub struct Server {
    pool: Pool<ConnectionManager<PgConnection>>
}

impl Server {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool");

        Server { pool }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // on Logger Level INFO
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let server = Server::new();

    HttpServer::new(move || {
        use handlers::*;

        // NOTE: https://stackoverflow.com/questions/57457202/the-correct-return-type-of-create-app
        // app_dataでDataを引き渡した結果としてApp<AppEntry>型が返る
        // App<AppEntry>型をApp<Server>に返る術がわからなかった
        // また、URLのリンク先に書かれているようにactix-webのAppEntryなどの型は外部公開されていない
        App::new()
            .wrap(Logger::default()) // Log出力
            .app_data(web::Data::new(server.clone()))
            .route("/", web::get().to(index))
            .route("/logs", web::get().to(handle_get_logs))
            .route("/logs", web::post().to(handle_post_logs))
            .route("/csv", web::get().to(handle_get_csv))
            // .route("/csv", web::post().to(handle_post_csv))
    })
    .bind("localhost:3000")
    .expect("Can't bind to port 3000")
    .run()
    .await
}
