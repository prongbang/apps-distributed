use std::env;
use serde_json::json;
use vercel_runtime::{Response, StatusCode};

pub fn response_ok<T: From<String>>(message: &str) -> Response<T> {
    return response(StatusCode::OK, message);
}

pub fn response_bad_request<T: From<String>>(message: &str) -> Response<T> {
    return response(StatusCode::BAD_REQUEST, message);
}

pub fn response<T: From<String>>(status: StatusCode, message: &str) -> Response<T> {
    return Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(
            json!({"message": message})
                .to_string()
                .into(),
        ).unwrap();
}

pub fn get_value(key: &str) -> String {
    return env::var(key).unwrap_or_else(|_| {
        "".to_string()
    });
}

pub fn get_kv_api_url() -> String {
    return get_value("KV_REST_API_URL");
}

pub fn get_kv_api_token() -> String {
    return get_value("KV_REST_API_TOKEN");
}