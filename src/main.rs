use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    #[serde(rename = "userId")]
    user_id: i32,
    id: i32,
    title: String,
    body: String,
}

fn build_client(proxy: Option<&str>) -> Result<reqwest::Client, Box<dyn Error>> {
    let mut builder = reqwest::Client::builder();
    
    if let Some(proxy_url) = proxy {
        builder = builder
            .proxy(reqwest::Proxy::http(proxy_url)?);
        builder = builder
            .proxy(reqwest::Proxy::https(proxy_url)?);
    }
    
    Ok(builder.build()?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 创建一个可选代理配置的 reqwest 客户端
    let proxy_url = Some("http://localhost:7890"); // 设置为 None 可以禁用代理
    let client = build_client(proxy_url)?;

    // 示例1：获取单个帖子并显示详细信息
    println!("示例1：获取单个帖子");
    let response = client
        .get("https://jsonplaceholder.typicode.com/posts/1")
        .send()
        .await?;

    if response.status().is_success() {
        let post: Post = response.json().await?;
        println!("帖子标题: {}", post.title);
        println!("作者ID: {}", post.user_id);
        println!("内容: {}", post.body);
        println!("-------------------");
    }

    // 示例2：获取用户的所有帖子
    println!("\n示例2：获取用户的所有帖子");
    let user_id = 1;
    let response = client
        .get(&format!("https://jsonplaceholder.typicode.com/posts?userId={}", user_id))
        .send()
        .await?;

    if response.status().is_success() {
        let posts: Vec<Post> = response.json().await?;
        println!("用户 {} 共有 {} 篇帖子", user_id, posts.len());
        for post in posts.iter().take(3) { // 只显示前3篇帖子
            println!("标题: {}", post.title);
            println!("内容预览: {}", post.body.chars().take(50).collect::<String>());
            println!("-------------------");
        }
    }

    // 示例3：创建新帖子并显示结果
    println!("\n示例3：创建新帖子");
    let new_post = Post {
        user_id: 1,
        id: 101,
        title: "测试标题".to_string(),
        body: "这是一个测试帖子的内容，用于演示API的使用。".to_string(),
    };

    let response = client
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&new_post)
        .send()
        .await?;

    if response.status().is_success() {
        let created_post: Post = response.json().await?;
        println!("帖子创建成功！");
        println!("新帖子ID: {}", created_post.id);
        println!("标题: {}", created_post.title);
        println!("内容: {}", created_post.body);
    } else {
        println!("创建帖子失败: {}", response.status());
    }

    // 示例4：更新帖子
    println!("\n示例4：更新帖子");
    let updated_post = Post {
        user_id: 1,
        id: 1,
        title: "更新后的标题".to_string(),
        body: "这是更新后的内容。".to_string(),
    };

    let response = client
        .put("https://jsonplaceholder.typicode.com/posts/1")
        .json(&updated_post)
        .send()
        .await?;

    if response.status().is_success() {
        let updated: Post = response.json().await?;
        println!("帖子更新成功！");
        println!("更新后的标题: {}", updated.title);
        println!("更新后的内容: {}", updated.body);
    } else {
        println!("更新帖子失败: {}", response.status());
    }

    Ok(())
} 