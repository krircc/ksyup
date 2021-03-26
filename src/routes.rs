//! List all server routes

use crate::{middlewares, handlers};
use ntex::web;

/// Defines Web's routes
pub fn web(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(handlers::web::index))
    .route("/health-check", web::get().to(handlers::web::health_check));
}

/// Defines API's routes
pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .route("/login", web::post().to(handlers::users::login))
            .route("/register", web::post().to(handlers::users::register))
            .service(
                web::scope("/users")
                    .wrap(middlewares::auth::Authentication)
                    .route("", web::get().to(handlers::users::get_all))
                    .route("/{id}", web::get().to(handlers::users::get_by_id))
                    .route("/delete/{id}", web::delete().to(handlers::users::delete))
                    .route("/put/{id}", web::put().to(handlers::users::update)),
            ),
    );
}
