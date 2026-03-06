# Overlayward

AI Agent 安全沙箱

## 这是什么

Overlayward 是一个让 AI 编程 Agent 的所有操作都在完全隔离环境中运行的安全沙箱系统。当前版本是 **Mock API 服务器**，实现了全部 4 种接入协议（REST / gRPC / MCP / CLI），只是用于测试。

Mock 后端基于内存存储，不需要 VM 或容器环境即可运行。项目当前为骨架阶段，后续可逐步替换为真实后端。

## 快速开始

### 前置条件

- Rust 1.75+（推荐 1.94+）
- protoc（protobuf 编译器）

**安装 protoc：**

Windows (PowerShell)：
```powershell
Invoke-WebRequest -Uri "https://github.com/protocolbuffers/protobuf/releases/download/v29.3/protoc-29.3-win64.zip" -OutFile protoc.zip
Expand-Archive protoc.zip -DestinationPath "$env:USERPROFILE\.local\protoc"
[Environment]::SetEnvironmentVariable("PROTOC", "$env:USERPROFILE\.local\protoc\bin\protoc.exe", "User")
# 重启终端使环境变量生效
```

Linux：
```bash
sudo apt install -y protobuf-compiler
```

macOS：
```bash
brew install protobuf
```

### 编译

```bash
cargo build
```

Release 编译（启用 LTO 优化）：
```bash
cargo build --release
```

### 启动服务器

```bash
# 同时启动 REST API (8420) + gRPC (8421) + MCP HTTP (8422)
overlayward serve

# 自定义端口
overlayward serve --rest-port 9000 --grpc-port 9001 --mcp-port 9002

# 启用详细日志
RUST_LOG=info overlayward serve
```

启动后你会看到：
```
INFO overlayward: Overlayward servers started — REST :8420 | gRPC :8421 | MCP :8422
```

### MCP 模式

**HTTP 传输（推荐）：** `overlayward serve` 启动时自动在 :8422 提供 MCP Streamable HTTP 端点。

在 Agent 框架中配置（HTTP 模式）：
```json
{
  "mcpServers": {
    "overlayward": {
      "url": "http://localhost:8422/mcp"
    }
  }
}
```

测试 MCP HTTP 端点：
```bash
curl -X POST http://localhost:8422/mcp \
  -H "Content-Type: application/json" \
  -H "Accept: application/json, text/event-stream" \
  -d '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"test","version":"0.1.0"}}}'
```

**stdio 传输：** 也可作为独立 MCP Server 运行（Agent 框架通过 stdio 调用）：
```bash
overlayward mcp-server
```

在 Agent 框架中配置（stdio 模式）：
```json
{
  "mcpServers": {
    "overlayward": {
      "command": "overlayward",
      "args": ["mcp-server"]
    }
  }
}
```

## 使用方法

### 认证

所有 API 请求需要 Bearer Token。预定义了 4 种角色的 Token：

| Token | 角色 | 说明 |
|-------|------|------|
| `ow-agent-token` | Agent | 只能使用 Agent 可见的操作 |
| `ow-user-token` | User | 可使用上传、审计等高级操作 |
| `ow-admin-token` | Admin | 可修改网络默认策略等管理操作 |
| `ow-human-token` | Human | 可进行审批等仅人类操作 |

### REST API

**基础 URL：** `http://localhost:8420/api/v1`

创建沙箱：
```bash
curl -X POST http://localhost:8420/api/v1/sandboxes \
  -H "Authorization: Bearer ow-agent-token" \
  -H "Content-Type: application/json" \
  -d '{"name": "dev-frontend", "cpu": 4, "memory": "8GB"}'
```

列出沙箱：
```bash
curl -H "Authorization: Bearer ow-agent-token" \
  http://localhost:8420/api/v1/sandboxes
```

启动沙箱：
```bash
curl -X POST http://localhost:8420/api/v1/sandboxes/sb-xxxxxxxx/start \
  -H "Authorization: Bearer ow-agent-token"
```

在沙箱内执行命令：
```bash
curl -X POST http://localhost:8420/api/v1/sandboxes/sb-xxxxxxxx/exec \
  -H "Authorization: Bearer ow-agent-token" \
  -H "Content-Type: application/json" \
  -d '{"command": "npm install express"}'
```

保存快照：
```bash
curl -X POST http://localhost:8420/api/v1/sandboxes/sb-xxxxxxxx/snapshots \
  -H "Authorization: Bearer ow-agent-token" \
  -H "Content-Type: application/json" \
  -d '{"name": "before-refactor"}'
```

**完整路由表：** 共 38 条路由，详见 `F:\Code\02-architecture\protocols\rest-api.md`

### CLI

CLI 作为 REST API 的客户端，连接到正在运行的服务器。

