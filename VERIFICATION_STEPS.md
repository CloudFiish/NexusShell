# CodeBuddy Code 验证步骤

## 验证目标

检查 CodeBuddy Code 是否支持：
1. WebSocket 通信
2. JSON 格式输出
3. MCP 协议
4. 特殊标记输出

## 手动验证步骤

### 步骤 1: 检查安装

运行以下命令检查 CodeBuddy Code 是否已安装：
```bash
codebuddy --version
```

如果未安装，运行：
```bash
npm install -g @tencent-ai/codebuddy-code
```

### 步骤 2: 查看帮助文档

```bash
codebuddy --help
```

检查是否有：
- WebSocket 相关选项
- MCP 相关命令
- 输出格式控制选项

### 步骤 3: 检查 MCP 支持

```bash
codebuddy mcp --help
codebuddy mcp list
```

### 步骤 4: 测试输出格式

```bash
codebuddy -p "用 TypeScript 写一个 Hello World 函数"
```

观察输出是：
- 纯文本格式
- JSON 格式
- 包含特殊标记

### 步骤 5: 测试交互模式

```bash
codebuddy
```

在交互式模式中：
1. 输入: "用 TypeScript 写一个简单的 User 接口"
2. 观察输出格式
3. 输入: "添加 email 字段"
4. 观察输出格式

## 验证结果分析

根据验证结果，选择通信方案：

### 方案 A: 支持 WebSocket
- 特征: 帮助文档中有 WebSocket 选项
- 实现: 使用 WebSocket 通信
- 优先级: 最高

### 方案 B: 支持 JSON 输出
- 特征: --print 模式输出 JSON 格式
- 实现: 解析 stdout 的 JSON
- 优先级: 高

### 方案 C: 支持特殊标记
- 特征: 输出中包含 @@UI_DATA@@ 等标记
- 实现: 正则表达式解析
- 优先级: 中

### 方案 D: 支持文件通信
- 特征: 上述都不支持
- 实现: 使用临时文件
- 优先级: 低

## 验证检查清单

- [ ] CodeBuddy Code 版本
- [ ] 支持 WebSocket
- [ ] 支持 JSON 输出
- [ ] 支持特殊标记
- [ ] 支持 MCP 协议
- [ ] 输出格式确认
- [ ] 交互模式测试
