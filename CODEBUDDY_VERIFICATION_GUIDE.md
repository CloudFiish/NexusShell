# CodeBuddy Code 通信能力验证指南

**验证日期**: 2026-01-29
**目标**: 确定 CodeBuddy Code CLI 的通信能力，选择合适的通信方案

---

## 📋 验证目标

我们需要验证 CodeBuddy Code 是否支持以下通信方式：

1. **WebSocket 通信** - 最理想的方案
2. **stdin/stderr JSON 输出** - 备选方案 A
3. **文件系统通信** - 备选方案 B
4. **命令行参数控制** - 备选方案 C

---

## 🔍 验证步骤

### 步骤 1: 自动验证脚本

**运行验证脚本**:
```bash
cd d:\Project\NexusShell
verify-codebuddy.bat
```

这个脚本会自动执行以下检查：
- ✓ 检查 CodeBuddy Code 安装状态
- ✓ 查看帮助文档
- ✓ 检查 MCP 相关命令
- ✓ 测试基本命令执行
- ✓ 查看 MCP 服务器列表
- ✓ 测试交互式模式

---

### 步骤 2: 手动验证通信方式

#### 2.1 验证 WebSocket 支持

**检查命令**:
```bash
codebuddy --help | findstr /i "websocket"
```

**预期结果**:
- 如果找到 "websocket" → **支持 WebSocket**
- 如果未找到 → **不支持 WebSocket**

**验证方式**:
```bash
# 检查是否有 WebSocket 相关的配置
codebuddy config list 2>nul | findstr /i "port ws socket"
```

#### 2.2 验证 stdin/stderr JSON 输出

**测试命令**:
```bash
# 测试 --print 模式的输出格式
codebuddy -p "生成一个 TypeScript 接口" > output_test.txt
type output_test.txt
```

**分析输出**:
- 如果输出是纯文本 → 不支持 JSON 输出
- 如果输出是 JSON 格式 → **支持 JSON 输出**
- 如果输出包含特殊标记（如 `@@UI_DATA@@`）→ **支持标记式输出**

#### 2.3 验证 MCP 协议支持

**检查命令**:
```bash
codebuddy mcp --help
```

**查看 MCP 服务器**:
```bash
codebuddy mcp list
```

**预期结果**:
- 如果命令存在且能执行 → **支持 MCP 协议**
- 这意味着我们可以通过 MCP 获取 Skill 列表

#### 2.4 验证命令行参数

**检查常用参数**:
```bash
# 检查是否有输出格式控制参数
codebuddy --help | findstr /i "format json output"
```

**常见参数**:
- `--print` / `-p`: 打印并退出（非交互式）
- `--model`: 选择模型
- `--format`: 输出格式
- `--output`: 输出文件

---

### 步骤 3: 测试实际的 Skill 执行

#### 3.1 测试代码生成

**命令**:
```bash
codebuddy -p "用 TypeScript 写一个简单的 Hello World 函数"
```

**观察输出**:
1. 输出格式（纯文本/JSON）
2. 是否包含执行状态
3. 是否包含元数据
4. 输出速度

#### 3.2 测试多轮对话

**命令**:
```bash
# 创建输入文件
echo "生成一个 User 接口" > input.txt
echo "添加 email 字段" >> input.txt

# 执行
codebuddy -f input.txt -p
```

**观察输出**:
- 是否支持多轮输入
- 输出是否区分每次对话

---

## 📊 验证结果分析

### 场景 A: CodeBuddy Code 支持 WebSocket

**特征**:
- ✓ `codebuddy --help` 显示 WebSocket 相关选项
- ✓ 有配置项可以设置 WebSocket 端口
- ✓ 支持启动 WebSocket 服务器

**推荐方案**: 使用 WebSocket 通信
- **优点**:
  - 实时双向通信
  - 传输效率高
  - 标准化协议
- **实现难度**: 低
- **风险**: 低

