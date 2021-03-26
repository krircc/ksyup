//! JWT middleware module

use crate::models::auth;
use crate::repositories::user::UserRepository;
use crate::AppState;
use ntex::service::{Service, Transform};
use ntex::web::{
    HttpResponse,
    types::Data,
    dev::{WebRequest, WebResponse},
};
use ntex::http::{Method, StatusCode};
use color_eyre::Result;
use futures::{
    Future,
    future::{ok, Ready},
};
use sqlx::PgPool;
use std::task::{Context, Poll};
use std::pin::Pin;

pub struct Authentication;

impl<S, E> Transform<S> for Authentication
where
    S: Service<Request = WebRequest<E>, Response = WebResponse> + 'static,
    S::Future: 'static,
    E: 'static,
{
    type Request = WebRequest<E>;
    type Response = WebResponse;
    type Error = S::Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware {
            service
        })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, E> Service for AuthenticationMiddleware<S>
where
    S: Service<Request = WebRequest<E>, Response = WebResponse> + 'static,
    S::Future: 'static,
    E: 'static,
{
    type Request = WebRequest<E>;
    type Response = WebResponse;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: WebRequest<E>) -> Self::Future {
        let service_cloned = self.service.clone();
        let mut is_authorized = false;
        let mut user_id = String::new();

        if Method::OPTIONS == *req.method() {
            //跨域在发送复杂post请求时，会先发送一个option的请求，所以在jwt过滤器中，需要先将options请求放掉
            // https://my.oschina.net/u/4290053/blog/4184173
            Box::pin(async move {
                return true;
            });
        } 
        
        if let Some(app_state) = req.app_data::<Data<AppState>>() {
            let secret_key = &app_state.jwt_secret_key;
            let token = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
                .and_then(|h| {
                    let words = h.split("Bearer").collect::<Vec<&str>>();
                    words.get(1).map(|w| w.trim())
                });

            is_authorized = match token {
                Some(token) => {
                    let claims = auth::JWT::parse(token.to_owned(), secret_key.to_owned());
                    match claims {
                        Ok(claims) => {
                            user_id = claims.user_id;
                            true
                        }
                        _ => false,
                    }
                }
                _ => false,
            };
        }

        Box::pin(async move {
            if is_authorized {
                // Check if user is still valid
                is_authorized = match req.app_data::<Data<PgPool>>() {
                    Some(pool) => match UserRepository::get_by_id(pool.get_ref(), user_id).await {
                        Ok(user) => user.is_some(),
                        _ => false,
                    },
                    None => false,
                };
            }

            if is_authorized {
                service_cloned.call(req).await
            } else {
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(&crate::errors::AppErrorMessage {
                            code: StatusCode::UNAUTHORIZED.as_u16(),
                            message: "Unauthorized".to_owned(),
                        })
                        .into_body(),
                ))
            }
        })
    }
}
