use actix_web::{web, HttpResponse, Responder};
use bcrypt::verify;
use serde_json::json;

use crate::{
    models::auth::auth_models::AuthLoginInput,
    repository::users::users_repository::find_user_by_email,
    types::{
        users::users_controllers_types::{ApiErrorResponse, ApiResponse},
        utilities::token_utilities_types::UserTokenResponse,
    },
    utilities::token_utilities::generate_jwt,
};

pub async fn login(user_input: web::Json<AuthLoginInput>) -> impl Responder {
    let email = user_input.email.to_lowercase();
    let password = &user_input.password;

    println!("Login: Email: {}, Password: {}", email, password);

    let user = match find_user_by_email(&email) {
        Ok(user) => user,
        Err(_) => {
            return HttpResponse::Unauthorized().json(ApiErrorResponse {
                message: "Invalid email or password".to_string(),
            });
        }
    };

    let is_valid = verify(password, &user.password).unwrap_or(false);

    if !is_valid {
        return HttpResponse::Unauthorized().json(ApiErrorResponse {
            message: "Invalid email or password".to_string(),
        });
    }

    let token = generate_jwt(&UserTokenResponse {
        users_id: user.users_id,
        email: user.email.clone(),
        is_admin: user.is_admin,
    });

    let user_without_password = json!({
        "users_id": user.users_id,
        "email": user.email,
        "is_admin": user.is_admin,
    });

    HttpResponse::Ok().json(ApiResponse {
        message: "Login successful".to_string(),
        body: json!({
            "token": token,
            "user": user_without_password,
        }),
    })
}
