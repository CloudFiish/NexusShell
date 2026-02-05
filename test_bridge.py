import sys
import subprocess
import json

python_exe = r"d:\JF\Project\NexusShell\src-tauri\.venv\Scripts\python.exe"
bridge_script = r"d:\JF\Project\NexusShell\scripts\codebuddy_bridge.py"

input_data = {
    "command": "query",
    "prompt": "Hello"
}

print(f"Running bridge script: {bridge_script}")
process = subprocess.Popen(
    [python_exe, bridge_script],
    stdin=subprocess.PIPE,
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
    text=True,
    encoding='utf-8' # Explicit encoding
)

json_input = json.dumps(input_data) + "\n"
print(f"Sending input: {json_input.strip()}")

stdout, stderr = process.communicate(input=json_input)

print("\n--- STDOUT ---")
print(stdout)
print("\n--- STDERR ---")
print(stderr)
