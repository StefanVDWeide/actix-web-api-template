use crate::errors::handlers::CustomError;
use crate::users::JsonInputUser;
use crate::users::{add_single_user, delete_single_user, get_all_users_helper, get_single_user};
use crate::Pool;
use actix_web::{delete, get, http::StatusCode, post, web, HttpResponse};

#[get("/get/users")]
async fn get_all_users(db: web::Data<Pool>) -> Result<HttpResponse, CustomError> {
    let users = web::block(move || get_all_users_helper(db))
        .await
        .map_err(|_| CustomError::new(500, format!("Internal error")))?;

    match users {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(err) => {
            Ok(HttpResponse::build(StatusCode::from_u16(err.error_status_code).unwrap()).json(err))
        }
    }
}

#[get("/get/user/{id}")]
async fn get_user(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, CustomError> {
    let user = web::block(move || get_single_user(db, user_id.into_inner()))
        .await
        .map_err(|_| CustomError::new(500, format!("Internal error")))?;

    match user {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(err) => {
            Ok(HttpResponse::build(StatusCode::from_u16(err.error_status_code).unwrap()).json(err))
        }
    }
}

#[post("/post/user")]
async fn post_user(
    db: web::Data<Pool>,
    user: web::Json<JsonInputUser>,
) -> Result<HttpResponse, CustomError> {
    let user = web::block(move || add_single_user(db, user))
        .await
        .map_err(|_| CustomError::new(500, format!("Internal error")))?;

    match user {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(err) => {
            Ok(HttpResponse::build(StatusCode::from_u16(err.error_status_code).unwrap()).json(err))
        }
    }
}

#[delete("/delete/user/{id}")]
async fn deleter_user(
    db: web::Data<Pool>,
    req: web::Path<i32>,
) -> Result<HttpResponse, CustomError> {
    let user_id = req.into_inner();

    let result = web::block(move || delete_single_user(db, user_id))
        .await
        .map_err(|_| CustomError::new(500, format!("Internal error")))?;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().json("User deleted")),
        Err(err) => {
            Ok(HttpResponse::build(StatusCode::from_u16(err.error_status_code).unwrap()).json(err))
        }
    }
}
