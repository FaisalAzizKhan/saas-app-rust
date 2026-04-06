use diesel::prelude::*;
use crate::{
    db::pg_db::establish_connection_pg,
    models::users::users_models::{NewUser, UserResponse},
    schema::users::dsl::{users, email}
};
// use crate::;

pub fn insert_user(new_user: &NewUser) -> Result<UserResponse, diesel::result::Error> {
    let mut connection = establish_connection_pg()
        .map_err(|_| diesel::result::Error::NotFound)?;

    diesel::insert_into(users)
        .values(new_user)
        .get_result::<UserResponse>(&mut connection)
        // .execute(&mut connection)
}

pub fn find_user_by_email(
    user_email: &str,
) -> Result<UserResponse, diesel::result::Error> {
    let mut connection = establish_connection_pg()
        .map_err(|_| diesel::result::Error::NotFound)?;

    users
        .filter(email.eq(user_email))
        .select(UserResponse::as_select()) // 👈 important
        .first(&mut connection)
}

pub fn get_all_users_list() -> Result<Vec<UserResponse>, diesel::result::Error> {
    let mut connection = establish_connection_pg()
        .map_err(|_| diesel::result::Error::NotFound)?;

    users
        .select(UserResponse::as_select()) // 👈 important
        .load::<UserResponse>(&mut connection)
}