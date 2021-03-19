use sqlx::postgres::PgPool;
use ntex_files as fs;
use ntex::web::{types, Error};
use crate::model::User;

pub async fn index() -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open("public/index.html")?)
}


// async fn user(
//     data: types::Json<User>,
//     dbpool: types::Data<sqlx::PgPool>,
// ) -> Result<HttpResponse, Error> {

//     Ok(HttpResponse::Ok().body(rendered))
// }
