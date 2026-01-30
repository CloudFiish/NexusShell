# CodeBuddy Code Verification Guide

## Quick Verification Steps

### Step 1: Check Installation
```bash
codebuddy --version
```

### Step 2: View Help
```bash
codebuddy --help
```

### Step 3: Check MCP Support
```bash
codebuddy mcp --help
codebuddy mcp list
```

### Step 4: Test Output Format
```bash
codebuddy -p "Generate a simple TypeScript interface with id and name fields"
```

### Step 5: Test Interactive Mode
```bash
codebuddy
```

## Analysis Checklist

### Basic Info
- [ ] Version number: ___________
- [ ] Installation path: ___________

### Communication Capabilities
- [ ] Supports WebSocket: Yes / No
- [ ] Supports JSON output: Yes / No
- [ ] Supports special markers: Yes / No
- [ ] Supports MCP: Yes / No

### Command Parameters
- [ ] `--print` / `-p`: Available / Not available
- [ ] `--model`: Available / Not available
- [ ] `--format`: Available / Not available

### Output Format
- [ ] Default format: Plain text / JSON
- [ ] Includes timestamps: Yes / No
- [ ] Includes progress: Yes / No
- [ ] Includes errors: Yes / No

### MCP Support
- [ ] `codebuddy mcp --help`: Available / Not available
- [ ] `codebuddy mcp list`: Success / Fail
- [ ] Server count: ___

## Decision Making

### Option A: WebSocket Support (Best)
If `codebuddy --help` shows WebSocket options:
- Use WebSocket communication
- Implementation: Easy
- Performance: Excellent

### Option B: JSON Output (Good)
If `codebuddy -p` outputs JSON format:
- Parse stdout JSON
- Implementation: Medium
- Performance: Good

### Option C: Special Markers (Acceptable)
If output contains special markers like `@@UI_DATA@@`:
- Parse special markers
- Implementation: Medium
- Performance: Fair

### Option D: File I/O (Fallback)
If none of above:
- Use file system communication
- Implementation: Hard
- Performance: Poor


