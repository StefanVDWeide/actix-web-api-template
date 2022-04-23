use actix_web::{web, App, HttpServer};

mod errors;
mod users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at: 127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
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
