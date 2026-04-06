use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};

use crate::{
    models::users::users_models::{NewUser, UserInput, UserWithoutPasswordResponse},
    repository::users::users_repository::{find_user_by_email, get_all_users_list, insert_user},
    types::users::users_controllers_types::{ApiErrorResponse, ApiResponse},
};

pub async fn create_new_user(new_user: web::Json<UserInput>) -> impl Responder {
    let hashed_password = match hash(&new_user.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let new_user_data = NewUser {
        name: new_user.name.clone(),
        email: new_user.email.to_lowercase().clone(),
        password: hashed_password,
        is_admin: new_user.is_admin.unwrap_or(false),
        is_deleted: new_user.is_deleted.unwrap_or(false),
    };

    let existing_user = find_user_by_email(&new_user_data.email.to_lowercase());

    if existing_user.is_ok() {
        return HttpResponse::BadRequest().json(ApiErrorResponse {
            message: "User with this email already exists".to_string(),
        });
    }

    match insert_user(&new_user_data) {
        Ok(user) => {
            let response_user = UserWithoutPasswordResponse {
                users_id: user.users_id,  
                name: user.name,
                email: user.email,
                is_admin: user.is_admin,
                is_deleted: user.is_deleted,
          
            };

            HttpResponse::Ok().json(ApiResponse {
                message: "User created successfully".to_string(),
                body: response_user,
            })
        }

        Err(e) => {
            eprintln!("Error inserting user: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse {
                message: "Failed to create user".to_string(),
                body: format!("{:?}", e),
            })
        }
    }
}

pub async fn get_all_users() -> impl Responder {
    let users_result = get_all_users_list();

    match users_result {
        Ok(users) => HttpResponse::Ok().json(ApiResponse {
            message: "Get all users endpoint".to_string(),
            body: users,
        }),
        Err(e) => {
            eprintln!("Error fetching users: {:?}", e);
            HttpResponse::InternalServerError().json(ApiErrorResponse {
                message: "Failed to fetch users".to_string(),
            })
        }
    }
}