## 验证结果
```shell
PS D:\Project\NexusShell> codebuddy --version
2.41.6
PS D:\Project\NexusShell> codebuddy --help
Usage: codebuddy|cbc [options] [command] [prompt]

CodeBuddy Code - starts an interactive session by default, use -p/--print for non-interactive output

Arguments:
  prompt                              Your prompt

Options:
  -V, --version                       output the version number
  -d, --debug [filter]                Enable debug mode with optional category filtering (e.g., "api,hooks" or
                                      "!statsig,!file") (default: false)
  --verbose                           Override verbose mode setting from config (default: false)
  -p, --print                         Print response and exit (useful for pipes) (default: false)
  --output-format <format>            Output format (only works with --print): "text" (default), "json" (single
                                      result), or "stream-json" (realtime streaming) (default: "text")
  --input-format <format>             Input format (only works with --print): "text" (default), or "stream-json"
                                      (realtime streaming input) (choices: "text", "stream-json") (default: "text")
  --json-schema <schema>              JSON Schema for structured output validation. Example:
                                      {"type":"object","properties":{"name":{"type":"string"}},"required":["name"]}
  --include-partial-messages          Output raw SSE incremental messages from model requests (only works with
                                      --output-format stream-json) (default: false)
  -y, --dangerously-skip-permissions  Bypass all permission checks. Recommended only for sandboxes with no internet
                                      access. (default: false)
  --permission-mode <mode>            Permission mode to use for the session (choices: "acceptEdits",
                                      "bypassPermissions", "default", "plan")
  --tools <value>                     Restrict which built-in tools CodeBuddy can use. Use "" to disable all, "default"
                                      for all, or comma-separated tool names like "Bash,Edit,Read".
  --allowedTools <tools...>           Comma or space-separated list of tool names to allow (e.g. "Bash(git:*) Edit")
  --disallowedTools <tools...>        Comma or space-separated list of tool names to deny (e.g. "Bash(git:*) Edit")
  --mcp-config <fileOrString>         Load MCP servers from a JSON file or string
  -c, --continue                      Continue the most recent conversation (default: false)
  -r, --resume [sessionId]            Resume a conversation - provide a session ID or interactively select a
                                      conversation to resume (default: false)
  --model <model>                     Model for the current session. Please provide the model ID. Currently supported:
                                      (glm-4.7, glm-4.6, glm-4.6v, deepseek-v3-2-volc, deepseek-v3.1, deepseek-v3-0324,
                                      kimi-k2.5, kimi-k2-thinking)
  --text-to-image-model <model>       Model for text-to-image generation
  --fallback-model <model>            Enable automatic fallback to specified model when default model is overloaded
                                      (only works with --print)
  --add-dir <directories...>          Additional directories to allow tool access to
  --ide                               Automatically connect to IDE on startup if exactly one valid IDE is available
                                      (default: false)
  -v, --version                       Output the version number (default: false)
  --strict-mcp-config                 Only use MCP servers from --mcp-config, ignoring all other MCP configurations
                                      (default: false)
  --session-id <uuid>                 Use a specific session ID for the conversation (supports letters, numbers,
                                      hyphens, underscores, and colons, but must start with a letter or number)
  -H, --header <headers...>           Custom HTTP headers for model requests (format: "Header-Key: Header-Value")
  --serve                             Start in HTTP server mode (non-interactive) (default: false)
  --port <number>                     Port for HTTP server (default: auto-assign)
  --host <string>                     Host address to bind HTTP server (default: 127.0.0.1) (default: "127.0.0.1")
  --acp                               Start in ACP (Agent Client Protocol) mode - enables communication via
                                      stdin/stdout using ndJsonStream (default: false)
  --acp-transport <transport>         ACP transport (only works with --acp): "stdio" (default) or "streamable-http"
                                      (default: "stdio")
  --sandbox [url]                     Run CodeBuddy in sandbox: "container" for Docker/Podman, or E2B API URL (must be
                                      complete URL like https://api.e2b.dev)
  --sandbox-upload-dir                Upload current working directory to sandbox (E2B only) (default: false)
  --sandbox-new                       Force create a new sandbox (ignore cached sandbox) (default: false)
  --sandbox-id <id>                   Connect to specific sandbox ID
  --sandbox-kill                      Kill sandbox on exit (default: keep alive for reuse) (default: false)
  --teleport <value>                  Teleport to existing sandbox (format: session_{cliSessionId})
  --max-turns <number>                Limit the number of agentic turns
  --teleport <value>                  Teleport to existing sandbox (format: 0010${sandboxId}${randomNumber} for e2b,
                                      0020${sandboxId}${randomNumber} for cvm)
  --system-prompt <prompt>            Override the system prompt for this session
  --system-prompt-file <path>         Load system prompt from a file
  --append-system-prompt <prompt>     Append additional content to the system prompt
  --agents <json>                     JSON object defining custom agents (e.g. '{"reviewer": {"description": "Reviews
                                      code", "prompt": "You are a code reviewer"}}')
  --settings <file-or-json>           Path to a settings JSON file or a JSON string to load additional settings from
  --setting-sources <sources>         Comma-separated list of setting sources to load (user, project, local). Default:
                                      user,project,local
  --fork-session                      When resuming, create a new session ID instead of reusing the original (use with
                                      --resume or --continue)
  --replay-user-messages              Re-emit user messages from stdin back on stdout for acknowledgment (only works
                                      with --input-format=stream-json and --output-format=stream-json)
  -h, --help                          display help for command

Commands:
  config                              Manage configuration (eg. codebuddy config set -g theme dark)
  mcp                                 Configure and manage MCP servers
  sandbox                             Manage sandboxes
  plugin                              Manage CodeBuddy Code plugins
  doctor                              Check the health of your CodeBuddy Code auto-updater
  update                              Check for updates and install if available
  install [target]                    Install CodeBuddy Code native build. Use [target] to specify version (latest, or
                                      specific version)
PS D:\Project\NexusShell> codebuddy mcp --help
Usage: codebuddy mcp [options] [command]

Configure and manage MCP servers

Options:
  -h, --help                                     display help for command

Commands:
  add [options] <name> <commandOrUrl> [args...]  Add an MCP server
  remove [options] <name>                        Remove an MCP server
  list                                           List configured MCP servers
  get <name>                                     Get details about an MCP server
  add-json [options] <name> <json>               Add an MCP server (stdio or SSE) with a JSON string
  help [command]                                 display help for command
PS D:\Project\NexusShell> codebuddy mcp list
No MCP servers configured. Use `codebuddy mcp add` to add a server.
PS D:\Project\NexusShell> codebuddy -p "hello world"
Plugins reloaded: index
Plugins reloaded: index
Plugins reloaded: ORIG_HEAD
Plugins reloaded: ORIG_HEAD
Plugins reloaded: known_marketplaces.json
你好!我是 CodeBuddy Code,你的智能编码助手。

我可以帮助你完成各种软件开发任务,包括:
- 编写和修改代码
- 调试和修复错误
- 重构代码
- 添加新功能
- 运行测试
- 解释代码逻辑

请告诉我你需要什么帮助?
```
