mod api;
mod api_bridge;
mod models;
mod utils;
mod website;

use api::api_routes;
use axum::{
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    routing::get,
    Json, Router,
};
use serde::Serialize;
use sqlx::{migrate::Migrator, postgres::PgPoolOptions, Connection, PgConnection, Pool, Postgres};
use utils::AppState;
use website::website_routes;

use crate::{api_bridge::ApiBridge, website::js_routes};

static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_base64 = std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
    let pool = create_and_setup_db(&db_url).await?;

    let router = Router::new()
        .route("/ping", get(ping))
        .nest("/api", api_routes())
        .merge(website_routes())
        .merge(js_routes())
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

    axum::serve(listener, router.into_make_service())
        .await
        .expect("Failed to start server");
    Ok(())
}

async fn create_and_setup_db(db_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    let mut url = url::Url::parse(db_url).unwrap();
    let db_name = url.path().trim_start_matches('/').to_string();
    url.set_path("/postgres");
    let postgres_url = url.as_str();

    let mut conn = PgConnection::connect(postgres_url).await?;

    let db_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = $1)",
    )
    .bind(&db_name)
    .fetch_one(&mut conn)
    .await?;

    if !db_exists {
        let create_db_query = format!("CREATE DATABASE \"{}\";", db_name);
        sqlx::query(&create_db_query).execute(&mut conn).await?;
        println!("Database '{}' created!", db_name);
    } else {
        println!("Database '{}' already exists.", db_name);
    }
    let pool = PgPoolOptions::new().connect(db_url).await?;
    MIGRATOR.run(&pool).await?;
    Ok(pool)
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
