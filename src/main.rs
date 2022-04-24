use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;

mod errors;
mod users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at: 127.0.0.1:8080");

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .app_data(
                web::JsonConfig::default().error_handler(errors::handlers::json_error_handler),
            )
            .app_data(
                web::PathConfig::default().error_handler(errors::handlers::path_error_handler),
            )
            .service(users::get_user)
            .service(users::post_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
