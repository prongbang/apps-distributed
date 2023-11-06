use std::collections::HashMap;
use std::env;
use serde_json::json;
use vercel_runtime::{Request, Response, StatusCode};

pub fn response_ok<T: From<String>>(message: &str) -> Response<T> {
    return response(StatusCode::OK, message);
}

pub fn response_bad_request<T: From<String>>(message: &str) -> Response<T> {
    return response(StatusCode::BAD_REQUEST, message);
}

pub fn response_forbidden<T: From<String>>(message: &str) -> Response<T> {
    return response(StatusCode::FORBIDDEN, message);
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

pub fn get_api_key() -> String {
    return get_value("API_KEY");
}

pub fn get_query(req: &Request, key: &str) -> String {
    if let Some(query) = req.uri().query() {
        let parsed_query: HashMap<_, _> = url::form_urlencoded::parse(query.as_bytes()).into_owned().collect();
        if let Some(value) = parsed_query.get(key) {
            return value.to_string();
        }
    }
    return "".to_string();
}