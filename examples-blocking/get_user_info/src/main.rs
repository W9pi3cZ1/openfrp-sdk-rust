use openfrp_sdk::blocking::prelude::*; // Import prelude module 导入预导入模块
use openfrp_sdk::blocking::{
    login, 
    get_user_info,
};

fn main() -> Result<()> {
    let account = Account {
        // Storage account 存储账户
        user: "your_email@example.com".to_string(),
        password: "your_password".to_string(),
    };
    let client = new_client()?; // New api client 创建API客户端
    let auth = login::login(&account, client.clone())?; // Verify account 验证账户
    let user_info = get_user_info::get_user_info(&auth, client)?;
    println!("{auth:#?}\n{user_info:#?}"); // Print request result 打印请求结果
    Ok(())
}
