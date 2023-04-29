use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    http, Error as ActixWebError,
};
use futures_util::future::LocalBoxFuture;

use crate::config::jwt::decode_token;

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixWebError>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixWebError;
    type InitError = ();
    type Transform = AuthenticationTokenMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationTokenMiddleware { service }))
    }
}

pub struct AuthenticationTokenMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationTokenMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixWebError>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixWebError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get(http::header::AUTHORIZATION);

        if auth_header.is_none() {
            return Box::pin(async move { Err(ErrorUnauthorized("No authentication token sent")) });
        }

        let auth_token = auth_header.unwrap().to_str().unwrap_or("").to_string();

        if auth_token.is_empty() {
            return Box::pin(async move { Err(ErrorUnauthorized("Token is empty")) });
        }

        let fut = self.service.call(req);

        let decode = decode_token(auth_token);

        Box::pin(async move {
            let res = fut.await?;

            match decode {
                Ok(_token) => Ok(res),
                Err(_) => Err(ErrorUnauthorized("Invalid auth token")),
            }
        })
    }
}
