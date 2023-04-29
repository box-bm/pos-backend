use actix_web::{get, web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};

use crate::config::jwt::{create_token, Subject};

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    message: String,
    token: String,
}

#[get("/login")]
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

pub fn authentication_service() -> Scope {
    web::scope("/auth").service(login)
}
