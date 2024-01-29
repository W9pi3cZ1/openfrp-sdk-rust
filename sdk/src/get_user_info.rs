use std::collections::HashMap;

use serde_json::Value;

use crate::api_url;
use super::prelude::*;

/// Get user info API impl 获取用户API的实现
pub async fn get_user_info(auth: &Auth, client: reqwest::Client) -> reqwest::Result<HashMap<String,Value>>{
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    headers.insert("authorization", auth.authorization.parse().unwrap());
    let json = serde_json::json!({
    });
    let response = request_post(client, api_url::GET_USER_INFO, headers, &json).await?;
    let json = get_json_by_response(response).await?;
    Ok(json)
}