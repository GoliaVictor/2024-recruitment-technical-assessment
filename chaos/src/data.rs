use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn process_data(Json(request): Json<DataRequest>) -> impl IntoResponse {
    // Calculate sums and return response

    let mut string_len = 0;
    let mut int_sum = 0;

    for value in &request.data {
        match value {
            serde_json::Value::String(s) => string_len += s.len(),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    int_sum += i;
                }
            }
            _ => {}
        }
    }

    let response = DataResponse {
        string_len,
        int_sum,
    };

    (StatusCode::OK, Json(response))
}

#[derive(Deserialize)]
pub struct DataRequest {
    // Add any fields here
    pub data: Vec<serde_json::Value>
}

#[derive(Serialize)]
pub struct DataResponse {
    // Add any fields here
    pub string_len: usize,
    pub int_sum: i64,
}
