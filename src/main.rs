use actix_web::middleware::Logger;
use actix_web::{middleware, web, App, HttpServer};
use pos::apis::{auth_api, product_api};
use pos::config::db_connection::get_pool;
use pos::middleware::authentication;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(middleware::NormalizePath::default())
            .service(auth_api::authentication_service())
            .service(
                web::scope("/api")
                    .wrap(authentication::Authentication)
                    .service(product_api::product_service()),
            )
            .app_data(web::Data::new(get_pool().clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
