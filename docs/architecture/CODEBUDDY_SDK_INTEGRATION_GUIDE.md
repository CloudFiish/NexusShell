# CodeBuddy SDK 集成指南

## 前置条件

### Python 环境要求
- Python 3.8 或更高版本
- pip 包管理器

### 安装 SDK

#### 方法 1: 使用 pip 安装 (推荐)
```bash
pip install codebuddy-agent-sdk
```

#### 方法 2: 从源码安装
```bash
git clone https://github.com/codebuddy/codebuddy-agent-sdk.git
cd codebuddy-agent-sdk
pip install -e .
```

#### 方法 3: 使用 requirements.txt
在项目根目录创建 `requirements.txt`:
```
codebuddy-agent-sdk>=0.1.0
```

然后安装:
```bash
pip install -r requirements.txt
```

### 验证安装

```bash
python -c "import codebuddy_agent_sdk; print(codebuddy_agent_sdk.__version__)"
```

---

## 快速开始

### 基本查询

```python
import asyncio
from codebuddy_agent_sdk import query

async def main():
    # 简单查询
    async for message in query(prompt="2+2=?"):
        if message.content:
            for block in message.content:
                if hasattr(block, 'text'):
                    print(block.text)

if __name__ == "__main__":
    asyncio.run(main())
```

### 多轮对话

```python
import asyncio
from codebuddy_agent_sdk import CodeBuddySDKClient

async def main():
    # 创建客户端
    async with CodeBuddySDKClient() as client:
        # 发送消息
        await client.query("你好!")
        
        # 接收响应
        async for msg in client.receive_messages():
            print(msg)

if __name__ == "__main__":
    asyncio.run(main())
```

### 会话管理

```python
import asyncio
from codebuddy_agent_sdk import CodeBuddySDKClient, CodeBuddyAgentOptions

async def main():
    # 创建客户端,并返回 session_id
    async with CodeBuddySDKClient() as client:
        # 第一条消息
        result1 = await client.query("我的名字是 Alice")
        
        # 获取 session_id
        session_id = None
        async for msg in result1:
            if hasattr(msg, 'session_id'):
                session_id = msg.session_id
                break
        
        print(f"Session ID: {session_id}")
        
        # 恢复会话
        result2 = await client.query("我的名字是什么?", session_id=session_id)
        
        async for msg in result2:
            if msg.content:
                for block in msg.content:
                    if hasattr(block, 'text'):
                        print(block.text)

if __name__ == "__main__":
    asyncio.run(main())
```

---

## 消息类型详解

### AssistantMessage

包含 AI 的响应内容。

**字段**:
- `content`: `List[ContentBlock]` - 内容块列表
- `model`: `str` - 使用的模型
- `parent_tool_use_id`: `str | None` - 父级工具使用 ID
- `error`: `str | None` - 错误信息

### ContentBlock 类型

#### TextBlock
纯文本响应。

```python
@dataclass
class TextBlock:
    text: str
```

#### ThinkingBlock
内部推理过程。

```python
@dataclass
class ThinkingBlock:
    thinking: str  # 思考过程
    signature: str  # 签名
```

#### ToolUseBlock
工具调用请求。

```python
@dataclass
class ToolUseBlock:
    id: str  # 工具 ID
    name: str  # 工具名称
    input: dict[str, Any] # 输入参数
```

#### ToolResultBlock
工具执行结果。

```python
@dataclass
class ToolResultBlock:
    tool_use_id: str # 对应的工具使用 ID
    content: str | List[dict[str, Any]] | None # 结果内容
    is_error: bool | None  # 是否出错
```

### ResultMessage

表示请求的最终结果。

**字段**:
- `subtype`: `str` - 结果子类型
- `duration_ms`: `int` - 执行时间(毫秒)
- `is_error`: `bool` - 是否出错
- `num_turns`: `int` - 交互轮数
- `session_id`: `str` - 会话 ID
- `total_cost_usd`: `float | None` - 总成本(美元)
- `result`: `str | None` - 结果文本
- `usage`: `dict[str, Any] | None` - 使用统计

### StreamEvent

实时事件更新。

