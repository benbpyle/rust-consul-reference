use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use chrono::{DateTime, Utc};
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ExternalModel {
    key_one: String,
    key_two: String,
    key_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServiceAModel {
    key_one: String,
    key_two: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServiceCModel {
    key_time: DateTime<Utc>,
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

async fn handler(Query(q): Query<Prefix>) -> Result<impl IntoResponse, StatusCode> {
    let service_a_model_response = get_service_a(q).await?;
    let service_c_model_response = get_service_c().await?;
    let external_model = ExternalModel {
        key_one: service_a_model_response.key_one,
        key_two: service_a_model_response.key_two,
        key_time: service_c_model_response.key_time,
    };
    Ok(Json(external_model))
}

async fn get_service_c() -> Result<ServiceCModel, StatusCode> {
    let service_c_host: String = std::env::var("SERVICE_C_URL").expect("SERVICE_C_URL Must be Set");
    let url = format!("{}/time", service_c_host);
    let response = reqwest::get(url.as_str()).await;
    tracing::info!("(Request)={}", url.as_str());
    match response {
        Ok(r) => {
            if r.status().is_success() {
                let j: Result<ServiceCModel, Error> = r.json().await;
                match j {
                    Ok(m) => Ok(m),
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

async fn get_service_a(q: Prefix) -> Result<ServiceAModel, StatusCode> {
    let service_a_host: String = std::env::var("SERVICE_A_URL").expect("SERVICE_A_URL Must be Set");

    let prefix: String;
    let passed_value = &q.name;

    if let Some(s) = passed_value {
        prefix = String::from(s.as_str());
    } else {
        prefix = String::from("Unknown");
    }

    let url = format!("{}/route?p={}", service_a_host, prefix);
    let response = reqwest::get(url.as_str()).await;
    tracing::info!("(Request)={}", url.as_str());
    match response {
        Ok(r) => {
            if r.status().is_success() {
                let j: Result<ServiceAModel, Error> = r.json().await;
                match j {
                    Ok(m) => Ok(m),
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
