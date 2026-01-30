# 文件重命名计划

**创建日期**: 2026-01-29
**目标**: 将所有指导文件用中文命名（README.md 除外）

---

## 📋 当前文件列表

### 已存在的文件（需要保留为英文）

1. **README.md** ✅ 保持英文
2. **架构.md** ✅ 保持英文
3. **CODEBUDDY.md** ✅ 保持英文

### 需要重命名的文件（10 个）

#### 项目状态文档（4 个）

| 原文件名 | 新文件名（中文） | 说明 |
|---------|---------------|------|
| `PROJECT_STATUS_REPORT.md` | `项目状态报告.md` | 项目整体进度 |
| `NEXT_STEPS_PLAN.md` | `下一步计划.md` | 实施计划 |
| `NEXT_STEPS_PLAN_UPDATED.md` | `下一步计划更新.md` | 更新的实施计划 |
| `UPDATED_STATUS_AND_PLAN.md` | `新状态和计划.md` | 更新的状态和计划 |

#### 验证相关文档（5 个）

| 原文件名 | 新文件名（中文） | 说明 |
|---------|---------------|------|
| `CODEBUDDY_VERIFICATION_GUIDE.md` | `CodeBuddy 验证指南.md` | CodeBuddy 验证指南 |
| `VERIFICATION_STEPS.md` | `验证步骤.md` | 验证步骤 |
| `VERIFICATION_QUICK.md` | `快速手动验证.md` | 快速手动验证 |
| `VERIFICATION_SUMMARY.md` | `验证总结.md` | 验证总结 |

#### 通信方案文档（3 个）

| 原文件名 | 新文件名（中文） | 说明 |
|---------|---------------|------|
| `ACP_COMMUNICATION_PLAN.md` | `ACP 通信方案.md` | ACP 通信方案分析 |
| `ACP_TRANSPORT_GUIDE.md` | `ACP 传输层指南.md` | ACP 传输层实施指南 |

#### 实施相关文档（5 个）

| 原文件名 | 新文件名（中文） | 说明 |
|---------|---------------|------
| `UPDATED_STATUS_AND_PLAN.md` | `更新状态和计划.md` | 更新的状态和计划 |
| `FINAL_STATUS_SUMMARY.md` | `最终状态总结.md` | 最终状态总结 |
| `PROJECT_COMPLETION_SUMMARY.md` | `项目完成总结.md` | 项目完成总结 |
| `IMPLEMENTATION_PLAN.md` | `实施计划.md` | 实施计划 |
| `NEXT_STEPS_PLAN.md` | `下一步计划.md` | 下一步计划 |

#### 脚本文件（2 个）

| 原文件名 | 新文件名（中文） | 说明 |
|---------|---------------|------
| `verify-codebuddy-stable.bat` | `稳定版验证脚本.bat` | 稳定版验证脚本 |
| `git-commit.bat` | `git 提交脚本.bat` | Git 提交脚本 |

---

## 🎯 批处理方案

### 方案 1: 手动重命名（推荐，精确控制）

#### 步骤 1: 打开文件资源管理器

1. 导航到：`d:\Project\NexusShell\`
2. 选择要重命名的文件

#### 步骤 2: 重命名文件

**操作方法**:
- 选中文件
- 按 `F2` 重命名
- 输入中文名称
- 按 `Enter` 确认

**需要重命名的文件列表**:

**项目状态文档**:
- [ ] `PROJECT_STATUS_REPORT.md` → `项目状态报告.md`
- [ ] `NEXT_STEPS_PLAN.md` → `下一步计划.md`
- [ ] `NEXT_STEPS_PLAN_UPDATED.md` → `下一步计划更新.md`
- [ ] `UPDATED_STATUS_AND_PLAN.md` → `新状态和计划.md`

**验证相关文档**:
- [ ] `CODEBUDDY_VERIFICATION_GUIDE.md` → `CodeBuddy 验证指南.md`
- [ ] `VERIFICATION_STEPS.md` → `验证步骤.md`
- [ ] `VERIFICATION_QUICK.md` → `快速手动验证.md`
- [ ] `VERIFICATION_SUMMARY.md` → `验证总结.md`

**通信方案文档**:
- [ ] `ACP_COMMUNICATION_PLAN.md` → `ACP 通信方案.md`
- [ ] `ACP_TRANSPORT_GUIDE.md` → `ACP 传输层指南.md`

**实施相关文档**:
- [ ] `UPDATED_STATUS_AND_PLAN.md` → `更新状态和计划.md`
- [ ] `FINAL_STATUS_SUMMARY.md` → `最终状态总结.md`
- [ ] `IMPLEMENTATION_PLAN.md` → `实施计划.md`
- [ ] `NEXT_STEPS_PLAN.md` → `下一步计划.md`

### 方案 2: 使用 PowerShell 批量重命名（快速，自动化）

#### 命令

```powershell
# 设置工作目录
cd d:\Project\NexusShell

