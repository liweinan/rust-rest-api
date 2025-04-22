# Rust REST API 示例

这是一个使用 Rust 和 reqwest 库调用 REST API 的示例项目。

## 功能

- 使用 reqwest 发送 HTTP 请求
- 支持 GET 和 POST 请求
- 使用 serde 进行 JSON 序列化和反序列化
- 异步处理 HTTP 请求

## 依赖项

- reqwest: HTTP 客户端
- tokio: 异步运行时
- serde: 序列化和反序列化
- serde_json: JSON 处理

## 使用方法

1. 确保已安装 Rust 和 Cargo
2. 克隆此仓库
3. 运行项目：

```bash
cargo run
```

## 示例说明

项目包含两个主要功能：
1. 获取指定 ID 的帖子（GET 请求）
2. 创建新帖子（POST 请求）

代码使用 JSONPlaceholder API 作为示例，这是一个免费的在线 REST API 测试服务。 