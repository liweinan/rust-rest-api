# Rust REST API 示例

这是一个使用 Rust 和 reqwest 库调用 REST API 的示例项目。项目展示了如何发送 HTTP 请求、处理 JSON 数据，以及配置代理服务器。

## 功能特点

- 使用 reqwest 发送 HTTP 请求（GET、POST、PUT）
- 支持可选的 HTTP/HTTPS 代理配置
- 使用 serde 进行 JSON 序列化和反序列化
- 异步处理 HTTP 请求
- 完整的错误处理

## 示例功能

1. **获取单个帖子**
   - 发送 GET 请求获取指定 ID 的帖子
   - 显示帖子的标题、作者和内容

2. **获取用户的所有帖子**
   - 获取指定用户的所有帖子列表
   - 显示帖子数量和前三篇帖子的预览

3. **创建新帖子**
   - 发送 POST 请求创建新帖子
   - 展示创建成功后的帖子信息

4. **更新帖子**
   - 发送 PUT 请求更新现有帖子
   - 显示更新后的帖子内容

## 依赖项

- reqwest: HTTP 客户端（支持代理配置）
- tokio: 异步运行时
- serde: 序列化和反序列化
- serde_json: JSON 处理

## 代理配置

项目支持配置 HTTP/HTTPS 代理。代理配置是可选的，可以通过以下方式控制：

```rust
// 启用代理
let proxy_url = Some("http://localhost:7890");

// 禁用代理
let proxy_url = None;
```

## 使用方法

1. 确保已安装 Rust 和 Cargo
2. 克隆此仓库
3. （可选）配置代理设置
4. 运行项目：

```bash
cargo run
```

## API 说明

项目使用 [JSONPlaceholder](https://jsonplaceholder.typicode.com/) 作为示例 API，这是一个免费的在线 REST API 测试服务。

## 错误处理

- 所有的 HTTP 请求都包含错误处理
- 使用 Result 类型处理可能的错误
- 显示友好的错误信息

## 代码结构

- `src/main.rs`: 主程序文件，包含所有示例代码
- `Cargo.toml`: 项目配置和依赖管理
- `README.md`: 项目说明文档

## 注意事项

1. 确保在使用代理时，代理服务器已正确配置并运行
2. API 调用可能受到网络状况的影响
3. JSONPlaceholder 是一个测试 API，不会真实保存数据 