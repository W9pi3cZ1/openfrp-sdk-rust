/// Login APIs 登录类API
pub const OAUTH2_URL: &str = "https://openid.17a.ink/api/public/login";
pub const OAUTH2_CALLBACK: &str = "https://openid.17a.ink/api/oauth2/authorize?response_type=code&redirect_uri=https://of-dev-api.bfsea.xyz/oauth_callback&client_id=openfrp";
pub const LOGIN_CALLBACK: &str = "https://of-dev-api.bfsea.xyz/oauth2/callback?code=";

/// Get user info API 获取用户信息API
pub const GET_USER_INFO: &str = "https://of-dev-api.bfsea.xyz/frp/api/getUserInfo";

/// Sign API 签到API
pub const SIGN_API: &str = "https://of-dev-api.bfsea.xyz/frp/api/userSign";

/// Get node list API 获取节点列表API
pub const GET_NODE_LIST: &str = "https://of-dev-api.bfsea.xyz/frp/api/getNodeList";

/// Get user proxies API 获取用户隧道API
pub const GET_USER_PROXIES: &str = "https://of-dev-api.bfsea.xyz/frp/api/getUserProxies";