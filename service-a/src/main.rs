use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    key_one: String,
    key_two: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Prefix {
    p: Option<String>,
}

#[tokio::main]
async fn main() {
    tracing::fmt().json().init();
    let port = std::env::var("PORT").expect("PORT is required");
    let host = format!("127.0.0.1:{}", port);
    let app = Router::new().route("/route", get(handler));
    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    tracing::debug!("Up and running ... listening on {}", port);
    axum::serve(listener, app).await.unwrap();
}

async fn handler(query: Query<Prefix>) -> Result<impl IntoResponse, StatusCode> {
    let prefix: String;
    let passed_value = &query.p;

    if let Some(s) = passed_value {
        prefix = String::from(s.as_str());
    } else {
        prefix = String::from("Unknown");
    }

    let m: Model = Model {
        key_two: format!("({})Field 2", prefix),
        key_one: format!("({})Field 1", prefix),
    };

    Ok(Json(m))
}