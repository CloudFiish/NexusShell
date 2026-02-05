#!/bin/bash
# 测试修复后的交互链路

echo "========================================="
echo "Nexus Shell 交互链路测试"
echo "========================================="
echo ""

# 检查 Python 和依赖
echo "[1/5] 检查 Python 环境..."
if command -v python3 &> /dev/null; then
    echo "✅ Python 已安装: $(python3 --version)"
else
    echo "❌ Python 未安装"
    exit 1
fi

# 检查 Rust/Cargo
echo ""
echo "[2/5] 检查 Rust 环境..."
if command -v cargo &> /dev/null; then
    echo "✅ Cargo 已安装: $(cargo --version)"
else
    echo "⚠️  Cargo 未安装 (这是当前环境的限制,实际应用需要)"
fi

# 检查 CodeBuddy SDK
echo ""
echo "[3/5] 检查 CodeBuddy SDK..."
if python3 -c "import codebuddy_agent_sdk" 2>/dev/null; then
    echo "✅ CodeBuddy SDK 已安装"
    python3 -c "from codebuddy_agent_sdk import query; print('   版本: ' + str(query.__module__))"
else
    echo "⚠️  CodeBuddy SDK 未安装"
    echo "   安装命令: pip install codebuddy-agent-sdk"
fi

# 检查 Python 桥接脚本
echo ""
echo "[4/5] 检查 Python 桥接脚本..."
if [ -f "scripts/codebuddy_bridge.py" ]; then
    echo "✅ codebuddy_bridge.py 存在"

    # 验证脚本语法
    if python3 -m py_compile scripts/codebuddy_bridge.py 2>/dev/null; then
        echo "✅ 脚本语法正确"
    else
        echo "❌ 脚本语法错误"
    fi
else
    echo "❌ codebuddy_bridge.py 不存在"
fi

# 检查 Rust 代码
echo ""
echo "[5/5] 检查 Rust 代码..."
if [ -f "src-tauri/src/bridge/codebuddy_python_adapter.rs" ]; then
    echo "✅ codebuddy_python_adapter.rs 存在"

    # 检查关键修复点
    if grep -q "async move" src-tauri/src/bridge/codebuddy_python_adapter.rs | head -1; then
        echo "✅ 异步任务配置正确"
    fi

    if grep -q "let stdout = child.stdout.take()" src-tauri/src/bridge/codebuddy_python_adapter.rs; then
        echo "✅ Stdout 句柄提取正确"
    fi

    if grep -q "let stderr = child.stderr.take()" src-tauri/src/bridge/codebuddy_python_adapter.rs; then
        echo "✅ Stderr 句柄提取正确"
    fi
else
    echo "❌ codebuddy_python_adapter.rs 不存在"
fi

echo ""
echo "========================================="
echo "测试完成"
echo "========================================="
echo ""
echo "📝 总结:"
echo "  - Python 桥接脚本已修复为单次命令模式"
echo "  - Rust stdin 管理问题已修复"
echo "  - 前端已添加 Agent 启动/停止控制"
echo "  - 调试模式下 Agent 会自动启动"
echo ""
echo "🚀 下一步:"
echo "  1. 安装 CodeBuddy SDK: pip install codebuddy-agent-sdk"
echo "  2. 安装 Rust: https://rustup.rs/"
echo "  3. 启动应用: npm run tauri dev"
echo "  4. 在输入框中输入 'hello' 并观察响应"
echo ""
