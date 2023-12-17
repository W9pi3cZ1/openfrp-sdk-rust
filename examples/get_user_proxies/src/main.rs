use openfrp_sdk::prelude::*; // Import prelude module 导入预导入模块
use openfrp_sdk::{login, get_user_proxies}; // Import login and get user proxies APIs 导入 登录 和 获取用户隧道 的API
use tokio; // Import tokio a async modules 导入异步实现模块tokio

fn main() -> Result<()> {
    tokio::runtime::Builder::new_current_thread() // Create a single threads runtime with tokio 用tokio创建一个单线程的异步运行时环境
        .enable_all() // Enable all features 启用所有异步功能
        .build() // Create it  创建它
        .unwrap() 
        .block_on(async { // Blocks current thread until tasks completed 阻塞当前线程，直到任务完成
            let account = Account { // Storage account 存储账户
                user: "your_email@example.com".to_string(),
                password: "your_password".to_string(),
            };
            let client = new_client()?; // New api client 创建API客户端
            let auth = login::login(&account, client.clone()).await?; // Verify account 验证账户
            let user_proxies = get_user_proxies::get_user_proxies(&auth, client).await?; // Get user proxies 获取用户隧道
            println!("{user_proxies:#?}"); // Print it 打印它
            Ok(())
        })
}
