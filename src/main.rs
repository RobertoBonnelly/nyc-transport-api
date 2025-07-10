use dotenvy::dotenv;
use std::env;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use axum_server;
use sqlx::{postgres::PgPoolOptions};

#[tokio::main]
async fn main(){
    // load .env with dotenv
    dotenv().ok();
    // load database url with env instance
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // print to console
    println!("DATABASE_URL: {}", database_url);
    // connect to actual database
    let _pool = PgPoolOptions::new().max_connections(5)
    .connect(&database_url)
    .await
    .expect("Could not connect to database");
    // Define app variable with root route getting "Hello, Rust"
    let app = Router::new().route("/", get(|| async { "Hello, Rust!" }));

    // Define SocketAddr to localhost(127.0.0.1):3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server is running at http://{}", addr);

    axum_server::Server::bind(addr).serve(app.into_make_service()).await.unwrap()
}
