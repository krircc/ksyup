use sqlx::postgres::PgPool;
use ntex_files as fs;
use ntex::web::{types, HttpResponse, Error};
use crate::model::User;

pub async fn index() -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open("public/index.html")?)
}


pub async fn user( dbpool: types::Data<sqlx::PgPool> ) -> Result<HttpResponse, Error> {

    let auser = sqlx::query("SELECT * FROM users").fetch(&**dbpool);

    // println!("-------{:?}--------------", &auser.email);
    Ok(HttpResponse::Ok().json(&0))
}
