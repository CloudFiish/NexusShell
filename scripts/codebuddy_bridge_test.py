#!/usr/bin/env python3
"""
CodeBuddy SDK 桥接脚本 - 测试版

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
    sys.stdout.write(json.dumps(message, cls=JSONEncoder, ensure_ascii=False) + '\n')
    sys.stdout.flush()


async def handle_query(config: Dict[str, Any]):
    """处理查询请求 (单次交互)"""
    try:
        logger.info(f"处理查询: {config.get('prompt', '')[:50]}...")

        # 执行查询
        async for message in query(prompt=config.get('prompt', '')):
            logger.info(f"处理消息: {message.__class__.__name__}")

            # 将消息转换为前端格式
            if hasattr(message, 'content'):
                for block in message.content:
                    if hasattr(block, 'text'):
                        # 直接发送 Assistant 消息
                        sys.stdout.write(json.dumps({
                            "type": "assistant",
                            "content": [{"text": block.text}],
                            "model": "",
                            "parent_tool_use_id": None,
                            "error": None
                        }, ensure_ascii=False) + '\n')
                        sys.stdout.flush()

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
                        sys.stdout.write(json.dumps({
                            "type": "assistant",
                            "content": [{
                                "tool_use": {
                                    "id": block.id,
                                    "name": block.name,
                                    "input": block.input
                                }
                            }],
                            "model": "",
                            "parent_tool_use_id": None,
                            "error": None
                        }, ensure_ascii=False) + '\n')
                        sys.stdout.flush()

                    elif hasattr(block, 'tool_result'):
                        sys.stdout.write(json.dumps({
                            "type": "assistant",
                            "content": [{
                                "tool_result": {
                                    "tool_use_id": block.tool_use_id,
                                    "content": block.content,
                                    "is_error": block.is_error
                                }
                            }],
                            "model": "",
                            "parent_tool_use_id": None,
                            "error": None
                        }, ensure_ascii=False) + '\n')
                        sys.stdout.flush()

            # 检查是否是结果消息
            if hasattr(message, 'subtype'):
                # 只有 ResultMessage 有 duration_ms
                if message.__class__.__name__ == 'ResultMessage':
                    # 发送执行完成事件
                    sys.stdout.write(json.dumps({
                        "type": "result",
                        "subtype": message.subtype,
                        "duration_ms": message.duration_ms,
                        "duration_api_ms": message.duration_api_ms,
                        "is_error": message.is_error,
                        "num_turns": message.num_turns,
                        "session_id": message.session_id,
                        "total_cost_usd": message.total_cost_usd,
                        "result": message.result,
                        "usage": message.usage,
                        "errors": message.errors
                    }, ensure_ascii=False) + '\n')
                    sys.stdout.flush()
                    break
                else:
                    # SystemMessage 或其他消息，继续等待
                    continue

        logger.info("查询执行完成")

    except Exception as e:
        logger.error(f"处理查询失败: {e}")
        send_message('error', {
            "code": "PROCESSING_ERROR",
            "message": str(e),
            "suggestion": "请检查输入格式和配置",
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

            logger.info(f"收到命令: {command}")

            if command == 'query':
                # 直接使用当前配置
                await handle_query(config)
            elif command == 'client':
                logger.info("client 模式暂不支持")
                send_message('error', {
                    "code": "NOT_IMPLEMENTED",
                    "message": "client 模式暂不支持",
                    "suggestion": "请使用 query 模式",
                })
            elif command == 'exit':
                logger.info("收到退出命令")
                break
            else:
                logger.error(f"未知的命令: {command}")
                send_message('error', {
                    "code": "UNKNOWN_COMMAND",
                    "message": f"未知的命令: {command}",
                    "suggestion": "支持的命令: query, exit",
                })

    except KeyboardInterrupt:
        logger.info("收到中断信号,退出")
    except Exception as e:
        logger.error(f"主循环错误: {e}")
        import traceback
        traceback.print_exc()
    finally:
        logger.info("脚本已退出")


def signal_handler(signum, frame):
    """信号处理器"""
    logger.info(f"收到信号: {signum}")
    sys.exit(0)


if __name__ == '__main__':
    # 注册信号处理器
    if sys.platform != 'win32':
        signal.signal(signal.SIGINT, signal_handler)
        signal.signal(signal.SIGTERM, signal_handler)

    # 运行主循环
    asyncio.run(main())
