use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};
use jsonwebtoken::{DecodingKey, Validation, errors::ErrorKind::*};
use ngrok::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Debug)]
struct Output {
    solution: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Claims {
    append: Option<String>,
    exp: Option<usize>,
    //    iss: Option<String>,
    //    iat: Option<usize>,
    //    aud: Option<String>,
    nbf: Option<usize>,
}

struct AppState {
    jwt_secret: String,
    solution: Mutex<String>,
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
    // publish localhost:3000 with ngrok
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

    let jwt_secret = util::get_problem::<Input>("jotting_jwts").await?.jwt_secret;
    let shared_state = Arc::new(AppState {
        jwt_secret,
        solution: Mutex::new(String::new()),
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
    Ok(())
}

// TODO: Somehow Invalid token are appended
async fn jwt_handler(State(state): State<Arc<AppState>>, body: String) -> impl IntoResponse {
    let secret = &state.jwt_secret;
    let mut cum_solution = state.solution.lock().unwrap();
    let solution = cum_solution.clone();

    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_required_spec_claims(&["append"]);
    validation.validate_nbf = true;
    match jsonwebtoken::decode::<Claims>(
        body.as_str(),
        &DecodingKey::from_secret(&secret.as_bytes()),
        &validation,
    ) {
        Ok(token) => {
            println!("Oked tolen: {:?}", token);
            if let Some(append_str) = token.claims.append {
                cum_solution.push_str(&append_str);
                return (axum::http::StatusCode::OK, "OK").into_response();
            } else {
                let result = Output { solution };
                return Json::<Output>(result).into_response();
            }
        }
        Err(e) => match e.into_kind() {
            InvalidSignature => {
                return (axum::http::StatusCode::UNAUTHORIZED, "Invalid Token").into_response();
            }
            ExpiredSignature => {
                return (axum::http::StatusCode::UNAUTHORIZED, "Expired Token").into_response();
            }
            ImmatureSignature => {
                return (axum::http::StatusCode::UNAUTHORIZED, "Expired Token").into_response();
            }
            kind => {
                println!("Error: {kind:?}");
                return (axum::http::StatusCode::UNAUTHORIZED, "Expired Token").into_response();
            }
        },
    };
}
