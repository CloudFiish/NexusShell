# Quick Manual Verification Guide

If the auto-script has issues, use this quick manual verification.

## Step 1: Check Installation
```bash
codebuddy --version
```
Expected:
- Version number (e.g., 1.2.3) - Installed
- "command not found" - Not installed

## Step 2: Check Help for Communication Options
```bash
codebuddy --help
```
Look for:
- WebSocket options (--websocket, --ws, --server)
- Output format (--format, --output)
- MCP commands (mcp)
- Print mode (--print, -p)

## Step 3: Test Output Format
```bash
codebuddy -p "Hello World"
```
Observe:
- Output format (plain text/JSON)
- Response time
- Error messages

## Step 4: Check MCP Support
```bash
codebuddy mcp --help
codebuddy mcp list
```

Look for:
- MCP command availability
- Server list success/failure

## Quick Analysis

### WebSocket Support?
- Check help for: websocket, ws, socket, server
- If found → WebSocket possible
- If not found → WebSocket unlikely

### JSON Output?
- Run step 3
- Check if output is JSON format
- If structured JSON → JSON output possible

### Special Markers?
- Look for: @@, ::, or other delimiters
- If present → Marker parsing possible

## Report Template

```
CodeBuddy Version: ______
WebSocket Support: Yes/No
JSON Output: Yes/No
Special Markers: Yes/No
MCP Support: Yes/No

Recommended Option: [A/B/C/D]
Reason: ______
```
