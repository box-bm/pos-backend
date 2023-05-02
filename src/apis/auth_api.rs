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
    models::user::{LoginUserSchema, RegisterUserHandler, User},
};

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    message: String,
    token: String,
}

#[post("/login")]
async fn login(
    pool: web::Data<DbPool>,
    user_credentials: web::Json<LoginUserSchema>,
) -> HttpResponse {
    let mut conn = pool.get().unwrap();

    match web::block(move || User::login(&mut conn, &user_credentials)).await {
        Ok(user_data) => match user_data {
            Ok(user_data) => {
                let token = create_token(Subject {
                    id: user_data.id.to_string(),
                    name: user_data.name,
                });
                HttpResponse::Ok().json(TokenResponse {
                    token,
                    message: String::from("Success"),
                })
            }
            Err(err) => HttpResponse::BadRequest().body(err),
        },
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
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
