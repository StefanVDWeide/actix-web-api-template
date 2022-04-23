use crate::errors::handlers::CustomError;
use crate::users::User;
use actix_web::{get, post, web, HttpResponse};

#[get("/get/user/{id}")]
async fn get_user(req: web::Path<u32>) -> Result<HttpResponse, CustomError> {
    let user_id = req.into_inner();
    Ok(HttpResponse::Ok().json(user_id))
}

#[post("/post/user")]
async fn post_user(user: web::Json<User>) -> Result<HttpResponse, CustomError> {
    Ok(HttpResponse::Ok().json(user))
}
