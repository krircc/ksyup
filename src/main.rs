use sqlx::postgres::PgPool;
use std::{env, io};
use ntex::{self, web};
use ntex_files as fs;

mod api;
mod model;

#[ntex::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "ksyup=info");

    let pool = PgPool::connect("postgres://dbuser:password@localhost:5432/demodb")
               .await.expect("unable to connect to database");
   
    web::HttpServer::new(move || {
        web::App::new()
            .data(pool.clone())
            .wrap(web::middleware::Logger::default())
            .service((
                web::resource("/").route(web::get().to(api::index)),
                // web::resource("/user").route(web::get().to(api::user)),
                fs::Files::new("/static", "static/"),
            ))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

