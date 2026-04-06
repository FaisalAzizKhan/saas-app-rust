use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub users_id: uuid::Uuid,    
    pub email: String,
    pub is_admin: bool,
    pub exp: usize,    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserTokenResponse {
    pub users_id: uuid::Uuid,
    pub email: String,
    pub is_admin: bool,

}