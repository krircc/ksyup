use sqlx::postgres::PgPool;
use ntex_files as fs;
use ntex::web::Error;

pub async fn index() -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open("public/index.html")?)
}


// async fn user( pool: web::types::Data<PgPool> ) -> Result<HttpResponse, Error> {

//     let rendered = tmpl
//         .render("index.html.tera", &context)
//         .map_err(error::ErrorInternalServerError)?;

//     Ok(HttpResponse::Ok().body(rendered))
// }