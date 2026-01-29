# 文件重命名手动指南

**创建日期**: 2026-01-29
**操作**: 将所有指导文件重命名为中文名称（README.md 除外）

---

## 📋 需要重命名的文件清单

### 项目级文档（4个文件）
```
1. PROJECT_STATUS_REPORT.md → 项目状态报告.md
2. NEXT_STEPS_PLAN.md → 下一步计划.md
3. IMPLEMENTATION_PLAN.md → 实施计划.md
4. UPDATED_STATUS_AND_PLAN.md → 新状态和计划.md
```

### 验证相关文档（2个文件）
```
5. QUICK_MANUAL_VERIFICATION.md → 快速手动验证.md
6. FINAL_STATUS_SUMMARY.md → 最终状态总结.md
```

### 通信方案文档（3个文件）
```
7. ACP_COMMUNICATION_PLAN.md → ACP 通信方案.md
8. ACP_TRANSPORT_GUIDE.md → ACP 传输层指南.md
9. VERIFICATION_SUMMARY.md → 验证总结.md
```

### 其他文档（1个文件）
```
10. CODEBUDDY_VERIFICATION_GUIDE.md → CodeBuddy 验证指南.md
```

---

## 🔧 重命名方法

### 方法 1: 使用资源管理器（推荐）

**步骤**:
1. 打开文件资源管理器
2. 导航到：`d:\Project\NexusShell`
3. 选择文件，按 `F2` 重命名
4. 输入新的中文名称
5. 按 `Enter` 确认

**优点**: 简单直观，不易出错

### 方法 2: 使用 PowerShell 批量重命名

**步骤**:
```powershell
cd d:\Project\NexusShell

# 项目级文档
ren "PROJECT_STATUS_REPORT.md" "项目状态报告.md"
ren "NEXT_STEPS_PLAN.md" "下一步计划.md"
ren "IMPLEMENTATION_PLAN.md" "实施计划.md"
ren "UPDATED_STATUS_AND_PLAN.md" "新状态和计划.md"
ren "FINAL_STATUS_SUMMARY.md" "最终状态总结.md"

# 验证相关文档
ren "QUICK_MANUAL_VERIFICATION.md" "快速手动验证.md"
ren "FINAL_STATUS_SUMMARY.md" "最终状态总结.md"

# 通信方案文档
ren "ACP_COMMUNICATION_PLAN.md" "ACP通信方案.md"
ren "ACP_TRANSPORT_GUIDE.md" "ACP传输层指南.md"
ren "VERIFICATION_SUMMARY.md" "验证总结.md"

# 其他文档
ren "CODEBUDDY_VERIFICATION_GUIDE.md" "CodeBuddy 验证指南.md"
```

### 方法 3: 使用批处理脚本

**文件**: `rename_files.bat` （已创建）

**使用方式**:
1. 双击 `rename_files.bat` 文件
2. 脚本会自动重命名所有 10 个文件

---

## 📝 文件重命名后的目录结构

```
nexus-shell/
├── README.md (保持英文)
├── 架构.md
├── CODEBUDDY.md
├── 控制通道协议规范.md
├── 实施计划.md
├── 下一步计划.md
├── 下一步计划更新.md
├── 新状态和计划.md
├── 最终状态总结.md
├── CodeBuddy 验证指南.md
├── 快速手动验证.md
├── 快速验证步骤.md
├── ACP 通信方案.md
├── ACP传输层指南.md
├── 验证总结.md
└── gitignore
```

---

## 🔄 文件引用更新

由于文件重命名，需要在其他文档中更新引用：

### 需要更新的文件

1. **README.md** - 可能包含文件链接
2. **架构.md** - 可能包含文件引用
3. **CODEBUDDY.md** - 可能包含文件引用
4. **控制通道协议规范.md** - 可能包含文件引用

### 更新方法

手动在各个文档中搜索 `.md` 文件名，并更新为中文名称。

**示例更新**:

**原内容**:
```
请参考 [VERIFICATION_STEPS.md](./VERIFICATION_STEPS.md) 进行验证
```

**更新为**:
```
请参考 [快速验证步骤.md](./快速验证步骤.md) 进行验证
```

---

## ⚠️ 注意事项

1. **README.md 保持英文** - 作为项目根文件，建议保持英文
2. **Git 历史** - 重命名会显示为删除和创建
3. **VS Code** - 重命名后需要关闭并重新打开文件
4. **链接引用** - 检查并更新 Markdown 文件中的链接
5. **Tauri 配置** - 检查是否有路径配置需要更新

---

## ✅ 完成检查清单

- [ ] 所有 .md 文件都已重命名为中文
- [ ] README.md 保持英文
- [ ] 验证了文件引用是否需要更新
- [ ] 文件资源管理器中确认文件重命名成功
- [ ] VS Code 中刷新了文件列表

---

**请选择重命名方式：**
1. **手动重命名** - 使用资源管理器（推荐，精确控制）
2. **批处理脚本** - 使用 `rename_files.bat`（快速，但可能失败）
3. **PowerShell 批量重命名** - 使用提供的 PowerShell 命令

**完成后，请告诉我！**
