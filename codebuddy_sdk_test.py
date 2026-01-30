#!/usr/bin/env python3
"""
CodeBuddy SDK 集成测试脚本

用于测试 Python SDK 的基本功能,验证通信协议和消息格式。
"""

import asyncio
import json
import sys
from datetime import datetime

# 尝试导入 SDK
try:
    from codebuddy_agent_sdk import query, CodeBuddySDKClient, CodeBuddyAgentOptions
    print("✅ CodeBuddy SDK 导入成功")
except ImportError as e:
    print(f"❌ 无法导入 CodeBuddy SDK: {e}")
    print("请运行: pip install codebuddy-agent-sdk")
    sys.exit(1)


class JSONEncoder(json.JSONEncoder):
    """自定义 JSON 编码器,处理特殊对象"""
    def default(self, obj):
        if hasattr(obj, '__dict__'):
            return obj.__dict__
        elif hasattr(obj, 'model_fields'):
            # 处理 pydantic 模型
            return obj.model_dump()
        return super().default(obj)


def print_message(msg):
    """打印消息的详细信息"""
    print(f"\n{'='*60}")
    print(f"类型: {msg.__class__.__name__}")
    print(f"时间: {datetime.now().isoformat()}")
    
    # 打印所有字段
    for key, value in msg.__dict__.items():
        if value is not None:
            if key == 'content' and isinstance(value, list):
                print(f"{key}: [{len(value)} 个 ContentBlock]")
                for i, block in enumerate(value):
                    print(f"  [{i}] {block.__class__.__name__}")
                    for bkey, bvalue in block.__dict__.items():
                        if bvalue is not None:
                            if isinstance(bvalue, str):
                                print(f"      {bkey}: {bvalue[:100]}...")
                            else:
                                print(f"      {bkey}: {bvalue}")
            else:
                print(f"{key}: {value}")
    print(f"{'='*60}")


async def test_simple_query():
    """测试 1: 基本查询 (fire-and-forget)"""
    print("\n" + "="*60)
    print("测试 1: 基本查询")
    print("="*60)
    
    try:
        async for message in query(prompt="2 + 2 等于几?"):
            print_message(message)
    except Exception as e:
        print(f"❌ 测试失败: {e}")


async def test_with_client():
    """测试 2: 使用 Client 类 (多轮对话)"""
    print("\n" + "="*60)
    print("测试 2: 使用 Client 类 (多轮对话)")
    print("="*60)
    
    try:
        async with CodeBuddySDKClient() as client:
            # 第一轮对话
            print("\n发送: 你好,我的名字是 Alice")
            await client.query("你好,我的名字是 Alice")
            
            print("\n接收消息:")
            async for msg in client.receive_messages():
                print_message(msg)
                
            # 第二轮对话
            print("\n发送: 我的名字是什么?")
            await client.query("我的名字是什么?")
            
            print("\n接收消息:")
            async for msg in client.receive_messages():
                print_message(msg)
                
    except Exception as e:
        print(f"❌ 测试失败: {e}")


async def test_session_resume():
    """测试 3: 会话恢复"""
    print("\n" + "="*60)
    print("测试 3: 会话恢复")
    print("="*60)
    
    session_id = None
    
    try:
        # 创建第一个会话
        print("\n会话 1: 设置名字")
        async with CodeBuddySDKClient() as client:
            await client.query("我的名字是 Bob")
            
            # 获取 session_id
            async for msg in client.receive_messages():
                print_message(msg)
                if hasattr(msg, 'session_id') and msg.session_id:
                    session_id = msg.session_id
                    break
        
        if session_id:
            print(f"\n获取到的 Session ID: {session_id}")
            
            # 恢复会话
            print("\n会话 2: 恢复会话,询问名字")
            async with CodeBuddySDKClient(
                options=CodeBuddyAgentOptions(resume=session_id)
            ) as client:
                await client.query("我的名字是什么?")
                
                async for msg in client.receive_messages():
                    print_message(msg)
                    
    except Exception as e:
        print(f"❌ 测试失败: {e}")


async def test_options():
    """测试 4: 使用配置选项"""
    print("\n" + "="*60)
    print("测试 4: 使用配置选项")
    print("="*60)
    
    try:
        # 使用不同的配置
        options = CodeBuddyAgentOptions(
            setting_sources=["project"],  # 只加载项目配置
            model="gpt-4",  # 指定模型 (如果支持)
        )
        
        print(f"\n配置: {json.dumps(options.__dict__, indent=2, cls=JSONEncoder)}")
        
        async with CodeBuddySDKClient(options=options) as client:
            await client.query("使用什么配置?")
            
            async for msg in client.receive_messages():
                print_message(msg)
                # 收到第一个响应后停止
                break
                
    except Exception as e:
        print(f"❌ 测试失败: {e}")


async def test_content_blocks():
    """测试 5: 详细处理 ContentBlock"""
    print("\n" + "="*60)
    print("测试 5: 详细处理 ContentBlock")
    print("="*60)
    
    try:
        async for message in query(prompt="写一段 Python 代码"):
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"\n[TextBlock]")
                        print(f"内容: {block.text}")
                    elif isinstance(block, ThinkingBlock):
                        print(f"\n[ThinkingBlock]")
                        print(f"思考: {block.thinking[:100]}...")
                        print(f"签名: {block.signature[:50]}...")
                    elif isinstance(block, ToolUseBlock):
                        print(f"\n[ToolUseBlock]")
                        print(f"工具 ID: {block.id}")
                        print(f"工具名: {block.name}")
                        print(f"输入: {json.dumps(block.input, indent=2)[:200]}...")
                    elif isinstance(block, ToolResultBlock):
                        print(f"\n[ToolResultBlock]")
                        print(f"工具 ID: {block.tool_use_id}")
                        print(f"结果: {block.content[:100] if block.content else 'None'}...")
                        print(f"错误: {block.is_error}")
                    
    except Exception as e:
        print(f"❌ 测试失败: {e}")


async def test_stream_events():
    """测试 6: 流式事件"""
    print("\n" + "="*60)
    print("测试 6: 流式事件")
    print("="*60)
    
    try:
        # 使用复杂 prompt,触发多个事件
        async for message in query(prompt="帮我写一个简单的计算器程序,包括加法和减法功能"):
            print_message(message)
            
    except Exception as e:
        print(f"❌ 测试失败: {e}")


async def main():
    """主测试函数"""
    print("\n" + "="*60)
    print("CodeBuddy SDK 集成测试")
    print("="*60)
    print(f"开始时间: {datetime.now().isoformat()}")
    
    # 运行所有测试
    await test_simple_query()
    await test_with_client()
    await test_session_resume()
    await test_options()
    await test_content_blocks()
    await test_stream_events()
    
    print("\n" + "="*60)
    print("所有测试完成")
    print(f"结束时间: {datetime.now().isoformat()}")
    print("="*60)


if __name__ == "__main__":
    # 设置日志
    import logging
    logging.basicConfig(
        level=logging.INFO,
        format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
    )
    
    # 运行测试
    asyncio.run(main())
