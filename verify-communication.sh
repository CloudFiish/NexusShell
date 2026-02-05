#!/bin/bash
# NexusShell 与 CodeBuddy 通信快速验证脚本
# 使用方法: ./verify-communication.sh

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 项目根目录
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$PROJECT_ROOT"

# Python 虚拟环境路径
VENV_PYTHON="$PROJECT_ROOT/src-tauri/.venv/Scripts/python.exe"
if [ ! -f "$VENV_PYTHON" ]; then
    VENV_PYTHON="$PROJECT_ROOT/src-tauri/.venv/bin/python"
fi

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}NexusShell 通信验证脚本${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# 检查 1: Python 环境
echo -e "${YELLOW}[1/5] 检查 Python 环境...${NC}"
if [ -f "$VENV_PYTHON" ]; then
    PYTHON_VERSION=$("$VENV_PYTHON" --version 2>&1)
    echo -e "${GREEN}✓${NC} Python 版本: $PYTHON_VERSION"
else
    echo -e "${RED}✗${NC} Python 虚拟环境未找到"
    exit 1
fi

# 检查 2: Python SDK
echo -e "${YELLOW}[2/5] 检查 Python SDK...${NC}"
if "$VENV_PYTHON" -c "import codebuddy_agent_sdk; print(codebuddy_agent_sdk.__version__)" 2>/dev/null; then
    SDK_VERSION=$("$VENV_PYTHON" -c "import codebuddy_agent_sdk; print(codebuddy_agent_sdk.__version__)" 2>&1)
    echo -e "${GREEN}✓${NC} CodeBuddy SDK 版本: $SDK_VERSION"
else
    echo -e "${RED}✗${NC} CodeBuddy SDK 未安装"
    exit 1
fi

# 检查 3: 桥接脚本
echo -e "${YELLOW}[3/5] 检查桥接脚本...${NC}"
if [ -f "$PROJECT_ROOT/scripts/codebuddy_bridge.py" ]; then
    # 使用绝对路径
    ABSOLUTE_PATH=$(cd "$PROJECT_ROOT/scripts" && pwd)/codebuddy_bridge.py
    if "$VENV_PYTHON" -m py_compile "$ABSOLUTE_PATH" 2>/dev/null; then
        echo -e "${GREEN}✓${NC} 桥接脚本语法正确"
    else
        echo -e "${RED}✗${NC} 桥接脚本有语法错误"
        exit 1
    fi
else
    echo -e "${RED}✗${NC} 桥接脚本未找到: $PROJECT_ROOT/scripts/codebuddy_bridge.py"
    exit 1
fi

# 检查 4: 基础通信
echo -e "${YELLOW}[4/5] 测试基础通信...${NC}"
ABSOLUTE_PATH=$(cd "$PROJECT_ROOT/scripts" && pwd)/codebuddy_bridge.py
TEST_OUTPUT=$(echo '{"command":"exit"}' | timeout 10 "$VENV_PYTHON" "$ABSOLUTE_PATH" 2>&1 || true)
if echo "$TEST_OUTPUT" | grep -q "收到退出命令"; then
    echo -e "${GREEN}✓${NC} 基础通信正常"
else
    echo -e "${RED}✗${NC} 基础通信失败"
    echo "$TEST_OUTPUT" | head -5
    exit 1
fi

# 检查 5: 查询功能
echo -e "${YELLOW}[5/5] 测试查询功能...${NC}"
echo "这可能需要 30-60 秒，请稍候..."

TEST_OUTPUT=$(echo '{"command":"query","session_id":"verify_test","prompt":"1+1=?"}' | timeout 120 "$VENV_PYTHON" "$ABSOLUTE_PATH" 2>&1 || true)

if echo "$TEST_OUTPUT" | grep -q '"type": "result"' && echo "$TEST_OUTPUT" | grep -q '"subtype": "success"'; then
    echo -e "${GREEN}✓${NC} 查询功能正常"
else
    echo -e "${RED}✗${NC} 查询功能失败"
    echo "$TEST_OUTPUT" | tail -10
    exit 1
fi

echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${GREEN}✓ 所有检查通过！${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo -e "${GREEN}NexusShell 与 CodeBuddy 之间的通信链路正常工作。${NC}"
echo ""
echo -e "下一步:"
echo -e "  1. 启动应用: ${YELLOW}npm run tauri dev${NC}"
echo -e "  2. 在 UI 中测试查询功能"
echo -e "  3. 查看详细报告: ${YELLOW}docs/testing/通信检查总结.md${NC}"
echo ""

exit 0