```bash
# 设置 Token（也可以每次用 --token 参数）
export OVERLAYWARD_TOKEN=ow-agent-token

# 沙箱管理
overlayward create --name dev --cpu 4 --memory 8GB
overlayward list
overlayward list --status running
overlayward info sb-xxxxxxxx
overlayward start sb-xxxxxxxx
overlayward pause sb-xxxxxxxx
overlayward resume sb-xxxxxxxx
overlayward stop sb-xxxxxxxx
overlayward destroy sb-xxxxxxxx

# 命令执行
overlayward exec sb-xxxxxxxx -- npm install express
overlayward exec sb-xxxxxxxx --workdir /app --timeout 60s -- cargo build

# 快照
overlayward snapshot save sb-xxxxxxxx --name checkpoint
overlayward snapshot list sb-xxxxxxxx
overlayward snapshot restore sb-xxxxxxxx snap-xxxxxxxx
overlayward snapshot diff sb-xxxxxxxx snap-001 current

# 网络策略
overlayward network get sb-xxxxxxxx
overlayward network allow sb-xxxxxxxx --domain "api.github.com" --ports 443

# 文件操作
overlayward file read sb-xxxxxxxx /app/main.rs
overlayward file list sb-xxxxxxxx /app
overlayward file write sb-xxxxxxxx /app/config.toml ./local-config.toml

# 资源
overlayward resource usage sb-xxxxxxxx

# 审计（需要 user 以上权限）
overlayward --token ow-user-token audit sb-xxxxxxxx --level command

# 审批（需要 human 权限）
overlayward --token ow-human-token approval list
overlayward --token ow-human-token approval decide apr-xxxxxxxx --approve

# JSON 输出
overlayward --output json list
```

### gRPC

服务端口 `localhost:8421`，package `overlayward.v1`。

proto 文件位于 `proto/overlayward/v1/`，包含 11 个 Service：
SandboxService, SnapshotService, NetworkService, ExecService,
FileService, VolumeService, AuditService, ResourceService,
InterService, ApprovalService, EventService

使用 grpcurl 测试：
```bash
grpcurl -plaintext -d '{"name":"test","cpu":2}' \
  -H "authorization: Bearer ow-agent-token" \
  localhost:8421 overlayward.v1.SandboxService/Create
```

### Rust SDK

```rust
use overlayward_sdk::{Client, Config};

let client = Client::new(Config {
    endpoint: "http://localhost:8421".into(),
    token: "ow-agent-token".into(),
}).await?;

let sandbox = client.sandbox().create(CreateSandboxRequest {
    name: "dev".into(), cpu: 4, ..Default::default()
}).await?;

client.sandbox().start(&sandbox.sandbox_id).await?;
```

### C SDK

```c
#include "overlayward.h"

OwClient *client = ow_client_new("http://localhost:8421", "ow-agent-token");
OwSandbox *sb = ow_sandbox_create(client, "dev", 4, "8GB");
ow_sandbox_start(client, sb->sandbox_id);

ow_sandbox_free(sb);
ow_client_free(client);
```

头文件自动生成在 `sdk/c/overlayward.h`。

## 项目结构

```
overlayward/
├── src/main.rs              # 入口：serve / mcp-server / CLI
├── proto/overlayward/v1/    # 12 个 protobuf 定义文件
├── crates/
│   ├── ow-types/            # 领域模型、错误码、认证类型
│   ├── ow-core/             # 服务 trait 契约 + mock 后端
│   ├── ow-rest/             # REST API (axum, 38 路由)
│   ├── ow-grpc/             # gRPC (tonic, 11 Service)
│   ├── ow-mcp/              # MCP Server (19 Tool, stdio + HTTP)
│   ├── ow-sdk/              # Rust SDK
│   ├── ow-ffi/              # C FFI
│   ├── ow-cli/              # CLI 客户端
│   └── ow-macros/           # 过程宏
├── sdk/
│   ├── c/overlayward.h      # 自动生成的 C 头文件
│   ├── go/                  # Go SDK (make gen-go 生成)
│   ├── python/              # Python SDK (make gen-python 生成)
│   └── cpp/                 # C++ SDK (make gen-cpp 生成)
└── Makefile                 # SDK 代码生成
```

## 生成其他语言 SDK

proto 文件是全部 SDK 的事实来源。安装对应语言的 protoc 插件后：

```bash
make gen-go       # Go gRPC stubs
make gen-python   # Python gRPC stubs
make gen-cpp      # C++ gRPC stubs
make gen-c        # C 头文件 (cbindgen)
make gen-all      # 全部
```

## 当前阶段说明

这是 Mock 阶段：
- 所有 API 返回模拟数据，沙箱不会真正创建 VM
- 文件系统使用内存 HashMap 模拟
- 命令执行返回 `(mock) executed: <command>`
- 网络策略中，内网地址自动拒绝，非白名单域名触发审批流程
- 快照 diff 返回硬编码结果

项目按生产架构设计。全部业务逻辑通过 Rust trait 抽象，mock 只是当前的一种实现。后续用真实 VMM/审计/网络策略引擎替换 mock 时，协议层代码不需要任何改动。

## License

MIT
