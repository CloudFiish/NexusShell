import sys
import os
import subprocess
import threading
import time

# 当通过 python -c 运行时，__file__ 可能不存在，我们假设 CWD 是项目根目录
CWD = os.getcwd()
REAL_BRIDGE = os.path.join(CWD, 'src-tauri', 'scripts', 'codebuddy_bridge.py')
LOG_FILE = os.path.join(CWD, 'scripts', 'bridge_debug.log')

def log(message):
    try:
        with open(LOG_FILE, 'a', encoding='utf-8') as f:
            timestamp = time.strftime("%H:%M:%S", time.localtime())
            f.write(f"[{timestamp}] {message}\n")
    except Exception:
        pass

def forward_stream(source, dest, prefix):
    try:
        for line in source:
            log(f"[{prefix}] {line.strip()}")
            dest.write(line)
            dest.flush()
    except Exception as e:
        log(f"[{prefix}] Error: {e}")

log("--- Debug Bridge Started ---")
log(f"Real bridge path: {REAL_BRIDGE}")
log(f"Python executable: {sys.executable}")

# 启动真实的 bridge
try:
    process = subprocess.Popen(
        [sys.executable, REAL_BRIDGE],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        encoding='utf-8',
        bufsize=0 # 无缓冲
    )
except Exception as e:
    log(f"Failed to start process: {e}")
    sys.exit(1)

# 启动线程转发 STDOUT 和 STDERR
t_out = threading.Thread(target=forward_stream, args=(process.stdout, sys.stdout, "PYTHON_OUT"))
t_err = threading.Thread(target=forward_stream, args=(process.stderr, sys.stderr, "PYTHON_ERR"))
t_out.daemon = True
t_err.daemon = True
t_out.start()
t_err.start()

# 转发 STDIN
try:
    # 强制将 sys.stdin 重新包装为 utf-8，避免 Windows 上的编码问题
    stdin_utf8 = sys.stdin
    if sys.platform == 'win32':
        import io
        stdin_utf8 = io.TextIOWrapper(sys.stdin.buffer, encoding='utf-8')

    for line in stdin_utf8:
        log(f"[RUST_IN] {line.strip()}")
        process.stdin.write(line)
        process.stdin.flush()
    
    # Close stdin to signal EOF to the subprocess
    process.stdin.close()
    # Wait for the subprocess to finish
    process.wait()

except KeyboardInterrupt:
    log("KeyboardInterrupt")
except Exception as e:
    log(f"STDIN Error: {e}")
finally:
    log("Terminating process")
    process.terminate()
