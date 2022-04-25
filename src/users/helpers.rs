use crate::diesel::RunQueryDsl;
use crate::errors::handlers::CustomError;
use crate::schema::users::dsl::*;
use crate::users::{JsonInputUser, NewUser, User};
use crate::Pool;
use actix_web::web;
use diesel::dsl::insert_into;

pub fn get_all_users_helper(pool: web::Data<Pool>) -> Result<Vec<User>, CustomError> {
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
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
