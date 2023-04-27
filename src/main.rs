extern crate diesel;
use actix_web::{web, App, HttpServer};
use pos::apis::product_api::product_service;
use pos::config::db_connection::get_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(product_service())
            .app_data(web::Data::new(get_pool().clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
