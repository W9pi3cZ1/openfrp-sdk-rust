use std::collections::HashMap;

use serde_json::Value;

use crate::api_url;
use super::prelude::*;

/// Get user proxies API impl 获取用户API的实现
pub fn get_user_proxies(auth: &Auth, client: reqwest::blocking::Client) -> reqwest::Result<HashMap<String,Value>>{
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    headers.insert("authorization", auth.authorization.parse().unwrap());
    let json = serde_json::json!({
        "session": auth.session_id,
    });
    let response = request_post(client, api_url::GET_USER_PROXIES, headers, &json)?;
    let json = get_json_by_response(response)?;
    Ok(json)
}