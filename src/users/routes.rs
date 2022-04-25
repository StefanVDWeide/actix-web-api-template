use crate::errors::handlers::CustomError;
use crate::users::JsonInputUser;
use crate::users::{add_single_user, get_all_users_helper};
use crate::Pool;
use actix_web::{delete, get, post, web, HttpResponse};

#[get("/get/users")]
async fn get_all_users(db: web::Data<Pool>) -> Result<HttpResponse, CustomError> {
    Ok(web::block(move || get_all_users_helper(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| CustomError::InternalError)?)
}

#[get("/get/user/{id}")]
async fn get_user(req: web::Path<u32>) -> Result<HttpResponse, CustomError> {
    let user_id = req.into_inner();
    Ok(HttpResponse::Ok().json(user_id))
}

#[post("/post/user")]
async fn post_user(
    db: web::Data<Pool>,
    user: web::Json<JsonInputUser>,
) -> Result<HttpResponse, CustomError> {
    Ok(web::block(move || add_single_user(db, user))
        .await
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|_| CustomError::InternalError)?)
}

#[delete("/delete/user/{id}")]
async fn deleter_user(req: web::Path<u32>) -> Result<HttpResponse, CustomError> {
    let user_id = req.into_inner();
    Ok(HttpResponse::Ok().json(user_id))
}
