use actix_web::{
    post,
    web::{self},
    HttpResponse, Responder, Scope,
};
use serde::{Deserialize, Serialize};

use crate::{
    config::{
        db_connection::DbPool,
        jwt::{create_token, Subject},
    },
    models::user::{RegisterUserHandler, User},
};

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    message: String,
    token: String,
}

#[post("/login")]
async fn login() -> HttpResponse {
    let token = create_token(Subject {
        id: 1,
        name: "Brandon".to_string(),
    });

    HttpResponse::Ok().json(TokenResponse {
        token,
        message: String::from("Success"),
    })
}

#[post("/register")]
async fn register(
    pool: web::Data<DbPool>,
    user_data: web::Json<RegisterUserHandler>,
) -> impl Responder {
    let mut conn = pool.get().unwrap();

    match web::block(move || User::register_new_user(&mut conn, &user_data)).await {
        Ok(result) => match result {
            Ok(result) => match result {
                Ok(_) => HttpResponse::Created().body("User created"),
                Err(err) => HttpResponse::BadRequest().body(err.to_string()),
            },
            Err(err) => HttpResponse::BadRequest().body(err.to_string()),
        },
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

pub fn authentication_service() -> Scope {
    web::scope("/auth").service(login).service(register)
}
