use std::collections::HashMap;

use serde_json::Value;

use crate::api_url;
use super::prelude::*;

/// New proxy API impl 新建隧道的API实现
pub fn new_proxy(
    auth: &Auth,
    proxy: &Proxy,
    client: reqwest::blocking::Client,
) -> reqwest::Result<HashMap<String,Value>> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    headers.insert("authorization", auth.authorization.parse().unwrap());
    let mut json = serde_json::to_value(proxy).unwrap();
    json.as_object_mut().unwrap().insert(
        "session".to_string(),
        serde_json::to_value(auth.session_id.clone()).unwrap(),
    );
    let response = request_post(client, api_url::NEW_PROXY, headers, &json)?;
    let json = get_json_by_response(response)?;
    Ok(json)
}
