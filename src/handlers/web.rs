use ntex_files::NamedFile;
use ntex::web::{HttpResponse, Error};
use crate::errors::AppError;

pub async fn index() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("public/index.html")?)
}

// Route: GET "/health_check"
pub async fn health_check() -> Result<HttpResponse, AppError> {
    error!("In health check");
    Ok(HttpResponse::Ok().finish())
}
