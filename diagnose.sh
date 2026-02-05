#!/bin/bash
# 快速诊断脚本 - 检查所有关键组件

echo "========================================="
echo "NexusShell 诊断工具"
echo "========================================="
echo ""

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 检查 1: 应用是否在运行
echo -e "${BLUE}[1/8]${NC} 检查应用运行状态..."
if pgrep -f "tauri dev" > /dev/null 2>&1; then
    echo -e "${GREEN}✓${NC} Tauri 开发服务器正在运行"
else
    echo -e "${RED}✗${NC} Tauri 开发服务器未运行"
    echo -e "${YELLOW}提示: 运行 'npm run tauri dev' 启动应用${NC}"
fi
echo ""

# 检查 2: Node.js
echo -e "${BLUE}[2/8]${NC} 检查 Node.js..."
if command -v node &> /dev/null; then
    NODE_VERSION=$(node --version)
    echo -e "${GREEN}✓${NC} Node.js 版本: $NODE_VERSION"
else
    echo -e "${RED}✗${NC} Node.js 未安装"
fi
echo ""

# 检查 3: Python 环境
echo -e "${BLUE}[3/8]${NC} 检查 Python 环境..."
if [ -f "src-tauri/.venv/Scripts/python.exe" ]; then
    PYTHON_VERSION=$(./src-tauri/.venv/Scripts/python.exe --version 2>&1)
    echo -e "${GREEN}✓${NC} Python 版本: $PYTHON_VERSION"
else
    echo -e "${RED}✗${NC} Python 虚拟环境未找到"
fi
echo ""

# 检查 4: Python SDK
echo -e "${BLUE}[4/8]${NC} 检查 CodeBuddy SDK..."
if [ -f "src-tauri/.venv/Scripts/python.exe" ]; then
    if ./src-tauri/.venv/Scripts/python.exe -c "import codebuddy_agent_sdk; print('OK')" 2>/dev/null; then
        SDK_VERSION=$(./src-tauri/.venv/Scripts/python.exe -c "import codebuddy_agent_sdk; print(codebuddy_agent_sdk.__version__)" 2>&1)
        echo -e "${GREEN}✓${NC} CodeBuddy SDK 版本: $SDK_VERSION"
    else
        echo -e "${RED}✗${NC} CodeBuddy SDK 未安装或导入失败"
    fi
else
    echo -e "${RED}✗${NC} 无法检查 SDK（Python 未找到）"
fi
echo ""

# 检查 5: Python 桥接脚本
echo -e "${BLUE}[5/8]${NC} 检查桥接脚本..."
if [ -f "scripts/codebuddy_bridge.py" ]; then
    if ./src-tauri/.venv/Scripts/python.exe -m py_compile scripts/codebuddy_bridge.py 2>/dev/null; then
        echo -e "${GREEN}✓${NC} 桥接脚本语法正确"
    else
        echo -e "${RED}✗${NC} 桥接脚本有语法错误"
    fi
else
    echo -e "${RED}✗${NC} 桥接脚本未找到"
fi
echo ""

# 检查 6: 测试基础通信
echo -e "${BLUE}[6/8]${NC} 测试基础通信..."
TEST_OUTPUT=$(echo '{"command":"exit"}' | timeout 10 ./src-tauri/.venv/Scripts/python.exe scripts/codebuddy_bridge.py 2>&1 || true)
if echo "$TEST_OUTPUT" | grep -q "收到退出命令"; then
    echo -e "${GREEN}✓${NC} 基础通信正常"
else
    echo -e "${RED}✗${NC} 基础通信失败"
fi
echo ""

# 检查 7: 检查日志文件
echo -e "${BLUE}[7/8]${NC} 检查日志文件..."
if [ -f "scripts/bridge_debug.log" ]; then
    LOG_SIZE=$(wc -l < scripts/bridge_debug.log)
    LOG_TIME=$(tail -1 scripts/bridge_debug.log | grep -oP '\[\K[0-9]{2}:[0-9]{2}:[0-9]{2}' || echo "未知")
    echo -e "${GREEN}✓${NC} 日志文件存在 ($LOG_SIZE 行)"
    echo "  最后更新时间: $LOG_TIME"
else
    echo -e "${YELLOW}⚠${NC} 日志文件不存在（尚未运行过）"
fi
echo ""

# 检查 8: 前端依赖
echo -e "${BLUE}[8/8]${NC} 检查前端依赖..."
if [ -f "package.json" ] && [ -d "node_modules" ]; then
    echo -e "${GREEN}✓${NC} 前端依赖已安装"
else
    echo -e "${RED}✗${NC} 前端依赖未安装"
    echo -e "${YELLOW}提示: 运行 'npm install' 安装依赖${NC}"
fi
echo ""

# 总结
echo "========================================="
echo -e "${BLUE}诊断总结${NC}"
echo "========================================="
echo ""
echo "下一步操作："
echo "  1. 如果有任何 ${RED}失败项${NC}，请先修复"
echo "  2. 运行应用: ${YELLOW}npm run tauri dev${NC}"
echo "  3. 打开浏览器控制台 (F12) 查看日志"
echo "  4. 在输入框中输入测试指令"
echo "  5. 查看调试指南: ${YELLOW}docs/debugging/前端响应调试指南.md${NC}"
echo ""
