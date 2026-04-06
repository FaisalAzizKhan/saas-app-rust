use actix_web::web;
use crate::controllers::users::users_controllers;


pub fn users_routes(cfg: &mut web::ServiceConfig) {
    
    cfg.service(
        web::scope("/users")
            .route("/create-new", web::post().to(users_controllers::create_new_user))
            .route("/get-all", web::get().to(users_controllers::get_all_users))
        
    );
}