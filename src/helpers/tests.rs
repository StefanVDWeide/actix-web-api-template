use crate::errors;
use actix_web::web;
extern crate diesel;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub fn test_app_factory(cfg: &mut web::ServiceConfig) {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a custom type for database connection pooling
    pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    cfg.app_data(web::Data::new(pool.clone()));
    cfg.app_data(web::JsonConfig::default().error_handler(errors::handlers::json_error_handler));
    cfg.app_data(web::PathConfig::default().error_handler(errors::handlers::path_error_handler));
}
