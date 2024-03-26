use std::{
    future::{ready, Ready},
    time::{SystemTime, UNIX_EPOCH},
};

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use actix_web::{http, HttpResponse};
use futures_util::{future::LocalBoxFuture, FutureExt, TryFutureExt};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::models::Claims;

pub struct Authorization {
    jwt_secret: String,
}

impl Authorization {
    pub fn new(jwt_secret: &str) -> Self {
        Self {
            jwt_secret: jwt_secret.to_string(),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for Authorization
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthorizationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthorizationMiddleware {
            service,
            jwt_secret: self.jwt_secret.clone(),
        }))
    }
}

pub struct AuthorizationMiddleware<S> {
    service: S,
    jwt_secret: String,
}

impl<S, B> Service<ServiceRequest> for AuthorizationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if let Some(auth_header) = req.headers().get(http::header::AUTHORIZATION) {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    if let Ok(token) = decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(self.jwt_secret.as_ref()),
                        &Validation::default(),
                    ) {
                        let now = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .expect("WTF!?")
                            .as_secs() as usize;

                        if now < token.claims.exp {
                            return self
                                .service
                                .call(req)
                                .map_ok(ServiceResponse::map_into_left_body)
                                .boxed_local();
                        }
                    }
                }
            }
        }

        Box::pin(async {
            Ok(req.into_response(HttpResponse::Unauthorized().finish().map_into_right_body()))
        })
    }
}
