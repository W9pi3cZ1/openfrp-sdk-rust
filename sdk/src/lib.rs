/// API URLs API的地址
pub mod api_url;

// APIs impl API的实现

/// Get node list 获取节点列表
#[cfg(feature = "async")]
pub mod get_node_list;

/// Get user info 获取用户信息
#[cfg(feature = "async")]
pub mod get_user_info;

/// Get user proxies 获取用户隧道
#[cfg(feature = "async")]
pub mod get_user_proxies;

/// Login 登录
#[cfg(feature = "async")]
pub mod login;

/* Unable API
/// Sign 签到
#[cfg(feature = "async")]
pub mod sign;
*/

/// New proxy 新建隧道
#[cfg(feature = "async")]
pub mod new_proxy;

/// Edit proxy 编辑隧道
#[cfg(feature = "async")]
pub mod edit_proxy;

/// Remove proxy 删除隧道
#[cfg(feature = "async")]
pub mod remove_proxy;

/// Prelude modules 预导入模块
#[cfg(feature = "async")]
pub mod prelude;

#[cfg(feature = "blocking")]
pub mod blocking;