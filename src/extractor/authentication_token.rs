use actix_web::{
    dev::Payload, error::ErrorUnauthorized, Error as ActixWebError, FromRequest, HttpRequest,
};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

use crate::config::jwt::{read_token_from_http, Subject};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationToken {
    subject: Subject,
}

impl FromRequest for AuthenticationToken {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let token_result = read_token_from_http(req);

        match token_result {
            Ok(claims) => ready(Ok(AuthenticationToken {
                subject: claims.subject,
            })),
            Err(_e) => ready(Err(ErrorUnauthorized("Invalid authentication token sent!"))),
        }
    }
}
