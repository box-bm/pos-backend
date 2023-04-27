use crate::config::db_connection::DbPool;
use crate::models::product::{NewProductHandler, Product};
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/products")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// #[get("/products/{product_id}")]
// async fn get_product_handler(
//     pool: web::Data<DbPool>,
//     product_id: web::Path<i32>,
// ) -> impl Responder {
// }

#[post("/products")]
pub async fn create_product(
    pool: web::Data<DbPool>,
    product_data: web::Json<NewProductHandler>,
) -> impl Responder {
    let mut conn = pool.get().unwrap();

    match web::block(move || Product::create_product(&mut conn, &product_data)).await {
        Ok(data) => HttpResponse::Ok().json(data.ok()),
        Err(err) => HttpResponse::Ok().body(err.to_string()),
    }
}
