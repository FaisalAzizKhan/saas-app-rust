use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct AuthLoginInput {
    pub email: String,
    pub password: String,
 
}

 
 