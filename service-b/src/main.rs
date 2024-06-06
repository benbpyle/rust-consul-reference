use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ExternalModel {
    key_one: String,
    key_two: String,
}

#[derive(Deserialize, Debug)]
struct Prefix {
    name: Option<String>,
}

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").expect("PORT is required");
    let host = format!("127.0.0.1:{}", port);
    let app = Router::new().route("/", get(handler));
    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    tracing::debug!("Up and running ... listening on {}", port);
    axum::serve(listener, app).await.unwrap();
}

async fn handler(Query(q): Query<Prefix>) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let host = std::env::var("SERVICE_A_URL").expect("SERVICE_A_URL Must be Set");
    let prefix: String;
    let passed_value = &q.name;

    if let Some(s) = passed_value {
        prefix = String::from(s.as_str());
    } else {
        prefix = String::from("Unknown");
    }

    let url = format!("{}/route?p={}", host, prefix);
    let response = reqwest::get(url).await;

    match response {
        Ok(r) => {
            if r.status().is_success() {
                let j: Result<ExternalModel, Error> = r.json().await;
                match j {
                    Ok(m) => Ok(Json(m)),
                    Err(e) => {
                        tracing::error!("Error parsing: {}", e);
                        Err(StatusCode::BAD_REQUEST)
                    }
                }
            } else {
                tracing::error!("Bad request");
                Err(StatusCode::BAD_REQUEST)
            }
        }
        Err(e) => {
            tracing::error!("Error requesting: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
