use std::collections::HashMap;

use serde_json::Value;

use crate::api_url;
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
    json.as_object_mut().unwrap().remove("remote_port");
    let response = request_post(client, api_url::EDIT_PROXY, headers, &json).await?;
    let json = get_json_by_response(response).await?;
    Ok(json)
}
