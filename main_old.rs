use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
    }
}

#[derive(Queryable, Debug)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUser<'a> {
    name: &'a str,
    email: &'a str,
}

 
fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("Error connecting to PostgreSQL")
}

 
fn create_user<'a>(
    conn: &mut PgConnection,
    user_name: &'a str, 
    user_email: &'a str,
) -> Result<usize, diesel::result::Error> {
   
    let new_user = NewUser {
        name: user_name,
        email: user_email,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
}

fn main() {
    let mut connection = establish_connection();

    create_user(&mut connection, "Alice", "alice@example.com").expect("Failed to create user");
    create_user(&mut connection, "Bob", "bob@example.com").expect("Failed to create user");

    let results = users::table
        .load::<User>(&mut connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!(
            "ID: {}, Name: {}, Email: {}",
            user.id, user.name, user.email
        );
    }
}


use actix_web::{web, HttpResponse, Responder};

use crate::{
    db::pg_db::establish_connection_pg,
    models::users::users_models::{NewUser, UserInput},
    schema::users,
    types::users::users_controllers_types::ApiResponse,
};
use diesel::prelude::*;

pub async fn create_new_user(new_user: web::Json<UserInput>) -> impl Responder {
    let new_user_data = NewUser {
        name: new_user.name.clone(),
        email: new_user.email.clone(),
        password: new_user.password.clone(),
        is_admin: false,
        is_deleted: false,
    };

    let mut connection = match establish_connection_pg() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to connect to DB: {:?}", e);
            return HttpResponse::InternalServerError().body("DB connection failed");
        }
    };

    match diesel::insert_into(users::table)
        .values(&new_user_data)
        .execute(&mut connection)
    {
        Ok(_) => HttpResponse::Ok().json(ApiResponse {
            message: "User created successfully".to_string(),
            body: new_user.into_inner(),
        }),
        Err(e) => {
            eprintln!("Error inserting user: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse {
                message: "Failed to create user".to_string(),
                body: format!("{:?}", e),
            })
        }
    }
}