**实施步骤**:
1. 启动 CodeBuddy Code 时启用 WebSocket 模式
2. CodeBuddyAdapter 连接到 WebSocket 服务器
3. 发送 Control Channel 消息
4. 接收实时响应

---

### 场景 B: CodeBuddy Code 不支持 WebSocket，但支持 JSON 输出

**特征**:
- ✗ 不支持 WebSocket
- ✓ `--print` 模式输出 JSON 格式
- ✓ 输出包含结构化数据（如 skill_list, execution_result）

**推荐方案**: 解析 stdout/stderr 的 JSON 输出
- **优点**:
  - 不需要修改 CodeBuddy Code
  - 实现相对简单
- **缺点**:
  - 只能单向通信
  - 无法实时获取进度
- **实现难度**: 中
- **风险**: 中

**实施步骤**:
1. 使用 `--print` 模式执行命令
2. 捕获 stdout 输出
3. 解析 JSON 格式
4. 提取结构化数据

**示例输出格式**:
```json
{
  "type": "skill_list",
  "skills": [...]
}
```

或者：
```json
{
  "type": "data_chunk",
  "data": {...},
  "progress": {...}
}
```

---

### 场景 C: CodeBuddy Code 不支持 JSON 输出，但支持特殊标记

**特征**:
- ✗ 不支持 WebSocket
- ✗ 不支持 JSON 输出
- ✓ 输出中包含特殊标记（如 `@@UI_DATA@@ {...}`）

**推荐方案**: 解析特殊标记
- **优点**:
  - 可以提取结构化数据
  - 不需要修改 CodeBuddy Code
- **缺点**:
  - 依赖特殊标记格式
  - 可能不稳定
- **实现难度**: 中
- **风险**: 中高

**实施步骤**:
1. 捕获 stdout/stderr 输出
2. 使用正则表达式解析特殊标记
3. 提取 JSON 数据
4. 处理解析错误

**示例输出格式**:
```
正在执行技能...

@@UI_DATA@@ {"type": "progress", "current": 1, "total": 10}

执行中...

@@UI_DATA@@ {"type": "data_chunk", "data": {...}}

执行完成！
```

---

### 场景 D: CodeBuddy Code 不支持任何结构化输出

**特征**:
- ✗ 不支持 WebSocket
- ✗ 不支持 JSON 输出
- ✗ 不支持特殊标记
- ✓ 只支持纯文本输出

**推荐方案**: 使用文件系统通信
- **优点**:
  - 完全可控
  - 可以实现双向通信
- **缺点**:
  - 需要管理临时文件
  - 性能较低
  - 需要轮询文件
- **实现难度**: 高
- **风险**: 中

**实施步骤**:
1. 创建输入文件（JSON 格式）
2. 指定输出文件路径
3. CodeBuddy Code 读取输入并写入输出
4. CodeBuddyAdapter 轮询输出文件变化
5. 解析输出文件内容

**示例**:
```bash
# 输入文件
{"command": "execute_skill", "skill_name": "security-review"}

# 输出文件
{"type": "data_chunk", "data": {...}}
```

---

### 场景 E: CodeBuddy Code 支持 MCP 协议

**特征**:
- ✓ `codebuddy mcp` 命令可用
- ✓ 可以列出 MCP 服务器
- ✓ 可以通过 MCP 获取 Skill 列表

**推荐方案**: 结合 MCP + stdout/stderr
- **优点**:
  - 可以动态获取 Skill
  - 标准 MCP 协议
- **缺点**:
  - 仍需要 stdout/stderr 传输执行结果
- **实现难度**: 中
- **风险**: 中

**实施步骤**:
1. 使用 `codebuddy mcp list` 获取 Skill 列表
2. 使用 `codebuddy -p` 执行 Skill
3. 解析 stdout 输出获取结果

---

## 🎯 决策树

