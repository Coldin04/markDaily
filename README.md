# markDaily

markDaily 是一款轻量级在线文档查看器，支持通过 API 上传 Markdown 文本，并在身份验证后于前端展示。

## Monorepo 结构

- `apps/web`：SvelteKit 前端（文档列表、登录认证、Markdown 展示）
- `apps/api`：Rust + Axum + sqlx 后端（认证、Markdown CRUD、后续 AI 查询接口）

## 快速开始

### 前端（SvelteKit）

```bash
cd /tmp/workspace/Coldin04/markDaily/apps/web
npm install
npm run dev
```

### 后端（Rust + Axum + sqlx + SQLite）

```bash
cd /tmp/workspace/Coldin04/markDaily/apps/api
cargo run
```

默认会启动在 `http://127.0.0.1:3000`，并预留如下基础接口路径：

- `GET /health`
- `GET /api/auth`
- `GET /api/docs`
- `GET /api/ai`
