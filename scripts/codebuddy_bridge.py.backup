#!/usr/bin/env python3
"""
CodeBuddy SDK 桥接脚本

该脚本作为 Rust 后端和 CodeBuddy CLI 之间的桥接层,使用官方 Python SDK。
"""

import asyncio
import json
import sys
import signal
import logging
from typing import Optional, Dict, Any, AsyncIterator
from dataclasses import dataclass

# 尝试导入 SDK
try:
    from codebuddy_agent_sdk import query, CodeBuddySDKClient, CodeBuddyAgentOptions
    print("✅ CodeBuddy SDK 导入成功", file=sys.stderr)
except ImportError as e:
    print(f"❌ 无法导入 CodeBuddy SDK: {e}", file=sys.stderr)
    sys.exit(1)

# 配置日志
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    stream=sys.stderr
)
logger = logging.getLogger(__name__)


class JSONEncoder(json.JSONEncoder):
    """自定义 JSON 编码器,处理特殊对象"""
    def default(self, obj):
        if hasattr(obj, '__dict__'):
            return obj.__dict__
        elif hasattr(obj, 'model_fields'):
            # 处理 pydantic 模型
            return obj.model_dump()
        return super().default(obj)


def send_message(msg_type: str, data: Dict[str, Any]):
    """发送消息到 stdout (JSON 格式)"""
    message = {
        "type": msg_type,
        "timestamp": __import__('datetime').datetime.now().isoformat(),
        "data": data
    }
    sys.stdout.write(json.dumps(message, cls=JSONEncoder) + '\n')
    sys.stdout.flush()


async def handle_query():
    """处理查询请求 (单次交互)"""
    try:
        # 从 stdin 读取配置
        config_line = await asyncio.get_event_loop().run_in_executor(None, sys.stdin.readline)
        if not config_line:
            return

        config = json.loads(config_line.strip())
        logger.info(f"收到配置: {config}")

        # 检查配置类型
        if config.get('command') == 'get_skills':
            # get_skills 暂时不支持,返回 MCP 服务器信息
            logger.info("处理 get_skills 请求")
            
            # 从 MCP 服务器获取 Skill 信息
            # 这里需要实现与 MCP 服务器的集成
            # 暂时返回空列表
            send_message('skill_list', {"skills": []})
            
        elif config.get('command') == 'execute_skill':
            logger.info(f"执行 Skill: {config.get('skill_name')}")
            
            # 执行查询
            async for message in query(prompt=config.get('prompt', '')):
                logger.info(f"处理消息: {message.__class__.__name__}")
                
                # 将消息转换为前端格式
                if hasattr(message, 'content'):
                    for block in message.content:
                        if hasattr(block, 'text'):
                            send_message('data_chunk', {
                                "session_id": config.get('session_id', 'default'),
                                "data": block.text,
                                "is_final": False,
                            })
                        elif hasattr(block, 'thinking'):
                            # 发送思考过程作为进度更新
                            send_message('progress', {
                                "session_id": config.get('session_id', 'default'),
                                "current": 0,
                                "total": 100,
                                "message": "思考中...",
                            })
                        elif hasattr(block, 'tool_use'):
                            # 发送工具使用事件
                            send_message('data_chunk', {
                                "session_id": config.get('session_id', 'default'),
                                "data": f"工具调用: {block.name}",
                                "is_final": False,
                            })
                
                # 检查是否是结果消息
                if hasattr(message, 'subtype'):
                    # 发送执行完成事件
                    send_message('execution_complete', {
                        "session_id": config.get('session_id', 'default'),
                        "success": not message.is_error,
                        "summary": message.result or "执行完成",
                    })
                    break
            
            logger.info("查询执行完成")
            
        else:
            logger.error(f"未知的命令: {config.get('command')}")
            send_message('error', {
                "code": "UNKNOWN_COMMAND",
                "message": f"未知的命令: {config.get('command')}",
                "suggestion": "请使用支持的命令: get_skills, execute_skill",
            })
            
    except Exception as e:
        logger.error(f"处理查询失败: {e}")
        send_message('error', {
            "code": "PROCESSING_ERROR",
            "message": str(e),
            "suggestion": "请检查输入格式和配置",
        })


