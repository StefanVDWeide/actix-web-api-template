// use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::errors::handlers::CustomError;
use crate::schema::users::dsl::*;
use crate::users::{JsonInputUser, NewUser, User};
use crate::Pool;
use actix_web::web;
use diesel::dsl::{delete, insert_into};
use diesel::prelude::*;

pub fn get_all_users_helper(pool: web::Data<Pool>) -> Result<Vec<User>, CustomError> {
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}

pub fn get_single_user(pool: web::Data<Pool>, user_id: i32) -> Result<User, CustomError> {
    let conn = pool.get().unwrap();
    let user = users.find(user_id).get_result::<User>(&conn)?;
    Ok(user)
}

// Function that takes a DB connection and a JSON payload and inserts it into the database
pub fn add_single_user(
    db: web::Data<Pool>,
    payload: web::Json<JsonInputUser>,
) -> Result<User, CustomError> {
    let conn = db.get().unwrap();
    let new_user = NewUser {
        first_name: &payload.first_name,
        last_name: &payload.last_name,
        email: &payload.email,
        created_at: chrono::Local::now().naive_local(),
    };
    let res = insert_into(users).values(&new_user).get_result(&conn)?;
    Ok(res)
}

pub fn delete_single_user(db: web::Data<Pool>, user_id: i32) -> Result<usize, CustomError> {
    let conn = db.get().unwrap();
    let res = delete(users.filter(id.eq(user_id))).execute(&conn)?;

    match res {
        0 => Err(CustomError::new(404, format!("User not found"))),
        1 => Ok(res),
        _ => Err(CustomError::new(500, format!("Internal server error"))),
    }
}
