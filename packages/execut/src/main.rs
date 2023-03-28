use axum::{routing::get, Router, Server, response::IntoResponse, http::StatusCode};
use clap::Parser;
use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;

use execut::{routes::router, Config, Context};

#[tokio::main]
async fn main() {
    // dotenvy::dotenv().expect("Unable to read environment variables from `.env`");

    let config = Config::parse();

    let Config {
        database_url,
        jwt_secret,
    } = config;

    let pool = PgPoolOptions::new()
        .connect_lazy(&database_url)
        .expect("Unable to connect to postgres database, ensure it is running");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Unable to run migrations on postgres database");

    let jwt_secret = jwt_secret.as_bytes();

    let encoding_key = EncodingKey::from_secret(jwt_secret);
    let decoding_key = DecodingKey::from_secret(jwt_secret);

    let state = Context {
        pool,
        encoding_key,
        decoding_key,
    };

    let app = Router::new()
        .route("/health", get(health_check))
        .merge(router())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = "[::]:3000".parse().unwrap();

    println!("Listening on http://{addr} 🚀");

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_check() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}
