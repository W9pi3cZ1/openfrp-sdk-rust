use crate::api_url;
use super::prelude::*;
use reqwest;
use reqwest::header::HeaderMap;
use serde_json::{self, Value};
use std::collections::HashMap;

/// Login to OpenFrp by account 用Account登录到OpenFrp
pub fn login(account: &Account, client: reqwest::blocking::Client) -> Result<Auth> {
    login_oauth2(client.clone(), account)?;

    let oauth2_callback = oauth2_callback(client.clone())?;

    let login_callback = login_by_callback(client.clone(), oauth2_callback)?;

    let session_id = login_callback
        .1
        .get("data")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    let authorization = login_callback
        .0
        .get("authorization")
        .unwrap()
        .to_str()?
        .to_string();
    
    let auth = Auth {
        session_id: session_id,
        authorization: authorization,
    };
    return Ok(auth);
}

/// Login to OAuth2 登录到OAuth2
pub fn login_oauth2(
    client: reqwest::blocking::Client,
    account: &Account,
) -> Result<HashMap<String, Value>> {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    let response = request_post(client.clone(), api_url::OAUTH2_URL, headers, &account)?;
    let json = get_json_by_response(response)?;
    if json.get("flag").unwrap() != true {
        return Err(Error::new(
            "OAuth2 Login Failed".to_string(),
            json.get("message").unwrap().to_string(),
        ));
    }
    Ok(json)
}

/// Get OAuth2 login callback 获取OAuth2登录回调
pub fn oauth2_callback(client: reqwest::blocking::Client) -> Result<HashMap<String, Value>> {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());

    let response = request_post(client.clone(), api_url::OAUTH2_CALLBACK, headers, "")?;
    let json = get_json_by_response(response)?;
    if json.get("flag").unwrap() != true {
        return Err(Error::new(
            "OAuth2 Callback Failed".to_string(),
            json.get("message").unwrap().to_string(),
        ));
    }
    Ok(json)
}


/// Login to OpenFrp by OAuth2 login callback 用OAuth2登录回调登录到OpenFrp
pub fn login_by_callback(
    client: reqwest::blocking::Client,
    oauth2_callback: HashMap<String, Value>,
) -> Result<(HeaderMap, HashMap<String, Value>)> {
    let headers = HeaderMap::new();
    let code = oauth2_callback
        .get("data")
        .unwrap()
        .get("code")
        .unwrap()
        .as_str()
        .unwrap();
    let url = format!("{0}{1}", api_url::LOGIN_CALLBACK, code);
    let response = request_post(client.clone(), url.as_str(), headers, "")?;
    let headers = get_headers_by_response(&response);
    let json = get_json_by_response(response)?;
    if json.get("flag").unwrap() != true {
        return Err(Error::new(
            "Login OpenFrp Failed".to_string(),
            json.get("message").unwrap().to_string(),
        ));
    }
    Ok((headers, json))
}
