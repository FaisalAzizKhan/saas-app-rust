
// File: src/models/users/users_models.rs

use diesel::prelude::*;
use crate::schema::users;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct UserInput {
    pub name: String,
    pub email: String,
    pub password: String,
    pub is_admin: Option<bool>,
    pub is_deleted: Option<bool>,
    // pub users_id: Uuid,
}


#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    // pub users_id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
    pub is_deleted: bool,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserResponse {
    pub users_id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
    pub is_deleted: bool,
}
#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserWithoutPasswordResponse {
    pub users_id: uuid::Uuid,
    pub name: String,
    pub email: String, 
    pub is_admin: bool,
    pub is_deleted: bool,
}

