//! Errors handlers module

use ntex::web::WebResponseError;
use ntex::web::dev;
use ntex::web::error;
use ntex::http::{
    header,
    body::Body,
    body::ResponseBody,
    StatusCode
};
use color_eyre::Result;
use serde_json::json;

fn render_error<B>(mut res: dev::WebResponse, code: u16, message: String) -> WebResponseError<B> {
    let err = json!(crate::errors::AppErrorMessage { code, message });

    res.request();
    res.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    res = res.map_body(|_, _| ResponseBody::Body(Body::from(err)).into_body());

    WebResponseError::Response(res)
}

/// Render 401 error
pub fn render_401<B>(res: dev::WebResponse) -> Result<WebResponseError<B>, error::Error> {
    Ok(render_error(
        res,
        StatusCode::UNAUTHORIZED.as_u16(),
        String::from("Unauthorized"),
    ))
}

/// Render 403 error
pub fn render_403<B>(res: dev::WebResponse) -> Result<WebResponseError<B>, error::Error> {
    Ok(render_error(
        res,
        StatusCode::FORBIDDEN.as_u16(),
        String::from("Forbidden"),
    ))
}

/// Render 408 error
pub fn render_408<B>(res: dev::WebResponse) -> Result<WebResponseError<B>, error::Error> {
    Ok(render_error(
        res,
        StatusCode::REQUEST_TIMEOUT.as_u16(),
        String::from("Request Time-out"),
    ))
}

/// Render 502 error
pub fn render_502<B>(res: dev::WebResponse) -> Result<WebResponseError<B>, error::Error> {
    Ok(render_error(
        res,
        StatusCode::BAD_GATEWAY.as_u16(),
        String::from("Bad Gateway"),
    ))
}

/// Render 503 error
pub fn render_503<B>(res: dev::WebResponse) -> Result<WebResponseError<B>, error::Error> {
    Ok(render_error(
        res,
        StatusCode::SERVICE_UNAVAILABLE.as_u16(),
        String::from("Service Unavailable"),
    ))
}

/// Render 504 error
pub fn render_504<B>(res: dev::WebResponse) -> Result<WebResponseError<B>, error::Error> {
    Ok(render_error(
        res,
        StatusCode::GATEWAY_TIMEOUT.as_u16(),
        String::from("Gateway Time-out"),
    ))
}
