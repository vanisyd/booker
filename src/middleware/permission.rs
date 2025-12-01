use std::future::{ready, Ready};
use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, error, Error, HttpMessage};
use actix_web::http::{header};
use futures_util::future::LocalBoxFuture;
use crate::services::auth::decode_token;

pub struct Permission;

impl <S,B> Transform<S, ServiceRequest> for Permission
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = PermissionMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PermissionMiddleware { service }))
    }
}

pub struct PermissionMiddleware<S> {
    service: S
}

impl<S, B> Service<ServiceRequest> for PermissionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        if let Some(token) = req.headers().get(header::AUTHORIZATION) {
            let claims = decode_token(token.to_str().unwrap().to_string());
            req.extensions_mut().insert(claims);
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;

                Ok(res)
            })
        } else {
            Box::pin(async move {
                Err(error::ErrorUnauthorized("Token is not provided"))
            })
        }
    }
}