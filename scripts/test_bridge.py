#!/usr/bin/env python3
"""
简单的集成测试 - 验证 Python 桥接脚本的基本功能

这个脚本模拟 Rust 后端发送配置到 Python 桥接脚本
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

def test_bridge_syntax():
    """测试桥接脚本语法"""
    print("=" * 60)
    print("测试 1: 桥接脚本语法检查")
    print("=" * 60)

    try:
        result = subprocess.run(
            [VENV_PYTHON, '-m', 'py_compile', BRIDGE_SCRIPT],
            capture_output=True,
            text=True,
            timeout=10
        )

        if result.returncode == 0:
            print("✅ 语法检查通过")
            return True
        else:
            print(f"❌ 语法检查失败: {result.stderr}")
            return False

    except Exception as e:
        print(f"❌ 测试失败: {e}")
        return False

def test_bridge_import():
    """测试桥接脚本导入"""
    print("\n" + "=" * 60)
    print("测试 2: 桥接脚本导入")
    print("=" * 60)

    try:
        # 读取桥接脚本内容
        with open(BRIDGE_SCRIPT, 'r', encoding='utf-8') as f:
            script_content = f.read()

        # 尝试导入
        result = subprocess.run(
            [VENV_PYTHON, '-c', script_content],
            capture_output=True,
            text=True,
            timeout=5,
            input='{"command": "exit"}\n'
        )

        # 检查是否有导入错误
        if 'ImportError' in result.stderr:
            print(f"❌ 导入失败: {result.stderr}")
            return False
        else:
            print("✅ 导入检查通过")
            return True

    except Exception as e:
        print(f"❌ 测试失败: {e}")
        return False

def test_simple_query():
    """测试简单查询"""
    print("\n" + "=" * 60)
    print("测试 3: 简单查询")
    print("=" * 60)

    try:
        # 创建测试配置
        config = {
            "command": "query",
            "session_id": "test_session_001",
            "prompt": "1+1=?"
        }

        print(f"发送配置: {json.dumps(config)}")

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

        # 读取输出（带超时）
        start_time = time.time()
        timeout = 60  # 60 秒超时

        output_lines = []
        error_lines = []

        while True:
            if time.time() - start_time > timeout:
                print(f"⚠️  超时 ({timeout}秒)")
                process.terminate()
                break

            # 检查进程是否结束
            if process.poll() is not None:
                break

            # 读取输出
            try:
                # 使用非阻塞读取
                import select
                if select.select([process.stdout], [], [], 0.1)[0]:
                    line = process.stdout.readline()
                    if line:
                        output_lines.append(line.strip())
                        print(f"  [OUT] {line.strip()}[:100]")

                if select.select([process.stderr], [], [], 0.1)[0]:
                    line = process.stderr.readline()
                    if line:
                        error_lines.append(line.strip())

            except:
                break

            # 如果收到了 execution_complete，可以退出
            for line in output_lines:
                if 'execution_complete' in line:
                    print("✓ 收到执行完成信号")
                    process.terminate()
                    break
            else:
                continue
            break

        # 等待进程结束
        process.wait(timeout=5)

        # 分析输出
        print(f"\n收到 {len(output_lines)} 行输出")
        print(f"收到 {len(error_lines)} 行错误")

        has_data_chunk = any('data_chunk' in line for line in output_lines)
        has_execution_complete = any('execution_complete' in line for line in output_lines)

        if has_data_chunk:
            print("✅ 收到数据块")

        if has_execution_complete:
            print("✅ 收到执行完成事件")
            return True
        else:
            print("⚠️  未收到执行完成事件")
            return False

    except subprocess.TimeoutExpired:
        print("❌ 测试超时")
        return False
    except Exception as e:
        print(f"❌ 测试失败: {e}")
        import traceback
        traceback.print_exc()
        return False

def test_error_handling():
    """测试错误处理"""
    print("\n" + "=" * 60)
    print("测试 4: 错误处理")
    print("=" * 60)

    try:
        # 创建无效配置
        config = {
            "command": "invalid_command"
        }

        print(f"发送无效配置: {json.dumps(config)}")

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
        process.stdin.close()

        # 读取输出（短超时）
        start_time = time.time()
        timeout = 10  # 10 秒超时

        output_lines = []
        error_lines = []

        while True:
            if time.time() - start_time > timeout:
                process.terminate()
                break

            if process.poll() is not None:
                break

            try:
                import select
                if select.select([process.stdout], [], [], 0.1)[0]:
                    line = process.stdout.readline()
                    if line:
                        output_lines.append(line.strip())
                        print(f"  [OUT] {line.strip()}")

                if select.select([process.stderr], [], [], 0.1)[0]:
                    line = process.stderr.readline()
                    if line:
                        error_lines.append(line.strip())
                        print(f"  [ERR] {line.strip()}")

            except:
                break

            # 如果收到了 error 消息，可以退出
            for line in output_lines:
                if '"type": "error"' in line:
                    print("✓ 收到错误事件")
                    process.terminate()
                    break
            else:
                continue
            break

        process.wait(timeout=5)

        # 检查是否收到错误消息
        has_error = any('"type": "error"' in line for line in output_lines)

        if has_error:
            print("✅ 错误处理正常")
            return True
        else:
            print("⚠️  未收到错误消息")
            return False

    except Exception as e:
        print(f"❌ 测试失败: {e}")
        return False

def main():
    """主测试函数"""
    print("\n" + "=" * 60)
    print("Python 桥接脚本集成测试")
    print("=" * 60)

    results = []

    # 运行测试
    results.append(("语法检查", test_bridge_syntax()))
    results.append(("导入检查", test_bridge_import()))
    results.append(("简单查询", test_simple_query()))
    results.append(("错误处理", test_error_handling()))

    # 总结
    print("\n" + "=" * 60)
    print("测试总结")
    print("=" * 60)

    total = len(results)
    passed = sum(1 for _, result in results if result)

    for name, result in results:
        status = "✅" if result else "❌"
        print(f"{status} {name}")

    print(f"\n总测试数: {total}")
    print(f"通过: {passed}")
    print(f"失败: {total - passed}")

    if passed == total:
        print("\n✅ 所有测试通过！")
        return 0
    else:
        print(f"\n⚠️  有 {total - passed} 个测试失败")
        return 1

if __name__ == '__main__':
    sys.exit(main())
