#[macro_use]
extern crate log;

extern crate chrono;
extern crate serde;

pub mod config;
mod errors;
pub mod handlers;
mod logger;
mod middlewares;
mod models;
mod repositories;
mod routes;

use crate::config::Config;
use ntex::web::middleware::Logger;
use ntex::web::{App, HttpServer};
use ntex::http::header;
use color_eyre::Result;
use sqlx::{Postgres, Pool};
use ntex_files;

#[derive(Debug, Clone)]
pub struct AppState {
    pub jwt_secret_key: String,
    pub jwt_lifetime: i64,
}

pub async fn run(settings: Config, db_pool: Pool<Postgres>) -> Result<()> {
    // Logger
    logger::init(settings.server_log);

    let data = AppState {
        jwt_secret_key: settings.jwt_secret_key.clone(),
        jwt_lifetime: settings.jwt_lifetime,
    };

    // Start server
    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .data(data.clone())
            .wrap(Logger::new("%s | %r | %Ts | %{User-Agent}i | %a"))
            .wrap(middlewares::timer::Timer)
            .wrap(middlewares::request_id::RequestId)
            .service((
                ntex_files::Files::new("/static", "static/"),
            ))
            .configure(routes::web)
            .configure(routes::api)
    })
    .bind(format!("{}:{}", settings.server_url, settings.server_port))?
    .run()
    .await?;

    Ok(())
}