**字段**:
- `uuid`: `str` - 事件唯一标识
- `session_id`: `str` - 会话 ID
- `event`: `dict[str, Any]` - 事件数据
- `parent_tool_use_id`: `str | None` - 父级工具使用 ID

---

## 错误处理

### 基础异常类

```python
from codebuddy_agent_sdk import CodeBuddySDKError, CLIConnectionError, CLINotFoundError

try:
    async for msg in query(prompt="test"):
        pass
except CLIConnectionError as e:
    print(f"连接失败: {e}")
except CLINotFoundError as e:
    print(f"未找到 CLI: {e}")
except CodeBuddySDKError as e:
    print(f"SDK 错误: {e}")
```

### 常见错误处理

```python
import asyncio
import logging
from codebuddy_agent_sdk import query, CodeBuddySDKError

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

async def safe_query(prompt: str) -> bool:
    try:
        async for message in query(prompt=prompt):
            logger.info(f"收到消息: {message.__class__.__name__}")
        return True
    except CodeBuddySDKError as e:
        logger.error(f"查询失败: {e}")
        return False

async def main():
    success = await safe_query("测试查询")
    if success:
        print("查询成功")
    else:
        print("查询失败")

if __name__ == "__main__":
    asyncio.run(main())
```

---

## 配置选项

### CodeBuddyAgentOptions

```python
from codebuddy_agent_sdk import CodeBuddyAgentOptions

# 选项示例
options = CodeBuddyAgentOptions(
    # CLI 路径
    codebuddy_code_path="/path/to/codebuddy",
    
    # 权限模式
    permission_mode="default",  # "acceptEdits" | "plan" | "bypassPermissions"
    
    # 会话管理
    continue_conversation=True,  # 继续对话
    resume="session_id_here",  # 恢复会话
    fork_session=False,  # 分叉会话
    
    # 执行控制
    max_turns=10,  # 最大轮数
    model="gpt-4",  # 模型选择
    
    # 工作目录
    cwd="/path/to/project",
    
    # 配置源
    setting_sources=["user", "project", "local"],
    
    # 高级选项
    include_partial_messages=True,
    extra_args={"key": "value"},
    stderr=lambda line: print(f"STDERR: {line}")
)

# 使用选项
from codebuddy_agent_sdk import CodeBuddySDKClient

client = CodeBuddySDKClient(options=options)
```

---

## Rust 集成

### Python 适配器设计

#### 概念

使用 Python 作为中间层,通过 Subprocess/stdio 与 CodeBuddy CLI 通信。

#### 架构

```
Rust (Tauri Backend)
    ↓ (spawn)
Python Process (codebuddy-agent-sdk)
    ↓ (stdio)
CodeBuddy CLI
```

#### Rust 代码示例

```rust
use std::process::{Command, Stdio};
use serde_json::{Value};

pub struct CodeBuddyPythonAdapter {
    process: Option<std::process::Child>,
}

impl CodeBuddyPythonAdapter {
    pub async fn start(&mut self, config: &Value) -> Result<(), String> {
        let python_code = r#"
import asyncio
import json
import sys
from codebuddy_agent_sdk import query, CodeBuddySDKClient, CodeBuddyAgentOptions

async def main():
    config = json.loads(sys.stdin.read())
    options = CodeBuddyAgentOptions(**config)
    
    async with CodeBuddySDKClient(options=options) as client:
        async for msg in client.query(prompt=config['prompt']):
            # 发送消息到 stdout (JSON 格式)
            sys.stdout.write(json.dumps(asdict(msg)) + '\n')
            sys.stdout.flush()

asyncio.run(main())
"#;

        let mut cmd = Command::new("python");
        cmd.arg("-c");
        cmd.arg(python_code);
        
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        
        let child = cmd.spawn().map_err(|e| {
            format!("启动 Python 适配器失败: {}", e)
        })?;
        
        self.process = Some(child);
        Ok(())
    }

    pub async fn execute(&mut self, prompt: &str) -> Result<Value, String> {
        if self.process.is_none() {
            return Err("适配器未启动".to_string());
        }

        let process = self.process.as_ref().unwrap();
        
        // 发送配置到 stdin
        let config = json!({
            "prompt": prompt,
        "model": "gpt-4",
            "continue_conversation": true
        });
        
        let stdin = process.stdin.as_ref().ok_or_else(|| {
            return Err("无法获取 stdin".to_string())
        })?;
        
        writeln!(stdin, "{}", config.to_string())
            .map_err(|e| format!("写入 stdin 失败: {}", e))?;
        stdin.flush().map_err(|e| format!("刷新 stdin 失败: {}", e))?;
        
        // 从 stdout 读取响应 (这里需要实现异步读取)
        // ... 读取逻辑
        
        Ok(json!({}))
    }
}
```

