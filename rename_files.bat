# 文件重命名列表（中文命名）

**执行日期**: 2026-01-29
**操作**: 将所有指导文件重命名为中文名称

---

## 📋 需要重命名的文件

### 项目级文档
```
PROJECT_STATUS_REPORT.md → 项目状态报告.md
NEXT_STEPS_PLAN.md → 下一步实施计划.md
NEXT_STEPS_PLAN_UPDATED.md → 下一步计划更新.md
UPDATED_STATUS_AND_PLAN.md → 新状态和计划.md
FINAL_STATUS_SUMMARY.md → 最终状态总结.md
```

### 验证相关文档
```
VERIFICATION_QUICK.md → 快速验证步骤.md
QUICK_MANUAL_VERIFICATION.md → 快速手动验证.md
```

### 通信方案文档
```
ACP_COMMUNICATION_PLAN.md → ACP通信方案.md
ACP_TRANSPORT_GUIDE.md → ACP传输层指南.md
```

### 验证总结文档
```
VERIFICATION_SUMMARY.md → 验证总结.md
```

---

## 🔧 重命名方法

### 方法 1: 使用资源管理器手动重命名（推荐）

1. 打开文件资源管理器
2. 导航到 `d:\Project\NexusShell\`
3. 选择要重命名的文件
4. 按 `F2` (重命名）
5. 输入新的中文名称
6. 按 `Enter` 确认

### 方法 2: 使用 PowerShell 批量重命名

```powershell
# 设置工作目录
cd d:\Project\NexusShell

# 重命名文件
Rename-Item -Path "PROJECT_STATUS_REPORT.md" -NewName "项目状态报告.md"
Rename-Item -Path "NEXT_STEPS_PLAN.md" -NewName "下一步实施计划.md"
Rename-Item -Path "NEXT_STEPS_PLAN_UPDATED.md" -NewName "下一步计划更新.md"
Rename-Item -Path "UPDATED_STATUS_AND_PLAN.md" -NewName "新状态和计划.md"
Rename-Item -Path "FINAL_STATUS_SUMMARY.md" -NewName "最终状态总结.md"
Rename-Item -Path "VERIFICATION_QUICK.md" -NewName "快速验证步骤.md"
Rename-Item -Path "QUICK_MANUAL_VERIFICATION.md" -NewName "快速手动验证.md"
Rename-Item -Path "ACP_COMMUNICATION_PLAN.md" -NewName "ACP通信方案.md"
Rename-Item -Path "ACP_TRANSPORT_GUIDE.md" -NewName "ACP传输层指南.md"
Rename-Item -Path "VERIFICATION_SUMMARY.md" -NewName "验证总结.md"
```

### 方法 3: 创建批处理脚本

**文件**: `rename_files.bat`

```batch
@echo off
chcp 65001 > nul
echo ==========================================
echo 文件重命名脚本
echo ==========================================
echo.

cd /d d:\Project\NexusShell

echo [1/11] PROJECT_STATUS_REPORT.md → 项目状态报告.md...
if exist "项目状态报告.md" (
    echo [SKIP] 文件已存在，跳过
) else if exist "PROJECT_STATUS_REPORT.md" (
    ren "PROJECT_STATUS_REPORT.md" "项目状态报告.md"
    echo [DONE] 重命名成功
) else (
    echo [ERROR] 源文件不存在
)
echo.

echo [2/11] NEXT_STEPS_PLAN.md → 下一步实施计划.md...
if exist "下一步实施计划.md" (
    echo [SKIP] 文件已存在，跳过
) else if exist "NEXT_STEPS_PLAN.md" (
    ren "NEXT_STEPS_PLAN.md" "下一步实施计划.md"
    echo [DONE] 重命名成功
) else (
    echo [ERROR] 源文件不存在
)
echo.

echo [3/11] NEXT_STEPS_PLAN_UPDATED.md → 下一步计划更新.md...
if exist "下一步计划更新.md" (
    echo [SKIP] 文件已存在，跳过
) else if exist "NEXT_STEPS_PLAN_UPDATED.md" (
    ren "NEXT_STEPS_PLAN_UPDATED.md" "下一步计划更新.md"
    echo [DONE] 重命名成功
) else (
    echo [ERROR] 源文件不存在
)
echo.

