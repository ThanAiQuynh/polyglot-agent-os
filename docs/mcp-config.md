# Polyglot-Agent OS: MCP Integration

## Servers
- **Linguistic-Server**: Connects to the Rust gRPC port.
  - *Resources*: `mcp://linguistic/vocabulary/{word}`
  - *Prompts*: `explain-kanji-via-hanviet`
- **Learning-State-Server**: Connects to PostgreSQL/Redis.
  - *Resources*: `mcp://learning/due-items`
  - *Tools*: `update-memory-stability`

## Usage
When I ask: "Viết hàm xử lý cho từ này", Agent sẽ dùng MCP để lấy âm Hán-Việt và độ khó từ Database trước khi sinh code.