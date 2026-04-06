use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};

use crate::types::utilities::token_utilities_types::{Claims, UserTokenResponse};

pub fn generate_jwt(user: &UserTokenResponse) -> String {
    let secret = std::env::var("JWT_SECRET_KEY")
        .expect("JWT_SECRET_KEY must be set");

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        users_id: user.users_id,
        email: user.email.clone(),
        is_admin: user.is_admin,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .expect("JWT token creation failed")
}