echo [4/11] UPDATED_STATUS_AND_PLAN.md → 新状态和计划.md...
if exist "新状态和计划.md" (
    echo [SKIP] 文件已存在，跳过
) else if exist "UPDATED_STATUS_AND_PLAN.md" (
    ren "UPDATED_STATUS_AND_PLAN.md" "新状态和计划.md"
    echo [DONE] 重命名成功
) else (
    echo [ERROR] 源文件不存在
)
echo.

echo [5/11] FINAL_STATUS_SUMMARY.md → 最终状态总结.md...
if exist "最终状态总结.md" (
    echo [SKIP] 文件已存在，跳过
) else if exist "FINAL_STATUS_SUMMARY.md" (
    ren "FINAL_STATUS_SUMMARY.md" "最终状态总结.md"
    echo [DONE] 重命名成功
) else (
    echo [ERROR] 源文件不存在
)
echo.

echo [6/11] VERIFICATION_QUICK.md → 快速验证步骤.md...
if exist "快速验证步骤.md" (
    echo [SKIP] 文件已存在，跳过
) else if exist "VERIFICATION_QUICK.md" (
    ren "VERIFICATION_QUICK.md" "快速验证步骤.md"
    echo [DONE] 重命名成功
) else (
    echo [ERROR] 源文件不存在
)
echo.

echo [7/11] QUICK_MANUAL_VERIFICATION.md → 快速手动验证.md...
if exist "快速手动验证.md" (
    echo [SKIP] 文件已存在，跳过
) else if exist "QUICK_MANUAL_VERIFICATION.md" (
    ren "QUICK_MANUAL_VERIFICATION.md" "快速手动验证.md"
    echo [DONE] 重命名成功
) else (
    echo [ERROR] 源文件不存在
)
echo.

echo [8/11] ACP_COMMUNICATION_PLAN.md → ACP通信方案.md...
if exist "ACP通信方案.md" (
    echo [SKIP] 文件已存在，跳过
) else if exist "ACP_COMMUNICATION_PLAN.md" (
    ren "ACP_COMMUNICATION_PLAN.md" "ACP通信方案.md"
    echo [DONE] 重命名成功
) else (
    echo [ERROR] 源文件不存在
)
echo.

echo [9/11] ACP_TRANSPORT_GUIDE.md → ACP传输层指南.md...
if exist "ACP传输层指南.md" (
    echo [SKIP] 文件已存在，跳过
) else if exist "ACP_TRANSPORT_GUIDE.md" (
    ren "ACP_TRANSPORT_GUIDE.md" "ACP传输层指南.md"
    echo [DONE] 重命名成功
) else (
    echo [ERROR] 源文件不存在
)
echo.

echo [10/11] VERIFICATION_SUMMARY.md → 验证总结.md...
if exist "验证总结.md" (
    echo [SKIP] 文件已存在，跳过
) else if exist "VERIFICATION_SUMMARY.md" (
    ren "VERIFICATION_SUMMARY.md" "验证总结.md"
    echo [DONE] 重命名成功
) else (
    echo [ERROR] 源文件不存在
)
echo.

echo [11/11] IMPLEMENTATION_PLAN.md → 实施计划.md...
if exist "实施计划.md" (
    echo [SKIP] 文件已存在，跳过
) else if exist "IMPLEMENTATION_PLAN.md" (
    ren "IMPLEMENTATION_PLAN.md" "实施计划.md"
    echo [DONE] 重命名成功
) else (
    echo [ERROR] 源文件不存在
)
echo.

echo.
echo ==========================================
echo 重命名操作完成！
echo ==========================================
echo.
echo 已创建/重命名的文件:
echo.
echo 项目文档:
echo   - 项目状态报告.md
echo   - 下一步实施计划.md
echo   - 下一步计划更新.md
echo   - 新状态和计划.md
echo   - 最终状态总结.md
echo.
echo 验证文档:
echo   - 快速验证步骤.md
echo   - 快速手动验证.md
echo.
echo 通信方案:
echo   - ACP通信方案.md
echo   - ACP传输层指南.md
echo   - 验证总结.md
echo.
echo 保持为英文的文档:
echo   - README.md
echo   - 架构.md
echo   - CODEBUDDY.md
echo   - 控制通道协议规范.md
echo.
echo.
pause
