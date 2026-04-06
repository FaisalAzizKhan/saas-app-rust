use actix_cors::Cors;
use actix_web::{get, http, App, HttpResponse, HttpServer, Responder};

use crate::db::pg_db::establish_connection_pg;
use crate::routes::auth::auth_routes;
use crate::routes::users::users_routes;

mod controllers;
mod db;
mod models;
mod repository;
mod routes;
mod schema;
mod types;
mod utilities;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_port: u16 = std::env::var("SERVER_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3002);

    #[get("/")]
    async fn base_route() -> impl Responder {
        HttpResponse::Ok().json(serde_json::json!({
            "message": "Backend Server is Up and Running",
            "version": "1.0.0",
        }))
    }

    match establish_connection_pg() {
        Ok(conn) => {
            println!("\nDatabase connected successfully ✅");
            conn
        }
        Err(err) => {
            eprintln!("Failed to connect to database ❌: {}", err);
            std::process::exit(1);
        }
    };

    println!("Starting server on port: {} 🚀 ", server_port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|_origin, _req_head| true)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(base_route)
            .configure(users_routes::users_routes)
            .configure(auth_routes::auth_routes)
    })
    .bind(("127.0.0.1", server_port))?
    .run()
    .await
}
