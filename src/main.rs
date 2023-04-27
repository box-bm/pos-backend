extern crate diesel;
use actix_web::{web, App, HttpServer};
use pos::apis::product_api;
use pos::config::db_connection::get_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(product_api::hello)
            .service(product_api::create_product)
            .app_data(web::Data::new(get_pool().clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
