use actix_web::web;
use crate::controllers::{auth::auth_controllers};


pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    
    cfg.service(
        web::scope("/auth")
            .route("/users-login", web::post().to(auth_controllers::login))
      
        
    );
}