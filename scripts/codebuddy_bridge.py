#!/usr/bin/env python3
"""
CodeBuddy SDK 桥接脚本 - 修复版

该脚本作为 Rust 后端和 CodeBuddy CLI 之间的桥接层,使用官方 Python SDK。
输出格式与 Rust SDKMessage 协议匹配。
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


def send_sdk_message(msg: Dict[str, Any]):
    """发送 SDK 消息到 stdout (JSON 格式)"""
    sys.stdout.write(json.dumps(msg, cls=JSONEncoder, ensure_ascii=False) + '\n')
    sys.stdout.flush()


def send_log_message(msg_type: str, data: Dict[str, Any]):
    """发送日志消息到 stderr"""
    message = {
        "type": msg_type,
        "timestamp": __import__('datetime').datetime.now().isoformat(),
        "data": data
    }
    logger.info(f"[{msg_type}] {data}")


async def handle_get_skills():
    """获取可用的 Skill 列表"""
    try:
        logger.info("获取 Skill 列表...")
        
        skills = []
        
        # 尝试从 SDK 获取技能列表
        try:
            # 尝试导入技能管理模块
            from codebuddy_agent_sdk import SkillManager, get_available_skills
            logger.info("尝试使用 SDK 获取技能列表...")
            
            # 尝试获取内置技能
            sdk_skills = get_available_skills()
            if sdk_skills:
                for skill in sdk_skills:
                    skills.append({
                        "name": skill.get("name", "unknown"),
                        "description": skill.get("description", ""),
                        "default_render": skill.get("default_render", "text"),
                        "supported_renders": skill.get("supported_renders", ["text"]),
                        "input_schema": skill.get("input_schema"),
                        "output_schema": skill.get("output_schema"),
                        "category": skill.get("category", "sdk"),
                        "requires_filesystem": skill.get("requires_filesystem", False),
                        "requires_network": skill.get("requires_network", True)
                    })
                logger.info(f"从 SDK 获取到 {len(skills)} 个技能")
        except ImportError as e:
            logger.warning(f"SDK 技能管理模块不可用: {e}")
        except Exception as e:
            logger.warning(f"从 SDK 获取技能失败: {e}")
        
        # 如果没有从 SDK 获取到技能，添加默认的 assistant skill
        if not skills:
            logger.info("使用默认技能列表...")
            skills = [
                {
                    "name": "assistant",
                    "description": "通用 AI 助手，可以回答问题和执行各种任务",
                    "default_render": "text",
                    "supported_renders": ["text", "json"],
                    "input_schema": None,
                    "output_schema": None,
                    "category": "core",
                    "requires_filesystem": False,
                    "requires_network": True
                }
            ]
        
        # 尝试从用户配置目录加载自定义技能
        try:
            import os
            import glob
            
            # 获取用户配置目录
            config_dir = os.path.expanduser("~/.codebuddy/skills")
            if os.path.exists(config_dir):
                logger.info(f"扫描用户技能目录: {config_dir}")
                
                # 查找所有技能配置文件
                skill_files = glob.glob(os.path.join(config_dir, "*.json"))
                for skill_file in skill_files:
                    try:
                        with open(skill_file, 'r', encoding='utf-8') as f:
                            skill_config = json.load(f)
                            
                        skill_name = os.path.basename(skill_file)[:-5]  # 去掉 .json
                        skills.append({
                            "name": skill_name,
                            "description": skill_config.get("description", f"用户自定义技能: {skill_name}"),
                            "default_render": skill_config.get("default_render", "text"),
                            "supported_renders": skill_config.get("supported_renders", ["text"]),
                            "input_schema": skill_config.get("input_schema"),
                            "output_schema": skill_config.get("output_schema"),
                            "category": skill_config.get("category", "user"),
                            "requires_filesystem": skill_config.get("requires_filesystem", False),
                            "requires_network": skill_config.get("requires_network", True)
                        })
                        logger.info(f"加载用户技能: {skill_name}")
                    except Exception as e:
                        logger.warning(f"加载技能文件失败 {skill_file}: {e}")
            else:
                logger.info(f"用户技能目录不存在: {config_dir}")
        except Exception as e:
            logger.warning(f"扫描用户技能目录失败: {e}")
        
        # 发送技能列表
        sdk_message = {
            "type": "skills",
            "skills": skills
        }
        send_sdk_message(sdk_message)
        logger.info(f"总共返回 {len(skills)} 个 Skill")
        
    except Exception as e:
        logger.error(f"获取 Skill 列表失败: {e}", exc_info=True)
        error_message = {
            "type": "error",
            "message": str(e)
        }
        send_sdk_message(error_message)


async def handle_query(config: Dict[str, Any]):
    """处理查询请求 (单次交互)"""
    try:
        session_id = config.get('session_id', 'default')
        prompt = config.get('prompt', '')
        logger.info(f"处理查询 (session={session_id}): {prompt[:50]}...")

        # 执行查询
        async for message in query(prompt=prompt):
            logger.info(f"处理消息: {message.__class__.__name__}")

            # 将消息转换为 Rust SDKMessage 格式
            if hasattr(message, 'content'):
                content_blocks = []
                for block in message.content:
                    if hasattr(block, 'text'):
                        # TextBlock 格式: {"type": "text", "text": "..."}
                        content_blocks.append({
                            "type": "text",
                            "text": block.text
                        })

                    elif hasattr(block, 'thinking'):
                        # ThinkingBlock 格式: {"type": "thinking", "thinking": "...", "signature": "..."}
                        content_blocks.append({
                            "type": "thinking",
                            "thinking": block.thinking,
                            "signature": getattr(block, 'signature', '')
                        })

                    elif hasattr(block, 'name') and hasattr(block, 'input'):
                        # ToolUseBlock 格式: {"type": "tool_use", "id": "...", "name": "...", "input": {...}}
                        content_blocks.append({
                            "type": "tool_use",
                            "id": getattr(block, 'id', ''),
                            "name": block.name,
                            "input": block.input if isinstance(block.input, dict) else json.loads(block.input)
                        })

                    elif hasattr(block, 'tool_use_id'):
                        # ToolResultBlock 格式: {"type": "tool_result", "tool_use_id": "...", "content": "...", "is_error": false}
                        content_blocks.append({
                            "type": "tool_result",
                            "tool_use_id": block.tool_use_id,
                            "content": getattr(block, 'content', None),
                            "is_error": getattr(block, 'is_error', None)
                        })

                # 发送 Assistant 消息 (SDKMessage::Assistant)
                if content_blocks:
                    sdk_message = {
                        "type": "assistant",
                        "content": content_blocks,
                        "model": getattr(message, 'model', ''),
                        "parent_tool_use_id": getattr(message, 'parent_tool_use_id', None),
                        "error": getattr(message, 'error', None)
                    }
                    send_sdk_message(sdk_message)

            # 检查是否是结果消息 (ResultMessage)
            if message.__class__.__name__ == 'ResultMessage':
                # SDKMessage::Result 格式
                sdk_message = {
                    "type": "result",
                    "subtype": getattr(message, 'subtype', 'complete'),
                    "session_id": session_id,
                    "duration_ms": getattr(message, 'duration_ms', 0),
                    "duration_api_ms": getattr(message, 'duration_api_ms', 0),
                    "is_error": getattr(message, 'is_error', False),
                    "num_turns": getattr(message, 'num_turns', 0),
                    "total_cost_usd": getattr(message, 'total_cost_usd', None),
                    "result": getattr(message, 'result', None),
                    "usage": getattr(message, 'usage', None),
                    "errors": getattr(message, 'errors', None)
                }
                send_sdk_message(sdk_message)
                break

            # 检查是否是系统消息 (SystemMessage)
            elif message.__class__.__name__ == 'SystemMessage':
                sdk_message = {
                    "type": "system",
                    "content": getattr(message, 'content', '')
                }
                send_sdk_message(sdk_message)

        logger.info("查询执行完成")

    except Exception as e:
        logger.error(f"处理查询失败: {e}", exc_info=True)
        # 发送错误消息
        error_message = {
            "type": "result",
            "subtype": "error",
            "session_id": config.get('session_id', 'default'),
            "duration_ms": 0,
            "duration_api_ms": 0,
            "is_error": True,
            "num_turns": 0,
            "total_cost_usd": None,
            "result": str(e),
            "usage": None,
            "errors": [{"message": str(e)}]
        }
        send_sdk_message(error_message)


async def main():
    """主循环"""
    logger.info("CodeBuddy SDK 桥接脚本已启动")

    try:
        # 读取 stdin 中的第一个命令(单次模式)
        line = await asyncio.get_event_loop().run_in_executor(None, sys.stdin.readline)

        if not line:
            logger.info("收到 EOF, 退出")
            return

        # 解析配置
        config = json.loads(line.strip())
        command = config.get('command', 'query')

        logger.info(f"收到命令: {command}")

        if command == 'get_skills':
            # 获取 Skill 列表
            await handle_get_skills()
        elif command == 'query':
            # 处理查询请求
            await handle_query(config)
        elif command == 'client':
            logger.info("client 模式暂不支持")
            error_message = {
                "type": "result",
                "subtype": "error",
                "session_id": config.get('session_id', 'default'),
                "duration_ms": 0,
                "duration_api_ms": 0,
                "is_error": True,
                "num_turns": 0,
                "total_cost_usd": None,
                "result": "client 模式暂不支持",
                "usage": None,
                "errors": [{"message": "client 模式暂不支持"}]
            }
            send_sdk_message(error_message)
        else:
            logger.error(f"未知的命令: {command}")
            error_message = {
                "type": "result",
                "subtype": "error",
                "session_id": config.get('session_id', 'default'),
                "duration_ms": 0,
                "duration_api_ms": 0,
                "is_error": True,
                "num_turns": 0,
                "total_cost_usd": None,
                "result": f"未知的命令: {command}",
                "usage": None,
                "errors": [{"message": f"未知的命令: {command}"}]
            }
            send_sdk_message(error_message)

    except json.JSONDecodeError as e:
        logger.error(f"JSON 解析失败: {e}")
        logger.error(f"原始输入: {line.strip() if 'line' in locals() else 'N/A'}")
        error_message = {
            "type": "result",
            "subtype": "error",
            "session_id": "default",
            "duration_ms": 0,
            "duration_api_ms": 0,
            "is_error": True,
            "num_turns": 0,
            "total_cost_usd": None,
            "result": f"JSON 解析失败: {e}",
            "usage": None,
            "errors": [{"message": f"JSON 解析失败: {e}"}]
        }
        send_sdk_message(error_message)
    except KeyboardInterrupt:
        logger.info("收到中断信号,退出")
    except Exception as e:
        logger.error(f"主循环错误: {e}")
        import traceback
        traceback.print_exc()
        error_message = {
            "type": "result",
            "subtype": "error",
            "session_id": "default",
            "duration_ms": 0,
            "duration_api_ms": 0,
            "is_error": True,
            "num_turns": 0,
            "total_cost_usd": None,
            "result": str(e),
            "usage": None,
            "errors": [{"message": str(e)}]
        }
        send_sdk_message(error_message)
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
