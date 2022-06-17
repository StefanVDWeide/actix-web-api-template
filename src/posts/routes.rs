use crate::errors::handlers::CustomError;
use crate::posts::models::JsonInputPost;
use crate::posts::{
    add_single_post_helper, delete_single_post_helper, get_all_posts_by_user_helper,
    get_all_posts_helper, get_single_post_helper,
};
use crate::Pool;
use actix_web::{delete, get, http::StatusCode, post, web, HttpResponse};

#[get("/get/posts")]
async fn get_all_posts(db: web::Data<Pool>) -> Result<HttpResponse, CustomError> {
    let posts = web::block(move || get_all_posts_helper(db))
        .await
        .map_err(|_| CustomError::new(500, format!("Internal error")))?;

    match posts {
        Ok(posts) => Ok(HttpResponse::Ok().json(posts)),
        Err(err) => {
            Ok(HttpResponse::build(StatusCode::from_u16(err.error_status_code).unwrap()).json(err))
        }
    }
}

#[get("/get/post/{id}")]
async fn get_single_post(
    db: web::Data<Pool>,
    post_id: web::Path<i32>,
) -> Result<HttpResponse, CustomError> {
    let post = web::block(move || get_single_post_helper(db, post_id.into_inner()))
        .await
        .map_err(|_| CustomError::new(500, format!("Internal error")))?;

    match post {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(err) => {
            Ok(HttpResponse::build(StatusCode::from_u16(err.error_status_code).unwrap()).json(err))
        }
    }
}

#[get("/get/posts/user/{id}")]
async fn get_all_posts_by_user(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, CustomError> {
    let posts = web::block(move || get_all_posts_by_user_helper(db, user_id.into_inner()))
        .await
        .map_err(|_| CustomError::new(500, format!("Internal error")))?;

    match posts {
        Ok(posts) => Ok(HttpResponse::Ok().json(posts)),
        Err(err) => {
            Ok(HttpResponse::build(StatusCode::from_u16(err.error_status_code).unwrap()).json(err))
        }
    }
}

#[post("/post/post")]
async fn post_new_post(
    db: web::Data<Pool>,
    post: web::Json<JsonInputPost>,
) -> Result<HttpResponse, CustomError> {
    let post = web::block(move || add_single_post_helper(db, post))
        .await
        .map_err(|_| CustomError::new(500, format!("Internal error")))?;

    match post {
        Ok(post) => Ok(HttpResponse::Ok().json(post)),
        Err(err) => {
            Ok(HttpResponse::build(StatusCode::from_u16(err.error_status_code).unwrap()).json(err))
        }
    }
}

#[delete("/delete/post/{id}")]
async fn delete_single_post(
    db: web::Data<Pool>,
    req: web::Path<i32>,
) -> Result<HttpResponse, CustomError> {
    let post_id = req.into_inner();

    let result = web::block(move || delete_single_post_helper(db, post_id))
        .await
        .map_err(|_| CustomError::new(500, format!("Internal error")))?;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().json("Post deleted")),
        Err(err) => {
            Ok(HttpResponse::build(StatusCode::from_u16(err.error_status_code).unwrap()).json(err))
        }
    }
}
