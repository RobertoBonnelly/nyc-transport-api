use dotenvy::dotenv;
use std::sync::Arc;
use std::env;
use axum::{routing::{get, post, put, delete}, Router};
use std::net::SocketAddr;
use axum_server;
use sqlx::{postgres::PgPoolOptions};
mod users;
use users::handler::{create_user, update_user, delete_user};

#[tokio::main]
async fn main(){
    // load .env with dotenv
    dotenv().ok();
    // load database url with env instance
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // print to console
    println!("DATABASE_URL: {}", database_url);
    // connect to actual database
    let pool = Arc::new(PgPoolOptions::new().max_connections(5)
    .connect(&database_url)
    .await
    .expect("Could not connect to database"));
    // Define app variable with root route getting "Hello, Rust"
    let app = Router::new().route("/", get(|| async { "Hello, Rust!" }))
    .merge(Router::new().route("/createUser", post(create_user))
    .with_state(pool.clone()))
    .merge(Router::new().route("/updateUser", put(update_user))
    .with_state(pool.clone()))
    .merge(Router::new().route("/deleteUser", delete(delete_user))
    .with_state(pool.clone()));

    // Define SocketAddr to localhost(127.0.0.1):3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server is running at http://{}", addr);

    axum_server::Server::bind(addr).serve(app.into_make_service()).await.unwrap()
}
