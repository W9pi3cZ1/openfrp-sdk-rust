use reqwest::{header::HeaderMap, redirect::Policy};
use serde::Serialize;

/// Create a new API Client 创建一个 API Client
pub fn new_client() -> reqwest::Result<reqwest::Client> {
    Ok(reqwest::Client::builder()
        .redirect(Policy::limited(2))
        .cookie_store(true)
        .build()?
        .clone())
}

/// Make a POST request 发送 POST 请求
pub async fn request_post<T: Serialize + ?Sized>(
    client: reqwest::Client,
    url: &str,
    header: HeaderMap,
    json: &T,
) -> reqwest::Result<reqwest::Response> {
    Ok(client.post(url).headers(header).json(&json).send().await?)
}

/// Make a GET request 发送 GET 请求
pub async fn request_get(
    client: reqwest::Client,
    url: &str,
    header: HeaderMap,
) -> reqwest::Result<reqwest::Response> {
    Ok(client.get(url).headers(header).send().await?)
}

/// Get JSON from response 获取响应中的 JSON
pub async fn get_json_by_response(
    response: reqwest::Response,
) -> reqwest::Result<std::collections::HashMap<String, serde_json::Value>> {
    Ok(response
        .json::<std::collections::HashMap<String, serde_json::Value>>()
        .await?)
}

/// Get headers from response 获取响应中的头
pub fn get_headers_by_response(response: &reqwest::Response) -> HeaderMap {
    response.headers().clone()
}

/// Storage auth info 存储验证信息
///
/// *If you wanna to get it, You need Login to OpenFrp.*
/// *如果你想要得到它，你需要登录到OpenFrp*
///
/// **The login module is `openfrp-sdk::login`*
/// *登录模块是`openfrp-sdk::login`**
#[allow(dead_code)]
#[derive(Debug)]
pub struct Auth {
    pub session_id: String,
    pub authorization: String,
}

// Error process 错误处理

/// Custom result 自定义Result
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]

/// Storage error struct 存储错误的结构体
pub struct Error {
    pub kind: String,
    pub message: String,
}

/// let Error impl trait StdError 让Error实现trait StdError
impl std::error::Error for Error {}

/// let Error impl trait Display 让Error实现trait Display
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

/// Error impls
impl Error {
    /// New an Error 新建一个Error
    pub fn new(kind: String, message: String) -> Box<dyn std::error::Error> {
        Box::new(Self { kind, message })
    }
}

/// Account struct 账户结构体
#[derive(Serialize, Debug)]
pub struct Account {
    pub user: String,
    pub password: String,
}

/// Proxy struct 隧道结构体
#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct Proxy {
    /// Proxy id 隧道ID
    pub proxy_id: i32,
    
    /// Node id 节点id
    pub node_id: i32,

    /// Proxy name 隧道名称
    pub name: String,

    /// Proxy type 隧道类型
    pub r#type: String,

    /// Local address 本地地址
    pub local_addr: String,
    
    /// Local port 本地端口
    pub local_port: String,
    
    /// Remote port 远程端口
    pub remote_port: i32,

    /// Domain bind 绑定的域名
    pub domain_bind: String,
    
    /// Data compress 数据压缩
    pub dataGzip: bool,
    
    /// Data encrypt 数据加密
    pub dataEncrypt: bool,
    
    /// URL route URL路由
    pub url_route: String,
    
    /// Host rewrite HOST重写
    pub host_rewrite: String,
    
    /// Request from 请求来源
    pub request_from: String,
    
    /// Request password 请求密码
    pub request_pass: String,

    /// Proxy custom profile 隧道的自定义配置文件
    pub custom: String,
}