```
开始验证
    │
    ▼
支持 WebSocket？
    │
    ├── Yes ──> [方案 A: WebSocket 通信] ✓ 最佳方案
    │
    └── No
         │
         ▼
    支持 JSON 输出？
    │
    ├── Yes ──> [方案 B: 解析 JSON 输出] ✓ 次选方案
    │
    └── No
         │
         ▼
    支持特殊标记？
    │
    ├── Yes ──> [方案 C: 解析特殊标记] ⚠️  中等方案
    │
    └── No
         │
         ▼
    支持 MCP？
    │
    ├── Yes ──> [方案 D: MCP + JSON/标记] ⚠️  混合方案
    │
    └── No
         │
         ▼
    [方案 E: 文件系统通信] ⚠️  最后方案
```

---

## 📝 验证检查清单

请完成以下检查，并记录结果：

### 基础信息
- [ ] CodeBuddy Code 版本号: `_______________`
- [ ] 安装路径: `_______________`
- [ ] 是否已登录: `_______________`

### 通信能力
- [ ] 支持 WebSocket: `Yes / No`
- [ ] 支持 JSON 输出: `Yes / No`
- [ ] 支持特殊标记: `Yes / No`
- [ ] 支持 MCP 协议: `Yes / No`
- [ ] 支持文件 I/O: `Yes / No`

### 命令参数
- [ ] `--print` / `-p`: `可用 / 不可用`
- [ ] `--model`: `可用 / 不可用`
- [ ] `--format`: `可用 / 不可用`
- [ ] `--output`: `可用 / 不可用`
- [ ] `-f` / `--file`: `可用 / 不可用`

### 输出格式
- [ ] 默认输出格式: `纯文本 / JSON / 混合`
- [ ] 是否包含时间戳: `Yes / No`
- [ ] 是否包含进度信息: `Yes / No`
- [ ] 是否包含错误信息: `Yes / No`

### MCP 支持
- [ ] `codebuddy mcp --help`: `可用 / 不可用`
- [ ] `codebuddy mcp list`: `成功 / 失败`
- [ ] MCP 服务器数量: `_______`

---

## 🔧 后续行动

### 如果验证支持 WebSocket:
1. ✅ 使用 WebSocket 通信方案
2. ✅ 完善 CodeBuddyAdapter 的 WebSocket 实现
3. ✅ 实现消息确认和重连机制
4. ✅ 测试双向通信

### 如果验证支持 JSON 输出:
1. ✅ 使用 JSON 解析方案
2. ✅ 实现 stdout JSON 解析器
3. ✅ 实现基于事件的消息分发
4. ✅ 测试单向通信

### 如果验证支持特殊标记:
1. ✅ 使用标记解析方案
2. ✅ 实现正则表达式解析器
3. ✅ 实现消息提取和分发
4. ✅ 测试解析稳定性

### 如果都不支持:
1. ✅ 使用文件系统通信方案
2. ✅ 实现文件轮询机制
3. ✅ 实现输入/输出文件管理
4. ✅ 测试文件 I/O 性能

---

## 📊 验证报告模板

```markdown
# CodeBuddy Code 验证报告

**验证人**: ___________
**验证日期**: 2026-01-29
**CodeBuddy Code 版本**: ___________

## 验证结果

### 通信能力
- WebSocket: [✓/✗]
- JSON 输出: [✓/✗]
- 特殊标记: [✓/✗]
- MCP 协议: [✓/✗]
- 文件 I/O: [✓/✗]

### 推荐方案
**选择方案**: [方案 A/B/C/D/E]
**理由**: ___________

## 测试输出

### 帮助文档
```
[粘贴 codebuddy --help 输出]
```

### MCP 命令
```
[粘贴 codebuddy mcp --help 输出]
```

### 测试命令输出
```
[粘贴 codebuddy -p 测试命令 输出]
```

## 建议

### 短期行动
1. ___________
2. ___________
3. ___________

### 长期规划
1. ___________
2. ___________
3. ___________
```

---

**验证完成后，请将结果告诉我，我将根据验证结果制定具体的实施计划。**
