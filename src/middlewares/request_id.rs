use std::pin::Pin;
use std::task::{Context, Poll};

use ntex::http::header;
use ntex::service::{Service, Transform};
use ntex::web::{dev::{WebRequest, WebResponse}, Error};
use color_eyre::Result;
use futures::future::{ok, Ready};
use futures::Future;
use uuid::Uuid;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct RequestId;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, E> Transform<S> for RequestId
where
    S: Service<Request = WebRequest<E>, Response = WebResponse>,
    S::Future: 'static,
{
    type Request = WebRequest<E>;
    type Response = WebResponse;
    type Error = S::Error;
    type Transform = RequestIdMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequestIdMiddleware { service })
    }
}

pub struct RequestIdMiddleware<S> {
    service: S,
}

impl<S, E> Service for RequestIdMiddleware<S>
where
    S: Service<Request = WebRequest<E>, Response = WebResponse>,
    S::Future: 'static,
{
    type Request = WebRequest<E>;
    type Response = WebResponse;
    type Error = S::Error;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: WebRequest<E>) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            let request_id = Uuid::new_v4().to_string();
            if let Ok(name) = header::HeaderName::from_lowercase(b"x-request-id") {
                if let Ok(value) = header::HeaderValue::from_str(&request_id) {
                    res.headers_mut().insert(name, value);
                }
            }

            Ok(res)
        })
    }
}
