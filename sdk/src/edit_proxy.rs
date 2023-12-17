use std::collections::HashMap;

use serde_json::Value;

use super::api_url;
use super::prelude::*;

/// Edit proxy API impl 编辑隧道的API实现
pub async fn edit_proxy(
    auth: &Auth,
    proxy: &Proxy,
    client: reqwest::Client,
) -> reqwest::Result<HashMap<String,Value>> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    headers.insert("authorization", auth.authorization.parse().unwrap());
    let mut json = serde_json::to_value(proxy).unwrap();
    json.as_object_mut().unwrap().insert(
        "session".to_string(),
        serde_json::to_value(auth.session_id.clone()).unwrap(),
    );
    let response = request_post(client, api_url::EDIT_PROXY, headers, &json).await?;
    let json = get_json_by_response(response).await?;
    Ok(json)
}
