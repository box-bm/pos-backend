use actix_web::HttpRequest;
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
};
use serde::{Deserialize, Serialize};

use super::env::get_enviroment_variable;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub subject: Subject,
    pub exp: usize,
    pub sub: String,
    pub company: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subject {
    pub id: String,
    pub name: String,
}

pub fn create_token(subject: Subject) -> String {
    let secret = get_enviroment_variable(String::from("JSWKEY"));
    let exp = (Utc::now() + Duration::days(2)).timestamp() as usize;
    let claims = Claims {
        subject,
        sub: "Customers".to_string(),
        exp,
        company: "Reactive Software".to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

pub fn decode_token(token: String) -> Result<TokenData<Claims>, JwtError> {
    let secret = get_enviroment_variable(String::from("JSWKEY"));
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
}

pub fn read_token_from_http(req: &HttpRequest) -> Result<Claims, String> {
    let auth_header = req.headers().get(actix_web::http::header::AUTHORIZATION);
    if auth_header.is_none() {
        return Err("No authentication token sent".to_string());
    }

    let auth_token = auth_header.unwrap().to_str().unwrap_or("").to_string();

    if auth_token.is_empty() {
        return Err("Authentication token has foreign chars!".to_string());
    }

    match decode_token(auth_token) {
        Ok(token) => Ok(token.claims),
        Err(_e) => Err("Invalid authentication token sent!".to_string()),
    }
}
