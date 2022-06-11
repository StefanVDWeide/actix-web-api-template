use crate::diesel::RunQueryDsl;
use crate::errors::handlers::CustomError;
use crate::posts::{JsonInputPost, NewPost, Post};
// Importing ID seperatly since we have an ID column in both the Users and Posts table. This resolved an ambiguousness error
use crate::schema::posts::dsl::id;
use crate::schema::posts::dsl::*;
use crate::schema::users::dsl::*;
use crate::users::User;
use crate::Pool;
use actix_web::web;
use diesel::dsl::{delete, insert_into};
use diesel::prelude::*;

pub fn get_all_posts_helper(pool: web::Data<Pool>) -> Result<Vec<Post>, CustomError> {
    let conn = pool.get().unwrap();
    let items = posts.load::<Post>(&conn)?;
    Ok(items)
}

pub fn get_single_post_helper(pool: web::Data<Pool>, post_id: i32) -> Result<Post, CustomError> {
    let conn = pool.get().unwrap();
    let post = posts.find(post_id).get_result::<Post>(&conn)?;
    Ok(post)
}

pub fn get_all_posts_by_user_helper(
    pool: web::Data<Pool>,
    owner_user_id: i32,
) -> Result<Vec<Post>, CustomError> {
    let conn = pool.get().unwrap();
    let user = users.find(owner_user_id).get_result::<User>(&conn)?;
    let items = Post::belonging_to(&user).load::<Post>(&conn)?;
    Ok(items)
}

pub fn add_single_post_helper(
    pool: web::Data<Pool>,
    payload: web::Json<JsonInputPost>,
) -> Result<Post, CustomError> {
    let conn = pool.get().unwrap();
    let new_post = NewPost {
        title: &payload.title,
        body: &payload.body,
        user_id: &payload.user_id,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(posts).values(&new_post).get_result(&conn)?;
    Ok(res)
}

pub fn delete_single_post_helper(
    pool: web::Data<Pool>,
    post_id: i32,
) -> Result<usize, CustomError> {
    let conn = pool.get().unwrap();
    let res = delete(posts.filter(id.eq(post_id))).execute(&conn)?;

    match res {
        0 => Err(CustomError::new(404, format!("User not found"))),
        1 => Ok(res),
        _ => Err(CustomError::new(500, format!("Internal server error"))),
    }
}
