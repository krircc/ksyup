use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::SystemTime;

use ntex::http::header;
use ntex::service::{Service, Transform};
use ntex::web::dev::{WebRequest, WebResponse};
use color_eyre::Result;
use futures::future::{ok, Ready};
use futures::Future;

pub struct Timer;

impl<S, E> Transform<S> for Timer
where
    S: Service<Request = WebRequest<E>, Response = WebResponse>,
    S::Future: 'static,
    E: 'static,
{
    type Request = WebRequest<E>;
    type Response = WebResponse;
    type Error = S::Error;
    type Transform = TimerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(TimerMiddleware { service })
    }
}

pub struct TimerMiddleware<S> {
    service: S,
}

impl<S, E> Service for TimerMiddleware<S>
where
    S: Service<Request = WebRequest<E>, Response = WebResponse>,
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
        let now = SystemTime::now();
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            let elapsed_result = now.elapsed();
            if let Ok(elapsed) = elapsed_result {
                if let Ok(name) = header::HeaderName::from_lowercase(b"x-process-time-s") {
                    let elapsed_sec = elapsed.as_micros() as f32 / 1_000_000f32;
                    if let Ok(value) = header::HeaderValue::from_str(&format!("{}", elapsed_sec)) {
                        res.headers_mut().insert(name, value);
                    }
                }
            }

            Ok(res)
        })
    }
}
