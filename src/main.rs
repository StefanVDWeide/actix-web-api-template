#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;

mod errors;
mod helpers;
mod posts;
mod schema;
mod users;

// Create a custom type for database connection pooling
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at: 127.0.0.1:8080");

    dotenv::dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(
                web::JsonConfig::default().error_handler(errors::handlers::json_error_handler),
            )
            .app_data(
                web::PathConfig::default().error_handler(errors::handlers::path_error_handler),
            )
            .service(users::get_user)
            .service(users::get_all_users)
            .service(users::post_user)
            .service(users::delete_user)
            .service(posts::get_all_posts)
            .service(posts::get_single_post)
            .service(posts::get_all_posts_by_user)
            .service(posts::post_new_post)
            .service(posts::delete_single_post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
