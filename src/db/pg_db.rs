use diesel::pg::PgConnection;
use diesel::Connection; 
use dotenvy::dotenv;
use std::env;



pub fn establish_connection_pg() -> Result<PgConnection, Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let conn = PgConnection::establish(&database_url)?;

    Ok(conn)
}