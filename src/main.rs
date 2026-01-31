mod schema;
mod model;
mod services;

use actix_web::{web, App, HttpServer};

use dotenv::dotenv;
use sqlx::{
    Pool, Postgres, postgres::PgPoolOptions
};

pub struct AppState {
    db: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting on http://127.0.0.1:8080");

    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new().max_connections(10).connect(&database_url).await {
        Ok(pool) => {
            println!("Connection DB resolved");
            pool
        }
        Err(error) => {
            println!("Failed to connect to the database: {:?}", error);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || App::new()
        .app_data(web::Data::new(AppState { db: pool.clone() }))
        .configure(services::config))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}