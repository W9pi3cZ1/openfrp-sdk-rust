use std::collections::HashMap;

use serde_json::Value;

use crate::api_url;
use super::prelude::*;

/// Remove proxy API impl 删除隧道的API实现
pub fn remove_proxy(
    auth: &Auth,
    proxy_id: i32,
    client: reqwest::blocking::Client,
) -> reqwest::Result<HashMap<String,Value>> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    headers.insert("authorization", auth.authorization.parse().unwrap());
    let json = serde_json::json!(
        {
            "proxy_id": proxy_id,
            "session": auth.session_id,
        }
    );
    let response = request_post(client, api_url::REMOVE_PROXY, headers, &json)?;
    let json = get_json_by_response(response)?;
    Ok(json)
}
