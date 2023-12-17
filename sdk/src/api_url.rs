/// OAuth2 Login API OAuth2登录API
pub const OAUTH2_URL: &str = "https://openid.17a.ink/api/public/login";

/// OAuth2 Callback API OAuth2回调API
pub const OAUTH2_CALLBACK: &str = "https://openid.17a.ink/api/oauth2/authorize?response_type=code&redirect_uri=https://of-dev-api.bfsea.xyz/oauth_callback&client_id=openfrp";

/// Login API 登录API
pub const LOGIN_CALLBACK: &str = "https://of-dev-api.bfsea.xyz/oauth2/callback?code=";

/// Get user info API 获取用户信息API
pub const GET_USER_INFO: &str = "https://of-dev-api.bfsea.xyz/frp/api/getUserInfo";

/// Sign API 签到API
pub const SIGN_API: &str = "https://of-dev-api.bfsea.xyz/frp/api/userSign";

/// Get node list API 获取节点列表API
pub const GET_NODE_LIST: &str = "https://of-dev-api.bfsea.xyz/frp/api/getNodeList";

/// Get user proxies API 获取用户隧道API
pub const GET_USER_PROXIES: &str = "https://of-dev-api.bfsea.xyz/frp/api/getUserProxies";

/// New proxy API 新建隧道API
pub const NEW_PROXY: &str = "https://of-dev-api.bfsea.xyz/frp/api/newProxy";

/// Edit proxy API 编辑隧道API
pub const EDIT_PROXY: &str = "https://of-dev-api.bfsea.xyz/frp/api/editProxy";

/// Remove proxy API 删除隧道API
pub const REMOVE_PROXY: &str = "https://of-dev-api.bfsea.xyz/frp/api/editProxy";
