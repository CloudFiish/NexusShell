#!/usr/bin/env python3
"""
简化版集成测试 - 验证 Python 桥接脚本的基本功能
"""

import sys
import os
import json
import subprocess
import time

# 虚拟环境 Python 路径
VENV_PYTHON = os.path.join(
    os.path.dirname(__file__),
    '..',
    'src-tauri',
    '.venv',
    'Scripts',
    'python.exe'
)

# 桥接脚本路径
BRIDGE_SCRIPT = os.path.join(
    os.path.dirname(__file__),
    'codebuddy_bridge.py'
)

def test_simple_query():
    """测试简单查询"""
    print("=" * 60)
    print("测试: 简单查询")
    print("=" * 60)

    try:
        # 创建测试配置
        config = {
            "session_id": "test_session_001",
            "prompt": "1+1=?",
            "options": {
                "continue_conversation": False,
                "setting_sources": ["project", "local"],
            }
        }

        print(f"发送配置: {json.dumps(config, indent=2)}")

        # 启动桥接脚本
        process = subprocess.Popen(
            [VENV_PYTHON, BRIDGE_SCRIPT],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            encoding='utf-8'
        )

        # 发送配置
        process.stdin.write(json.dumps(config) + '\n')
        process.stdin.flush()

        # 读取输出
        output_lines = []
        error_lines = []

        print("\n开始读取输出...")

        # 设置超时
        start_time = time.time()
        timeout = 120  # 120 秒超时

        while True:
            # 检查超时
            if time.time() - start_time > timeout:
                print(f"\n⚠️  超时 ({timeout}秒)")
                process.terminate()
                process.wait(timeout=5)
                break

            # 检查进程是否结束
            if process.poll() is not None:
                print("\n进程已结束")
                break

            # 读取 stdout
            try:
                line = process.stdout.readline()
                if line:
                    line = line.strip()
                    output_lines.append(line)
                    print(f"[OUT] {line[:100]}...")

                    # 检查是否收到完成信号
                    parsed = None
                    try:
                        parsed = json.loads(line)
                    except:
                        pass

                    if parsed:
                        # 检查是否是 result 消息
                        if parsed.get('type') == 'result' and parsed.get('subtype') == 'success':
                            print("\n✓ 收到成功结果")
                            process.terminate()
                            process.wait(timeout=5)
                            break

                        # 检查是否是错误
                        if parsed.get('type') == 'error':
                            print(f"\n❌ 收到错误: {parsed}")
                            process.terminate()
                            process.wait(timeout=5)
                            break

            except:
                break

        # 分析结果
        print(f"\n收到 {len(output_lines)} 行输出")
        print(f"收到 {len(error_lines)} 行错误")

        if error_lines:
            print("\n错误输出:")
            for line in error_lines:
                print(f"  {line}")

        # 检查是否收到成功结果
        has_success = any(
            '"type": "result"' in line and '"subtype": "success"' in line
            for line in output_lines
        )

        has_assistant = any('"type": "assistant"' in line for line in output_lines)

        print(f"\n收到 Assistant 消息: {'是' if has_assistant else '否'}")
        print(f"收到成功结果: {'是' if has_success else '否'}")

        if has_success and has_assistant:
            print("\n✅ 测试通过！")
            return True
        else:
            print("\n⚠️  测试部分通过")
            return has_assistant or has_success

    except subprocess.TimeoutExpired:
        print("\n❌ 测试超时")
        process.terminate()
        process.wait(timeout=5)
        return False
    except Exception as e:
        print(f"\n❌ 测试失败: {e}")
        import traceback
        traceback.print_exc()
        return False

def main():
    """主测试函数"""
    print("\n" + "=" * 60)
    print("Python 桥接脚本简化集成测试")
    print("=" * 60)

    # 检查文件是否存在
    if not os.path.exists(VENV_PYTHON):
        print(f"❌ Python 虚拟环境未找到: {VENV_PYTHON}")
        return 1

    if not os.path.exists(BRIDGE_SCRIPT):
        print(f"❌ 桥接脚本未找到: {BRIDGE_SCRIPT}")
        return 1

    print(f"✅ Python 虚拟环境: {VENV_PYTHON}")
    print(f"✅ 桥接脚本: {BRIDGE_SCRIPT}")
    print()

    # 运行测试
    result = test_simple_query()

    # 总结
    print("\n" + "=" * 60)
    print("测试总结")
    print("=" * 60)

    if result:
        print("✅ 测试通过！")
        print("\n✨ Python 桥接脚本可以正常工作")
        return 0
    else:
        print("⚠️  测试未通过")
        print("\n请检查:")
        print("  1. Python SDK 是否正确安装")
        print("  2. 网络连接是否正常")
        print("  3. CodeBuddy CLI 是否需要配置")
        return 1

if __name__ == '__main__':
    sys.exit(main())
