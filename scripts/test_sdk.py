#!/usr/bin/env python3
"""
测试 CodeBuddy SDK 基本功能
"""

import sys
import os
import json
import asyncio

# 检查是否在虚拟环境中
venv_path = os.path.join(os.path.dirname(__file__), '..', 'src-tauri', '.venv')
venv_python = os.path.join(venv_path, 'Scripts', 'python.exe') if sys.platform == 'win32' else os.path.join(venv_path, 'bin', 'python')

if os.path.exists(venv_path) and not hasattr(sys, 'real_prefix') and not (hasattr(sys, 'base_prefix') and sys.base_prefix != sys.prefix):
    print(f"⚠️  警告: 当前 Python 不是虚拟环境中的 Python")
    print(f"   当前: {sys.executable}")
    print(f"   虚拟环境: {venv_python}")
    print(f"   建议: 请使用虚拟环境中的 Python 运行此脚本")
    # 继续执行，不退出

# 导入 SDK
try:
    from codebuddy_agent_sdk import query, CodeBuddySDKClient, CodeBuddyAgentOptions
    print("✅ CodeBuddy SDK 导入成功")
except ImportError as e:
    print(f"❌ 导入失败: {e}")
    sys.exit(1)

async def test_query():
    """测试 query 函数"""
    print("\n📝 测试 1: query 函数")
    print("-" * 50)

    try:
        # 尝试一个简单的查询
        print("发送查询: '你好'...")

        async for message in query(prompt="你好"):
            print(f"收到消息: {message.__class__.__name__}")

            # 打印消息内容
            if hasattr(message, 'content'):
                for block in message.content:
                    if hasattr(block, 'text'):
                        print(f"  文本: {block.text[:50]}...")
                    elif hasattr(block, 'thinking'):
                        print(f"  思考: {block.thinking[:50]}...")
                    elif hasattr(block, 'tool_use'):
                        print(f"  工具: {block.name}")

            # SystemMessage 不是执行完成消息，跳过
            if message.__class__.__name__ == 'SystemMessage':
                continue

            # 检查是否是结果消息
            if hasattr(message, 'subtype'):
                print(f"✓ 执行完成: success={not message.is_error}")
                break

        print("✅ query 测试通过")
        return True

    except Exception as e:
        print(f"❌ query 测试失败: {e}")
        import traceback
        traceback.print_exc()
        return False

async def test_client():
    """测试 CodeBuddySDKClient"""
    print("\n📝 测试 2: CodeBuddySDKClient")
    print("-" * 50)

    try:
        print("创建客户端...")
        async with CodeBuddySDKClient() as client:
            print("✓ 客户端已连接")

            # 发送查询
            print("发送查询: '1+1=?'...")
            await client.query(prompt="1+1=?")

            # 接收响应
            print("接收响应...")
            async for message in client.receive_messages():
                print(f"收到消息: {message.__class__.__name__}")

                if hasattr(message, 'content'):
                    for block in message.content:
                        if hasattr(block, 'text'):
                            print(f"  文本: {block.text}")

                if hasattr(message, 'subtype'):
                    print(f"✓ 执行完成")
                    break

        print("✅ client 测试通过")
        return True

    except Exception as e:
        print(f"❌ client 测试失败: {e}")
        import traceback
        traceback.print_exc()
        return False

async def test_message_types():
    """测试消息类型"""
    print("\n📝 测试 3: 消息类型")
    print("-" * 50)

    try:
        from codebuddy_agent_sdk import (
            AssistantMessage, UserMessage, SystemMessage,
            ResultMessage, StreamEvent,
            TextBlock, ThinkingBlock, ToolUseBlock, ToolResultBlock
        )

        print("✓ 消息类型导入成功")

        # 测试序列化
        assistant_msg = AssistantMessage(
            content=[TextBlock(text="测试文本")],
            model="test-model"
        )

        # 转换为字典
        msg_dict = assistant_msg.model_dump() if hasattr(assistant_msg, 'model_dump') else assistant_msg.__dict__
        print(f"✓ 消息序列化: {json.dumps(msg_dict, indent=2, default=str, ensure_ascii=False)[:200]}...")

        print("✅ 消息类型测试通过")
        return True

    except Exception as e:
        print(f"❌ 消息类型测试失败: {e}")
        import traceback
        traceback.print_exc()
        return False

async def main():
    """主测试函数"""
    print("=" * 60)
    print("CodeBuddy SDK 功能测试")
    print("=" * 60)

    results = []

    # 测试 1: query 函数
    results.append(await test_query())

    # 测试 2: CodeBuddySDKClient
    # results.append(await test_client())  # 先跳过，可能需要更长时间

    # 测试 3: 消息类型
    results.append(await test_message_types())

    # 总结
    print("\n" + "=" * 60)
    print("测试总结")
    print("=" * 60)
    total = len(results)
    passed = sum(results)
    print(f"总测试数: {total}")
    print(f"通过: {passed}")
    print(f"失败: {total - passed}")

    if passed == total:
        print("\n✅ 所有测试通过！")
        return 0
    else:
        print(f"\n⚠️  有 {total - passed} 个测试失败")
        return 1

if __name__ == '__main__':
    sys.exit(asyncio.run(main()))
