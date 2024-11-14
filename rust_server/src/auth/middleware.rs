// src/auth/middleware.rs
use std::pin::Pin;
use std::task::{Context, Poll};
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use actix_web::error::ErrorUnauthorized;
use futures::future::{ok, Future, Ready};
use crate::auth::jwt::verify_token;

pub struct AuthenticationMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthenticationMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddlewareService { service })
    }
}

pub struct AuthenticationMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // 인증이 필요없는 경로들
        let public_paths = [
            "/api/v1/auth/login",
            "/api/v1/auth/register",
            "/api/v1/health",
            "/ws"
        ];

        if public_paths.iter().any(|path| req.path().starts_with(path)) {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

        // 인증 헤더 검사
        let auth_header = match req.headers().get("Authorization") {
            Some(header) => header,
            None => {
                return Box::pin(async move {
                    Err(ErrorUnauthorized("Missing authorization header"))
                });
            }
        };

        // 헤더 값을 문자열로 변환
        let auth_str = match auth_header.to_str() {
            Ok(str) => str,
            Err(_) => {
                return Box::pin(async move {
                    Err(ErrorUnauthorized("Invalid authorization header format"))
                });
            }
        };

        // Bearer 토큰 형식 확인
        if !auth_str.starts_with("Bearer ") {
            return Box::pin(async move {
                Err(ErrorUnauthorized("Invalid authorization header format"))
            });
        }

        // 토큰 추출 및 검증
        let token = &auth_str[7..];
        match verify_token(token) {
            Ok(claims) => {
                req.extensions_mut().insert(claims);
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            Err(e) => {
                let error_message = format!("Token verification failed: {}", e);
                Box::pin(async move {
                    Err(ErrorUnauthorized(error_message))
                })
            }
        }
    }
}