---

## 常见问题

### 1. 找不到 CLI

**错误信息**:
```
CLINotFoundError: CLI not found for platform 'windows' architecture 'x86_64'
```

**解决方案**:
- 确保 CodeBuddy CLI 已安装
- 设置环境变量 `CODEBUDDY_CODE_PATH`
- 在代码中指定 `codebuddy_code_path`

### 2. Python 版本不兼容

**错误信息**:
```
Python version must be 3.8 or higher
```

**解决方案**:
```bash
# 检查 Python 版本
python --version

# 升级 Python
# Windows
py -m pip install --upgrade pip
py -m pip install --upgrade python

# macOS/Linux
sudo python3 -m pip install --upgrade pip
sudo python3 -m pip install --upgrade python
```

### 3. 权限被拒绝

**错误信息**:
```
Permission denied: 'write to file'
```

**解决方案**:
- 检查文件/目录权限
- 使用虚拟环境
- 检查 SELinux/AppArmor 设置

### 4. 内存不足

**错误信息**:
```
MemoryError: Unable to allocate memory
```

**解决方案**:
- 减少 `max_turns` 参数
- 清理未使用的会话
- 增加系统内存或优化代码

---

## 性能优化

### 1. 异步处理

始终使用 `async/await` 而不是阻塞调用:

```python
# ✅ 好
async for msg in query(prompt="test"):
    process(msg)

# ❌ 差
messages = list(query(prompt="test"))  # 会等待所有响应
for msg in messages:
    process(msg)
```

### 2. 流式处理

处理大输入时使用流式:

```python
async def large_input_stream():
    """流式发送大输入"""
    async def input_generator():
        for chunk in generate_large_data():
            yield chunk
            await asyncio.sleep(0.1)
    
    async for message in query(prompt=input_generator()):
        # 流式处理响应
        pass
```

### 3. 缓存配置

避免重复加载配置:

```python
from functools import lru_cache

@lru_cache(maxsize=1)
def get_client():
    return CodeBuddySDKClient()

async def query_with_cache(prompt: str):
    async with get_client() as client:
        async for msg in client.query(prompt=prompt):
            pass
```

---

## 调试技巧

### 1. 启用详细日志

```python
import logging
from codebuddy_agent_sdk import CodeBuddySDKClient

logging.basicConfig(
    level=logging.DEBUG,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)

async def main():
    # 客户端会输出详细日志
    async with CodeBuddySDKClient() as client:
        await client.query("test")
```

### 2. 捕获和打印原始 JSON

```python
import json

async def debug_query():
    async for message in query(prompt="test"):
        # 打印原始 JSON 结构
        print("Raw Message:", message)
        print("Dict:", message.__dict__)
        print("Content:", message.content if hasattr(message, 'content') else None)
```

### 3. 验证消息类型

```python
from codebuddy_agent_sdk import AssistantMessage, TextBlock, ThinkingBlock

async def validate_messages():
    async for message in query(prompt="写一段代码"):
        if isinstance(message, AssistantMessage):
            for block in message.content:
                if isinstance(block, TextBlock):
                    assert block.text is not None
                elif isinstance(block, ThinkingBlock):
                    assert block.thinking is not None
                    assert block.signature is not None
```

---

## 下一步

1. ✅ 安装 CodeBuddy SDK
2. ✅ 运行 `codebuddy_sdk_test.py` 验证功能
3. ✅ 开始实现 Python 适配器
4. ✅ 更新 Rust 消息结构
5. ✅ 更新前端组件
6. ✅ 测试完整流程
