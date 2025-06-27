use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Router, routing::post};
use ngrok::prelude::*;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

struct AppState {
    jwt_secret: String,
    solution: String,
}

#[derive(Deserialize, Debug)]
struct Input {
    jwt_secret: String,
}

#[derive(Serialize, Debug)]
struct AppUrl {
    app_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tunnel = ngrok::Session::builder()
        .authtoken_from_env()
        .connect()
        .await?
        .http_endpoint()
        .listen_and_forward(url::Url::parse("http://localhost:3000").unwrap())
        .await?;

    let app_url = AppUrl {
        app_url: tunnel.url().to_string(),
    };
    println!("{:#?}", app_url);
    let json_data = util::get_problem::<Input>("jotting_jwts").await?;
    let shared_state = Arc::new(AppState {
        jwt_secret: json_data.jwt_secret,
        solution: String::new(),
    });
    let app = Router::new()
        .route("/", post(jwt_handler))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tokio::spawn(async move {
        println!("Starting Web Server...");
        axum::serve(listener, app).await.unwrap();
    });
    util::post_answer::<AppUrl>("jotting_jwts", app_url, false).await?;

    println!("Server started. Waiting for requests... (Press Ctrl+C to quit)");
    tokio::signal::ctrl_c().await?;
    println!("Shutting down...");
    Ok(())
}

async fn jwt_handler(State(_state): State<Arc<AppState>>, body: String) -> impl IntoResponse {
    println!("{}", body);
}
