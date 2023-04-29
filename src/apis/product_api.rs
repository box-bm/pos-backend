use crate::config::db_connection::DbPool;
use crate::middleware;
use crate::models::product::{NewProductHandler, Product};
use actix_service::ServiceFactory;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("")]
async fn get_products(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().unwrap();
    match web::block(move || Product::get_all_products(&mut conn)).await {
        Ok(data) => HttpResponse::Ok().json(data.ok()),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

#[get("/{product_id}")]
async fn get_product(pool: web::Data<DbPool>, product_id: web::Path<i32>) -> impl Responder {
    let mut conn = pool.get().unwrap();
    match web::block(move || Product::get_product(&mut conn, &product_id)).await {
        Ok(data) => HttpResponse::Ok().json(data.ok()),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

#[post("")]
async fn create_product(
    pool: web::Data<DbPool>,
    product_data: web::Json<NewProductHandler>,
) -> impl Responder {
    let mut conn = pool.get().unwrap();

    match web::block(move || Product::create_product(&mut conn, &product_data)).await {
        Ok(data) => HttpResponse::Ok().json(data.ok()),
        Err(err) => HttpResponse::Ok().body(err.to_string()),
    }
}

pub fn product_service() -> actix_web::Scope<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    web::scope("/products")
        .service(get_product)
        .service(get_products)
        .service(create_product)
        .wrap(middleware::authentication_token::Authentication)
}