# 项目状态文档
Rename-Item -Path "PROJECT_STATUS_REPORT.md" -NewName "项目状态报告.md"
Rename-Item -Path "NEXT_STEPS_PLAN.md" -NewName "下一步计划.md"
Rename-Item -Path "NEXT_STEPS_PLAN_UPDATED.md" -NewName "下一步计划更新.md"
Rename-Item -Path "UPDATED_STATUS_AND_PLAN.md" -NewName "新状态和计划.md"

# 验证相关文档
Rename-Item -Path "CODEBUDDY_VERIFICATION_GUIDE.md" -NewName "CodeBuddy 验证指南.md"
Rename-Item -Path "VERIFICATION_STEPS.md" -NewName "验证步骤.md"
Rename-Item -Path "VERIFICATION_QUICK.md" -NewName "快速手动验证.md"
Rename-Item -Path "VERIFICATION_SUMMARY.md" -NewName "验证总结.md"

# 通信方案文档
Rename-Item -Path "ACP_COMMUNICATION_PLAN.md" -NewName "ACP 通信方案.md"
Rename-Item -Path "ACP_TRANSPORT_GUIDE.md" -NewName "ACP 传输层指南.md"

# 实施相关文档
Rename-Item -Path "FINAL_STATUS_SUMMARY.md" -NewName "最终状态总结.md"
Rename-Item -Path "IMPLEMENTATION_PLAN.md" -NewName "实施计划.md"
```

**执行步骤**:
1. 打开 PowerShell
2. 复制上述命令
3. 粘贴到 PowerShell
4. 执行命令

---

## 📋 需要更新的文件引用

### README.md
**需要更新的引用**:
- `[VERIFICATION_STEPS.md](./验证步骤.md)`
- `[IMPLEMENTATION_PLAN.md](./实施计划.md)`
- `[ACP通信方案.md](./ACP通信方案.md)`
- `[ACP传输层指南.md](./ACP传输层指南.md)`

### 架构.md
**可能需要更新的引用**:
- `[CodeBuddy 验证指南.md](./CodeBuddy 验证指南.md)`
- `[ACP通信方案.md](./ACP通信方案.md)`

### CODEBUDDY.md
**可能需要更新的引用**:
- `[ACP传输层指南.md](./ACP传输层指南.md)`

---

## 🎯 快速操作指南

### 立即执行（推荐）

**选项 1**: 使用 PowerShell 批量重命名（2 分钟）
- 复制 PowerShell 命令
- 在 PowerShell 中执行
- 验证文件是否已重命名成功

**选项 2**: 手动逐个重命名（5-10 分钟）
- 打开文件资源管理器
- 逐个重命名文件
- 验证文件引用是否需要更新

---

## 📝 完成检查清单

### 文件重命名
- [ ] 所有 .md 文件已重命名为中文（除 README.md）
- [ ] 文件夹结构清晰
- [ ] 文件名有意义且一致

### 文件引用更新
- [ ] README.md 中的链接已更新为新文件名
- [ ] 其他文档中的交叉引用已更新
- [ ] 链接有效性验证（Markdown 链接检查）

### 文档一致性
- [ ] 所有文档中文名称统一
- [ ] 文档说明清晰
- [ ] 文档格式一致

---

## 🔄 后续行动

### 立即执行
1. [ ] 重命名所有指导文件为中文名称
2. [ ] 验证文件引用
3. [ ] 更新相关文档中的链接
4. [ ] 测试 Markdown 链接有效性

### 短期执行
1. [ ] 完成 ACP 通信层实施
2. [ ] 前端 ACP 事件集成
3. [ ] 流式数据处理实现
4. [ ] 前端组件完全集成

---

**完成文件重命名后，请告诉我！**

**我将：**
1. ✅ 验证所有文件重命名成功
2. ✅ 检查文件引用是否需要更新
3. ✅ 提供更新后的文件列表
4. ✅ 确认项目文档结构

---

**请告诉我您想：**
1. 立即执行文件重命名（我提供详细命令）
2. 手动重命名（我提供检查清单）
3. 先了解更多细节（ACP 协议、ndjson 库等）
4. 其他需求
