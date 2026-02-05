#!/usr/bin/env python3
"""
测试前端与 CodeBuddy SDK 的通信
"""

import subprocess
import json
import sys
import os

# 添加 scripts 目录到路径
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'scripts'))

def test_bridge_script():
    """测试 bridge 脚本是否能正确响应"""
    print("=" * 60)
    print("测试 CodeBuddy Bridge 脚本")
    print("=" * 60)

    # 准备测试输入
    test_config = {
        "command": "query",
        "session_id": "test-session-001",
        "prompt": "Hello, this is a test message",
        "options": {
            "continue_conversation": False,
            "setting_sources": ["project", "local"]
        }
    }

    # 启动 Python 脚本
    try:
        process = subprocess.Popen(
            [sys.executable, '-c', open('scripts/codebuddy_bridge.py').read()],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )

        # 发送配置
        process.stdin.write(json.dumps(test_config) + '\n')
        process.stdin.flush()
        process.stdin.close()

        # 读取输出
        print("\n📤 发送到 Bridge 的数据:")
        print(json.dumps(test_config, indent=2))

        print("\n📥 从 Bridge 接收到的数据:")
        stdout_lines = []
        stderr_lines = []

        # 读取 stdout
        for line in process.stdout:
            line = line.strip()
            if line:
                stdout_lines.append(line)
                try:
                    parsed = json.loads(line)
                    print(f"  ✅ 有效 JSON: {json.dumps(parsed, indent=2)[:200]}...")
                except json.JSONDecodeError as e:
                    print(f"  ❌ 无效 JSON: {line[:100]}...")
                    print(f"     错误: {e}")

        # 读取 stderr
        for line in process.stderr:
            line = line.strip()
            if line:
                stderr_lines.append(line)
                print(f"  📝 日志: {line}")

        # 等待进程结束
        process.wait()

        print(f"\n📊 统计:")
        print(f"  - 收到 {len(stdout_lines)} 行 stdout")
        print(f"  - 收到 {len(stderr_lines)} 行 stderr")
        print(f"  - 进程返回码: {process.returncode}")

        return len(stdout_lines) > 0

    except Exception as e:
        print(f"❌ 测试失败: {e}")
        import traceback
        traceback.print_exc()
        return False


def test_sdk_import():
    """测试 SDK 是否能正确导入"""
    print("\n" + "=" * 60)
    print("测试 CodeBuddy SDK 导入")
    print("=" * 60)

    try:
        from codebuddy_agent_sdk import query, CodeBuddySDKClient, CodeBuddyAgentOptions
        print("✅ CodeBuddy SDK 导入成功")
        return True
    except ImportError as e:
        print(f"❌ 无法导入 CodeBuddy SDK: {e}")
        print("\n💡 请确保已安装 SDK:")
        print("   pip install codebuddy-agent-sdk")
        return False


def main():
    print("🧪 CodeBuddy 通信测试工具")
    print("=" * 60)

    # 测试 SDK 导入
    sdk_ok = test_sdk_import()

    if not sdk_ok:
        print("\n⚠️ SDK 未安装，跳过 Bridge 测试")
        return

    # 测试 Bridge 脚本
    bridge_ok = test_bridge_script()

    # 总结
    print("\n" + "=" * 60)
    print("测试结果总结")
    print("=" * 60)
    print(f"  SDK 导入: {'✅ 通过' if sdk_ok else '❌ 失败'}")
    print(f"  Bridge 脚本: {'✅ 通过' if bridge_ok else '❌ 失败'}")

    if sdk_ok and bridge_ok:
        print("\n🎉 所有测试通过！通信应该正常。")
    else:
        print("\n⚠️ 部分测试失败，请检查配置。")


if __name__ == '__main__':
    main()
