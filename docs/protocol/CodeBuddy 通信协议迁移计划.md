# CodeBuddy 通信协议修改计划

## 概述

根据 CodeBuddy 官方文档 (https://www.codebuddy.ai/docs/zh/cli/sdk-python),当前的通信协议实现需要做出修改,以与官方 SDK 保持一致。

## 当前实现与官方文档的差异

### 1. 通信方式

#### 当前实现 (需要修改)
- 使用 `--acp --acp-transport "stdio"` 参数
- 自定义 ndjson 消息格式
- 自定义消息类型: `get_skills`, `execute_skill`, `start`, `data`, `progress`, `error`, `end`

#### 官方方式 (推荐)
- 默认使用 Subprocess/stdio (无需特殊参数)
- 通过 Python SDK 封装,不直接操作底层协议
- 官方消息类型: `Message = UserMessage | AssistantMessage | SystemMessage | ResultMessage | StreamEvent`

### 2. 消息格式

#### 当前实现
```json
{"command":"execute_skill","skill_name":"security-review","input":"Review code security"}
```

#### 官方方式
```python
# 使用 SDK 的 query() 方法
async for message in query(prompt="Review code security"):
    if isinstance(message, AssistantMessage):
        for block in message.content:
            if isinstance(block, TextBlock):
                print(block.text)
```

### 3. 响应格式

#### 当前实现
```json
{"type":"start","session_id":"uuid","skill_name":"security-review"}
{"type":"data","session_id":"uuid","data":"..."}
{"type":"progress","session_id":"uuid","current":1,"total":10}
{"type":"error","session_id":"uuid","code":"ERROR","message":"...","suggestion":"..."}
{"type":"end","session_id":"uuid","success":true,"summary":"..."}
```

#### 官方方式
```python
@dataclass
class AssistantMessage:
    content: list[ContentBlock]  # TextBlock, ThinkingBlock, ToolUseBlock
    model: str
    parent_tool_use_id: str | None = None
    error: str | None = None

@dataclass
class ResultMessage:
    subtype: str
    duration_ms: int
    is_error: bool
    num_turns: int
    session_id: str
    total_cost_usd: float | None = None
    result: str | None = None
```

### 4. 事件类型

#### 当前实现
- `start`: Session started
- `data`: Streaming data chunk
- `progress`: Progress update
- `error`: Error occurred
- `end`: Session ended

#### 官方方式
- `UserMessage`: User input
- `AssistantMessage`: AI response (with ContentBlock)
- `SystemMessage`: System-level data
- `StreamEvent`: Real-time updates during execution
- `ResultMessage`: Final message indicating completion

#### ContentBlock 类型
- `TextBlock`: Plain text response
- `ThinkingBlock`: Internal reasoning
- `ToolUseBlock`: Tool invocation request
- `ToolResultBlock`: Tool execution result

---

## 修改方案

### 方案 A: 完全采用官方 SDK (推荐)

**优势**:
- ✅ 与官方保持一致,稳定性高
- ✅ 自动获得官方更新和 bug 修复
- ✅ 减少维护成本
- ✅ 更好的错误处理和恢复机制

**劣势**:
- ⚠️ 需要集成 Python 环境
- ⚠️ 增加依赖 (Python SDK)
- ⚠️ 可能影响性能 (Python 进程开销)

**实现步骤**:

1. **安装 Python SDK**
   ```bash
   pip install codebuddy-agent-sdk
   ```

2. **创建 Python 适配器** (`src-tauri/src/bridge/codebuddy_python_adapter.rs`)
   ```rust
   use std::process::{Command, Stdio};
   use serde_json::Value;

   pub struct CodeBuddyPythonAdapter {
       process: Option<Child>,
       // ...
   }

   impl CodeBuddyPythonAdapter {
       pub async fn start(&mut self) -> Result<(), AdapterError> {
           let mut cmd = Command::new("python");
           cmd.arg("-c");
           cmd.arg(r#"
               import asyncio
               import sys
               from codebuddy_agent_sdk import query, CodeBuddySDKClient

               async def main():
                   # 从 stdin 读取配置
                   config = json.loads(sys.stdin.read())
                   
                   # 使用官方 SDK
                   async with CodeBuddySDKClient() as client:
                       async for msg in client.query(prompt=config['prompt']):
                           # 输出消息到 stdout (JSON 格式)
                           sys.stdout.write(json.dumps(asdict(msg)) + '\n')
                           sys.stdout.flush()
               
               asyncio.run(main())
           "#);
           
           cmd.stdin(Stdio::piped());
           cmd.stdout(Stdio::piped());
           cmd.stderr(Stdio::piped());

           self.process = Some(cmd.spawn()?);
           Ok(())
       }

       pub async fn execute(&self, prompt: &str) -> Result<String, AdapterError> {
           // 将配置写入 stdin
           let config = json!({"prompt": prompt});
           self.write_to_stdin(&config.to_string())?;
           
           // 从 stdout 读取响应
           self.read_from_stdout().await
       }
   }
   ```

3. **更新消息处理**
   ```rust
   // 解析官方 SDK 的消息格式
   #[derive(Debug, Clone, Serialize, Deserialize)]
   #[serde(tag = "type", rename_all = "lowercase")]
   pub enum SDKMessage {
       User(UserMessage),
       Assistant(AssistantMessage),
       System(SystemMessage),
       Result(ResultMessage),
       Stream(StreamEvent),
   }

   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct AssistantMessage {
       content: Vec<ContentBlock>,
       model: String,
       parent_tool_use_id: Option<String>,
       error: Option<String>,
   }

   #[derive(Debug, Clone, Serialize, Deserialize)]
   #[serde(tag = "type", rename_all = "lowercase")]
   pub enum ContentBlock {
       Text(TextBlock),
       Thinking(ThinkingBlock),
       ToolUse(ToolUseBlock),
       ToolResult(ToolResultBlock),
   }

   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct TextBlock {
       text: String,
   }

   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct ToolUseBlock {
       id: String,
       name: String,
       input: Value,
   }

   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct ToolResultBlock {
       tool_use_id: String,
       content: Option<String>,
       is_error: Option<bool>,
   }

   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct ResultMessage {
       subtype: String,
       duration_ms: i64,
       is_error: bool,
       num_turns: i64,
       session_id: String,
       total_cost_usd: Option<f64>,
       result: Option<String>,
   }
   ```

4. **更新前端事件处理**
   ```typescript
   // 处理新的消息类型
   function handleSDKMessage(message: SDKMessage) {
     switch (message.type) {
       case 'assistant':
         handleAssistantMessage(message);
         break;
       case 'system':
         handleSystemMessage(message);
         break;
       case 'result':
         handleResultMessage(message);
         break;
       case 'stream':
         handleStreamEvent(message);
         break;
     }
   }

   function handleAssistantMessage(message: AssistantMessage) {
     // 处理 AssistantMessage.content 中的 ContentBlock
     message.content.forEach(block => {
       switch (block.type) {
         case 'text':
           // 显示文本
           handleTextBlock(block);
           break;
         case 'thinking':
           // 显示思考过程
           handleThinkingBlock(block);
           break;
         case 'tooluse':
           // 显示工具使用
           handleToolUseBlock(block);
           break;
         case 'toolresult':
           // 显示工具结果
           handleToolResultBlock(block);
           break;
       }
     });
   }
   ```

5. **更新 Session Store**
   ```typescript
   // 更新 Session 数据结构
   interface Session {
     id: string;
     prompt: string;  // 改为 prompt
     messages: SDKMessage[];  // 存储所有 SDK 消息
     model: string;  // 新增: 使用的模型
     duration_ms: number;  // 新增: 执行时间
     total_cost_usd: number | null;  // 新增: 成本
     // ...
   }

   // 更新 DataChunk 为 ContentBlock
   interface ContentChunk {
     type: 'text' | 'thinking' | 'tooluse' | 'toolresult';
     data: any;
     timestamp: string;
   }
   ```

---

### 方案 B: 逆向工程 CLI 协议 (高风险)

**优势**:
- ✅ 不需要 Python 依赖
- ✅ 更轻量,性能更好
- ✅ 完全控制通信过程

**劣势**:
- ❌ 与官方 SDK 不一致
- ❌ 官方更新可能导致兼容性问题
- ❌ 需要维护自定义协议解析器
- ❌ 错误处理复杂

**实现步骤**:
- 通过抓包/日志分析 CLI 实际协议
- 实现与官方一致的消息格式
- 持续跟踪官方 SDK 更新

**不推荐**: 维护成本高,兼容性风险大

---

### 方案 C: 混合方案 (妥协)

**思路**:
- 核心通信使用 Python SDK
- 特定场景使用自定义协议优化性能

**优势**:
- 平衡稳定性和性能
- 可以针对特定场景优化

**劣势**:
- 复杂度增加
- 需要维护两套通信机制

---

## 推荐方案: 方案 A (完全采用官方 SDK)

### 理由

1. **长期稳定性**: 官方 SDK 会持续更新和维护,减少我们的维护成本
2. **功能完整性**: 官方 SDK 提供了所有功能,包括最新的特性
3. **错误处理**: 官方 SDK 有完善的错误处理和恢复机制
4. **社区支持**: 使用官方方式,可以获得社区支持和文档
5. **开发效率**: 不需要逆向工程协议,开发更快

### 实施计划

#### Phase 1: 集成 Python SDK (3 天)

**任务**:
1. [ ] 安装和测试 Python SDK
2. [ ] 创建 Python 适配器
3. [ ] 实现基本通信功能
4. [ ] 更新 Rust 消息结构
5. [ ] 更新前端事件处理

**预期成果**:
- Python 适配器可用
- 基本功能可正常使用

#### Phase 2: 完全替换 (2 天)

**任务**:
1. [ ] 移除旧的 ACP 通信层
2. [ ] 更新所有组件以使用新的消息格式
3. [ ] 更新文档
4. [ ] 测试所有功能

**预期成果**:
- 完全迁移到官方 SDK
- 所有功能正常工作

#### Phase 3: 优化和测试 (2 天)

**任务**:
1. [ ] 性能优化
2. [ ] 错误场景测试
3. [ ] 更新用户文档

**预期成果**:
- 性能满足要求
- 稳定性良好

---

## 详细技术方案

### 1. 消息类型映射

#### 当前 → 官方

| 当前类型 | 官方类型 | 说明 |
|---------|---------|------|
| `execute_skill` | `UserMessage` | 用户输入 |
| `start` | - | 使用 `AssistantMessage` 开始 |
| `data` | `TextBlock` / `ToolResultBlock` | 数据块 |
| `progress` | - | 使用 `StreamEvent` 表示进度 |
| `error` | `AssistantMessage.error` | 错误信息 |
| `end` | `ResultMessage` | 执行完成 |

### 2. Session 管理调整

#### 当前实现
```typescript
interface Session {
  id: string;
  skill_name: string;  // 特定于 Skill
  input: string;
  status: SessionStatus;
  data_chunks: DataChunk[];
  progress?: ProgressInfo;
  error?: ErrorInfo;
  summary?: string;
  success: boolean;
}
```

#### 推荐实现
```typescript
interface Session {
  id: string;  // session_id from ResultMessage
  prompt: string;  // 用户输入的 prompt
  messages: SDKMessage[];  // 所有消息
  model: string;  // 使用的模型
  status: SessionStatus;
  content_blocks: ContentBlock[];  // 所有 ContentBlock
  duration_ms: number;  // 执行时间 (ms)
  total_cost_usd: number | null;  // 成本 (USD)
  num_turns: number;  // 交互轮数
  is_error: boolean;  // 是否出错
}

enum SessionStatus {
  'running',  // 等待响应
  'completed',  // ResultMessage.is_error = false
  'error',  // ResultMessage.is_error = true
}
```

### 3. ContentBlock 处理

#### TextBlock
```typescript
interface TextBlock {
  type: 'text';
  text: string;
}
```

#### ThinkingBlock
```typescript
interface ThinkingBlock {
  type: 'thinking';
  thinking: string;
  signature: string;
}
```

#### ToolUseBlock
```typescript
interface ToolUseBlock {
  type: 'tooluse';
  id: string;
  name: string;
  input: any;
}
```

#### ToolResultBlock
```typescript
interface ToolResultBlock {
  type: 'toolresult';
  tool_use_id: string;
  content?: string;
  is_error?: boolean;
}
```

### 4. 前端更新

#### OmniBox
```typescript
// 不再需要 Skill 匹配,直接发送 prompt
async function handleSubmit() {
  const prompt = input.value.trim();
  
  // 直接发送 prompt,不匹配 Skill
  const sessionId = await executeSkill('assistant', { input: prompt });
  
  // ...
}
```

#### SemanticCanvas
```vue
<template>
  <div class="content-blocks">
    <div v-for="block in session.content_blocks" :key="block">
      <!-- TextBlock -->
      <div v-if="block.type === 'text'" class="text-block">
        {{ block.text }}
      </div>

      <!-- ThinkingBlock -->
      <div v-if="block.type === 'thinking'" class="thinking-block">
        <details>
          <summary>思考过程</summary>
          <pre>{{ block.thinking }}</pre>
        </details>
      </div>

      <!-- ToolUseBlock -->
      <div v-if="block.type === 'tooluse'" class="tool-use-block">
        <div class="tool-name">{{ block.name }}</div>
        <pre>{{ JSON.stringify(block.input, null, 2) }}</pre>
      </div>

      <!-- ToolResultBlock -->
      <div v-if="block.type === 'toolresult'" class="tool-result-block">
        <div v-if="block.is_error" class="error">工具执行失败</div>
        <pre>{{ block.content }}</pre>
      </div>
    </div>
  </div>

  <!-- Result 信息 -->
  <div class="result-info">
    <p>模型: {{ session.model }}</p>
    <p>执行时间: {{ (session.duration_ms / 1000).toFixed(2) }}秒</p>
    <p>交互轮数: {{ session.num_turns }}</p>
    <p v-if="session.total_cost_usd">成本: ${{ session.total_cost_usd.toFixed(4) }}</p>
  </div>
</template>
```

---

## 迁移清单

### 后端 (Rust)

- [ ] 移除 `src-tauri/src/bridge/acp_transport.rs`
- [ ] 移除 `src-tauri/src/bridge/codebuddy_adapter.rs` 中的 ACP 协议
- [ ] 创建 `src-tauri/src/bridge/codebuddy_python_adapter.rs`
- [ ] 更新消息类型定义 (`src-tauri/src/bridge/protocol.rs`)
- [ ] 更新 `src-tauri/src/commands.rs`
- [ ] 移除 `--acp` 相关参数

### 前端 (Vue/TypeScript)

- [ ] 更新 `src/stores/session.ts` 中的 Session 类型
- [ ] 更新 `src/components/OmniBox.vue` 移除 Skill 匹配
- [ ] 更新 `src/components/SemanticCanvas.vue` 的渲染逻辑
- [ ] 更新 `src/composables/useAgent.ts`

### 文档

- [ ] 更新 `ACP_IMPLEMENTATION_SUMMARY.md`
- [ ] 更新 `FRONTEND_INTEGRATION_SUMMARY.md`
- [ ] 创建 `PYTHON_SDK_INTEGRATION_GUIDE.md`
- [ ] 更新 `NEXT_STEPS_PLAN_UPDATED.md`

---

## 风险评估

### 高风险

1. **Python 环境依赖**
   - **风险**: 用户需要安装 Python 和 SDK
   - **缓解**: 打包 Python 环境,或提供一键安装脚本
   - **回退**: 检测 Python 可用性,提示用户

2. **性能影响**
   - **风险**: Python 进程可能有额外开销
   - **缓解**: 缓存常用操作,优化 Python 脚本
   - **监控**: 性能测试,识别瓶颈

### 中等风险

3. **迁移复杂度**
   - **风险**: 需要大量修改现有代码
   - **缓解**: 分阶段迁移,保持向后兼容
   - **测试**: 完整的测试覆盖

4. **学习曲线**
   - **风险**: 团队需要学习新的 API
   - **缓解**: 提供详细文档和示例
   - **培训**: 代码审查和知识分享

### 低风险

5. **兼容性**
   - **风险**: 未来 SDK 版本可能有破坏性变更
   - **缓解**: 使用稳定版本,跟踪更新公告
   - **版本锁定**: 明确指定 SDK 版本

---

## 时间估算

| 任务 | 预估时间 | 备注 |
|------|---------|------|
| 研究 Python SDK | 0.5 天 | 阅读文档,编写示例 |
| 创建 Python 适配器 | 1.5 天 | 实现基本功能 |
| 更新后端消息处理 | 1 天 | Protocol, Commands |
| 更新前端组件 | 1.5 天 | OmniBox, Canvas |
| 测试和调试 | 1 天 | 功能测试 |
| 文档更新 | 0.5 天 | API 文档 |
| **总计** | **6 天** | **约 1 周** |

---

## 成功标准

### 功能性
- [ ] 可以通过 Python SDK 与 CodeBuddy 通信
- [ ] 支持发送 prompt 和接收响应
- [ ] 支持 ContentBlock 渲染 (text, thinking, tooluse, toolresult)
- [ ] 支持 ResultMessage 显示 (duration, cost, etc.)
- [ ] 支持 StreamEvent 实时更新

### 性能
- [ ] 启动时间 < 3 秒
- [ ] 响应延迟 < 200ms
- [ ] 内存使用 < 100MB

### 兼容性
- [ ] 与官方 SDK 版本一致
- [ ] 支持所有主要功能
- [ ] 错误处理完善

### 用户体验
- [ ] UI 响应流畅
- [ ] 错误提示清晰
- [ ] 文档完善

---

## 下一步行动

### 立即行动
1. [ ] 研究 Python SDK 文档和示例
2. [ ] 创建简单的 Python 测试脚本
3. [ ] 验证 Python SDK 功能

### 本周
1. [ ] 实现 Python 适配器基础功能
2. [ ] 更新消息类型定义
3. [ ] 更新前端组件

### 下周
1. [ ] 完成迁移
2. [ ] 全面测试
3. [ ] 更新文档

---

## 附录: 示例代码

### Python 测试脚本
```python
import asyncio
import json
from codebuddy_agent_sdk import query, CodeBuddySDKClient

async def main():
    # 测试基本查询
    print("测试 1: 基本查询")
    async for message in query(prompt="What is 2+2?"):
        print(f"Message type: {message.__class__.__name__}")
        print(f"Content: {message}")
        print()

    # 测试多轮对话
    print("测试 2: 多轮对话")
    async with CodeBuddySDKClient() as client:
        await client.query("Hello!")
        async for msg in client.receive_response():
            print(f"Received: {msg}")
        print()

    # 测试会话恢复
    print("测试 3: 会话恢复")
    async with CodeBuddySDKClient() as client:
        result1 = await client.query("My name is Alice")
        # 提取 session_id
        session_id = None
        async for msg in result1:
            if hasattr(msg, 'session_id'):
                session_id = msg.session_id
                break

        print(f"Session ID: {session_id}")

        # 恢复会话
        if session_id:
            async with CodeBuddySDKClient() as client2:
                await client.query("What's my name?", session_id=session_id)
                async for msg in client2.receive_response():
                    print(f"Resumed: {msg}")
        print()

if __name__ == "__main__":
    asyncio.run(main())
```

### Rust 消息定义
```rust
// src-tauri/src/bridge/protocol.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SDKMessage {
    User(UserMessage),
    Assistant(AssistantMessage),
    System(SystemMessage),
    Result(ResultMessage),
    Stream(StreamEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessage {
    // User messages may not have fields in the SDK
    // We'll adapt as needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantMessage {
    pub content: Vec<ContentBlock>,
    pub model: String,
    #[serde(rename = "parent_tool_use_id")]
    pub parent_tool_use_id: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMessage {
    // System messages
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultMessage {
    pub subtype: String,
    pub duration_ms: i64,
    #[serde(rename = "duration_api_ms")]
    pub duration_api_ms: i64,
    #[serde(rename = "is_error")]
    pub is_error: bool,
    #[serde(rename = "num_turns")]
    pub num_turns: i64,
    #[serde(rename = "session_id")]
    pub session_id: String,
    #[serde(rename = "total_cost_usd")]
    pub total_cost_usd: Option<f64>,
    pub result: Option<String>,
    pub usage: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamEvent {
    pub uuid: String,
    #[serde(rename = "session_id")]
    pub session_id: String,
    pub event: serde_json::Value,
    #[serde(rename = "parent_tool_use_id")]
    pub parent_tool_use_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ContentBlock {
    Text(TextBlock),
    Thinking(ThinkingBlock),
    ToolUse(ToolUseBlock),
    ToolResult(ToolResultBlock),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextBlock {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThinkingBlock {
    pub thinking: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolUseBlock {
    pub id: String,
    pub name: String,
    pub input: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResultBlock {
    #[serde(rename = "tool_use_id")]
    pub tool_use_id: String,
    pub content: Option<String>,
    #[serde(rename = "is_error")]
    pub is_error: Option<bool>,
}
```

---

**总结**: 建议采用官方 Python SDK,虽然需要额外依赖,但可以确保长期稳定性和获得官方支持。预计需要 6 天完成迁移。
