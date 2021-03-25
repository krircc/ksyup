use sqlx::{Row, postgres::{PgPool,PgRow}};
use ntex_files as fs;
use ntex::web::{types, HttpResponse, Error};
use crate::model::User;
use futures::stream::TryStreamExt;

pub async fn index() -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open("public/index.html")?)
}


pub async fn user( dbpool: types::Data<sqlx::PgPool> ) -> Result<HttpResponse, Error> {

    let auser = sqlx::query("SELECT * FROM users")
    .map(|row: PgRow| {
        let one: &str = row.try_get(1).unwrap();
        println!("-------{:?}--------------", one);
        // map the row into a user-defined domain type
    }).fetch_all(&**dbpool).await.unwrap();

    Ok(HttpResponse::Ok().json(&0))
}
