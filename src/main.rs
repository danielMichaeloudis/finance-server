mod models;
mod routes;
mod utils;
use axum::{
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    routing::get,
    Json, Router,
};
use routes::rest_router;
use serde::Serialize;
use sqlx::{migrate::Migrator, postgres::PgPoolOptions};
use utils::AppState;

static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_base64 = std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
    let pool = PgPoolOptions::new().connect(&db_url).await?;

    MIGRATOR.run(&pool).await?;

    let app = Router::new()
        .route("/ping", get(ping))
        .nest("/api", rest_router())
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_headers([CONTENT_TYPE, AUTHORIZATION])
                .allow_methods(Method::GET),
        )
        .with_state(AppState::new(pool, &jwt_base64));
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Server started. Listening on {addr}");

    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to start server");
    Ok(())
}

#[derive(Serialize)]
struct PingRes {
    pub server_name: String,
}

async fn ping() -> Result<Json<PingRes>, ()> {
    Ok(Json(PingRes {
        server_name: "Finance Server".to_string(),
    }))
}