async def handle_client():
    """处理多轮对话 (持久化连接)"""
    try:
        # 从 stdin 读取配置
        config_line = await asyncio.get_event_loop().run_in_executor(None, sys.stdin.readline)
        if not config_line:
            return

        config = json.loads(config_line.strip())
        logger.info(f"收到配置: {config}")

        # 创建客户端
        async with CodeBuddySDKClient() as client:
            logger.info("客户端已连接")

            # 发送初始消息
            send_message('session_started', {
                "session_id": config.get('session_id', 'default'),
                "agent_type": "codebuddy-sdk",
            })

            # 发送 prompt
            await client.query(prompt=config.get('prompt', ''))

            # 接收所有响应
            async for message in client.receive_messages():
                logger.info(f"收到消息: {message.__class__.__name__}")

                # 处理消息
                if hasattr(message, 'content'):
                    for block in message.content:
                        if hasattr(block, 'text'):
                            send_message('data_chunk', {
                                "session_id": config.get('session_id', 'default'),
                                "data": block.text,
                                "is_final": False,
                            })
                        elif hasattr(block, 'thinking'):
                            send_message('progress', {
                                "session_id": config.get('session_id', 'default'),
                                "current": 0,
                                "total": 100,
                                "message": f"思考: {block.thinking[:50]}...",
                            })
                        elif hasattr(block, 'tool_use'):
                            send_message('data_chunk', {
                                "session_id": config.get('session_id', 'default'),
                                "data": f"工具: {block.name}",
                                "is_final": False,
                            })
                        elif hasattr(block, 'tool_result'):
                            send_message('data_chunk', {
                                "session_id": config.get('session_id', 'default'),
                                "data": block.content or "",
                                "is_final": False,
                            })

                # 检查是否是结果消息
                if hasattr(message, 'subtype'):
                    send_message('execution_complete', {
                        "session_id": config.get('session_id', 'default'),
                        "success": not message.is_error,
                        "summary": message.result or "完成",
                    })
                    break

            logger.info("对话完成")
            
    except Exception as e:
        logger.error(f"处理客户端连接失败: {e}")
        send_message('error', {
            "code": "CLIENT_ERROR",
            "message": str(e),
            "suggestion": "请检查 CodeBuddy CLI 是否正常运行",
        })


async def main():
    """主循环"""
    logger.info("CodeBuddy SDK 桥接脚本已启动")

    try:
        while True:
            # 读取命令行
            line = await asyncio.get_event_loop().run_in_executor(None, sys.stdin.readline)
            
            if not line:
                logger.info("收到 EOF, 退出")
                break

            config = json.loads(line.strip())
            command = config.get('command', 'query')

            if command == 'query':
                await handle_query()
            elif command == 'client':
                await handle_client()
            elif command == 'exit':
                logger.info("收到退出命令")
                break
            else:
                logger.error(f"未知的命令: {command}")
                send_message('error', {
                    "code": "UNKNOWN_COMMAND",
                    "message": f"未知的命令: {command}",
                    "suggestion": "支持的命令: query, client, exit",
                })
                
    except KeyboardInterrupt:
        logger.info("收到中断信号,退出")
    except Exception as e:
        logger.error(f"主循环错误: {e}")
    finally:
        logger.info("脚本已退出")


def signal_handler(signum, frame):
    """信号处理器"""
    logger.info(f"收到信号: {signum}")
    sys.exit(0)


if __name__ == '__main__':
    # 注册信号处理器
    signal.signal(signal.SIGINT, signal_handler)
    signal.signal(signal.SIGTERM, signal_handler)

    # 运行主循环
    asyncio.run(main())
