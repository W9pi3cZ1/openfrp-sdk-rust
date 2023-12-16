use reqwest::{header::HeaderMap, redirect::Policy};
use serde::Serialize;

pub fn new_client() -> reqwest::Result<reqwest::Client> {
    Ok(reqwest::Client::builder()
        .redirect(Policy::limited(2))
        .cookie_store(true)
        .build()?
        .clone())
}

pub async fn request_post<T: Serialize + ?Sized>(
    client: reqwest::Client,
    url: &str,
    header: HeaderMap,
    json: &T,
) -> reqwest::Result<reqwest::Response> {
    Ok(client.post(url).headers(header).json(&json).send().await?)
}

pub async fn request_get(
    client: reqwest::Client,
    url: &str,
    header: HeaderMap,
) -> reqwest::Result<reqwest::Response> {
    Ok(client.get(url).headers(header).send().await?)
}

pub async fn get_json_by_response(
    response: reqwest::Response,
) -> reqwest::Result<std::collections::HashMap<String, serde_json::Value>> {
    Ok(response
        .json::<std::collections::HashMap<String, serde_json::Value>>()
        .await?)
}

pub fn get_headers_by_response(response: &reqwest::Response) -> HeaderMap {
    response.headers().clone()
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Auth {
    pub session_id: String,
    pub authorization: String,
}

// Error processer

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Error {
    pub kind: String,
    pub message: String,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind as &str {
            _ => write!(
                f,
                "Program Error:{{Error kind: {}, Error message: {}}}",
                self.kind, self.message
            ),
        }
    }
}

impl Error {
    pub fn new(kind: String, message: String) -> Box<dyn std::error::Error> {
        Box::new(Self { kind, message })
    }
}

// Account

#[derive(Serialize, Debug)]
pub struct Account {
    pub user: String,
    pub password: String,
}
