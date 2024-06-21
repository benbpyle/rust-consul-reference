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

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthCheck {
    status: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .json()
        .with_target(false)
        .without_time()
        .init();

    let bind_address = std::env::var("BIND_ADDRESS").expect("BIND_ADDRESS is required");
    let app = Router::new()
        .route("/", get(handler))
        .route("/health", get(health));
    let listener = tokio::net::TcpListener::bind(bind_address.clone())
        .await
        .unwrap();
    tracing::info!("Up and running ... listening on {}", bind_address);
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
    tracing::info!("(Request)={}", prefix);
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
                tracing::error!("Bad request={:?}", r.status());
                Err(StatusCode::BAD_REQUEST)
            }
        }
        Err(e) => {
            tracing::error!("Error requesting: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn health() -> Result<impl IntoResponse, StatusCode> {
    let healthy = HealthCheck {
        status: String::from("Healthy"),
    };

    Ok(Json(healthy))
}
