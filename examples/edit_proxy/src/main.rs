use openfrp_sdk::prelude::*; // Import prelude module 导入预导入模块
use openfrp_sdk::{login, edit_proxy}; // Import login and edit proxy APIs 导入 登录 和 编辑隧道 的API
use tokio; // Import tokio a async modules 导入异步实现模块tokio

fn main() -> Result<()> {
    tokio::runtime::Builder::new_current_thread() // Create a single threads runtime with tokio 用tokio创建一个单线程的异步运行时环境
        .enable_all() // Enable all features 启用所有异步功能
        .build() // Create it  创建它
        .unwrap()
        .block_on(async {
            // Blocks current thread until tasks completed 阻塞当前线程，直到任务完成
            let account = Account {
                // Storage account 存储账户
                user: "your_email@example.com".to_string(),
                password: "your_password".to_string(),
            };
            let client = new_client()?; // New api client 创建API客户端
            let auth = login::login(&account, client.clone()).await?; // Verify account 验证账户
            let proxy = Proxy { // Proxy info 隧道信息
                proxy_id: 0,
                node_id: 44,
                name: "test".to_string(),
                r#type: "tcp".to_string(),
                local_addr: "127.0.0.1".to_string(),
                local_port: "25565".to_string(),
                remote_port: 27388,
                domain_bind: "".to_string(),
                dataGzip: false,
                dataEncrypt: false,
                url_route: "".to_string(),
                host_rewrite: "".to_string(),
                request_from: "".to_string(),
                request_pass: "".to_string(),
                custom: "".to_string(),
            };
            let edit_proxy = edit_proxy::edit_proxy(&auth, &proxy, client).await?;
            println!("{edit_proxy:#?}"); // Print request result 打印请求结果
            Ok(())
        })
}
