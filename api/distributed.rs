use reqwest::header;
use serde::{Deserialize, Serialize};
use apps_distributed::{get_api_key, get_kv_api_token, get_kv_api_url, get_query, response_bad_request, response_forbidden, response_ok};
use vercel_runtime::{run, Body, Error, Request, Response};

#[derive(Debug, Deserialize, Serialize)]
struct DistributedResponse {
    result: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct DistributedRequest {
    platform: String,
    version: String,
    build: String,
    action: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let api_key = get_api_key();
    let key = get_query(&req, "key");

    if api_key != key || key.is_empty() {
        return Ok(response_forbidden("Forbidden"));
    }

    if req.method() == "POST" {
        let url = get_kv_api_url();
        let mut headers = header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, format!("Bearer {}", get_kv_api_token()).parse().unwrap());
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        let body = req.body();
        let full_body = body.as_ref();
        let data = serde_json::from_slice::<DistributedRequest>(full_body)?;

        if data.platform.is_empty() || data.version.is_empty() || data.build.is_empty() || data.action.is_empty() {
            return Ok(response_bad_request("Invalid data"));
        }

        let key = format!("{}:{}:{}", data.platform, data.version, data.build);
        if data.action == "get" {
            let response = client.get(format!("{}/get/{}", url, key)).send().await?;
            if response.status().is_success() {
                let json_body: DistributedResponse = response.json().await.unwrap();
                return match json_body.result {
                    None => {
                        println!("Get Key: {} FAILED", key);
                        Ok(response_ok(""))
                    }
                    Some(res) => {
                        println!("Get Key: {} SUCCESS", key);
                        Ok(response_ok(res.as_str()))
                    }
                };
            } else {
                println!("Get Key: {} FAILED", key);
                println!("Request was not successful: {}", response.status());
            }
        } else if data.action == "set" {
            let response = client.get(format!("{}/set/{}/done", url, key)).send().await?;
            if response.status().is_success() {
                let json_body: DistributedResponse = response.json().await.unwrap();
                return match json_body.result {
                    None => {
                        println!("Set Key: {} FAILED", key);
                        Ok(response_bad_request("Bad Request"))
                    }
                    Some(_) => {
                        println!("Set Key: {} SUCCESS", key);
                        Ok(response_ok("OK"))
                    }
                };
            } else {
                println!("Set Key: {} FAILED", key);
                println!("Request was not successful: {}", response.status());
            }
        }
    }

    Ok(response_bad_request("Bad Request"))
}