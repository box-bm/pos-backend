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
    pub id: usize,
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
