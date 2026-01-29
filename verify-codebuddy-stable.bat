@echo off
chcp 65001 > nul
setlocal enabledelayedexpansion

echo ==========================================
echo CodeBuddy Code Verification Script (Stable Version)
echo ==========================================
echo.

:: Create log directory
if not exist "verification_logs" mkdir "verification_logs"
set LOG_FILE=verification_logs\verification_%date:~0,4%%date:~5,2%%date:~8,2%_time:~0,2%%time:~3,2%%time:~6,2%.log

echo Starting verification... Log will be saved to: %LOG_FILE%
echo.

echo [1/6] Check CodeBuddy Code installation status...
echo Checking installation... >> %LOG_FILE% 2>&1
where codebuddy >nul 2>&1
if %errorlevel% equ 0 (
    for /f "delims=" %%i in ('codebuddy --version') do set CODEBUDDY_VERSION=%%i
    echo [OK] CodeBuddy Code is installed
    echo Version: %CODEBUDDY_VERSION%
    echo Version: %CODEBUDDY_VERSION% >> %LOG_FILE%
) else (
    echo [FAIL] CodeBuddy Code is not installed
    echo.
    echo Please install CodeBuddy Code first:
    echo   npm install -g @tencent-ai/codebuddy-code
    echo.
    echo Installation Guide: https://cnb.cool/codebuddy/codebuddy-code
    echo.
    pause
    goto :end
)

echo.
echo [2/6] View CodeBuddy Code help documentation...
echo Getting help documentation... >> %LOG_FILE%
echo.
echo ----------------------------------------
codebuddy --help
echo ----------------------------------------
echo.

echo Check the output above for:
echo - WebSocket related options
echo - Output format control options
echo - MCP related commands
echo.
echo Press any key to continue...
pause >nul
echo.

echo [3/6] Check CodeBuddy Code MCP related commands...
echo Checking MCP commands... >> %LOG_FILE%
echo.
echo Checking mcp subcommand... >> %LOG_FILE% 2>&1
codebuddy mcp --help >nul 2>&1
if %errorlevel% equ 0 (
    echo [OK] codebuddy mcp command is available
    echo.
    echo Getting MCP help... >> %LOG_FILE%
    echo ----------------------------------------
    codebuddy mcp --help
    echo ----------------------------------------
) else (
    echo [FAIL] codebuddy mcp command is not available
)
    echo This might be normal depending on your version
)
echo.
echo Press any key to continue...
pause >nul
echo.

echo [4/6] Test basic code generation functionality...
echo Creating test file... >> %LOG_FILE%
echo console.log("Hello, CodeBuddy!"); > test_input.js
echo.
echo Executing codebuddy print command...
echo Testing code generation... >> %LOG_FILE%
echo ----------------------------------------
codebuddy -p "Generate a simple TypeScript interface with id and name fields"
echo ----------------------------------------
echo.
echo Checking output format...
echo Deleting test file... >> %LOG_FILE%
del test_input.js 2>nul
echo.
echo Press any key to continue...
pause >nul
echo.

echo [5/6] View available MCP servers...
echo Getting MCP server list... >> %LOG_FILE%
echo.
echo Getting server list... >> %LOG_FILE% 2>&1
codebuddy mcp list 2>nul
if %errorlevel% equ 0 (
    echo [OK] Successfully retrieved MCP server list
    echo.
    echo ----------------------------------------
    codebuddy mcp list
    echo ----------------------------------------
) else (
    echo [WARN] Could not retrieve MCP server list
    echo This might be normal depending on configuration
    echo or MCP might not be configured yet
)
echo.
echo Press any key to continue...
pause >nul
echo.

echo [6/6] Test interactive mode (if supported)...
echo.
echo Note: Interactive mode test will start (10 second timeout)...
echo This might require login or configuration
echo.
echo Starting interactive test... >> %LOG_FILE%
echo.
start /b cmd /c "echo interactive test | timeout /t 10 > nul & codebuddy"
timeout /t 12 >nul 2>nul
echo.

echo.
echo ==========================================
echo Verification Complete!
echo ==========================================
echo.
echo All logs have been saved to: %LOG_FILE%
echo.
echo Please review the output above and check for:
echo 1. CodeBuddy Code version number
echo 2. Available commands and options
echo 3. Output format (JSON/Plain text)
echo 4. MCP support
echo 5. Interactive mode behavior
echo.
echo ==========================================
echo Analysis Questions:
echo ==========================================
echo.
echo 1. Does codebuddy --help show WebSocket options?
echo    - Look for keywords: websocket, ws, socket, server
echo.
echo 2. Does codebuddy support --print or -p option?
echo.
echo 3. Does codebuddy have MCP subcommands?
echo.
echo 4. What is the default output format?
echo    - Plain text, JSON, or mixed?
echo.
echo 5. Does the output include special markers?
echo    - Examples: @@UI_DATA@@, ::UI_PROGRESS::, etc.
echo.
echo.
echo ==========================================
echo Quick Decision Guide:
echo ==========================================
echo.
echo Based on your findings, choose a communication approach:
echo.
echo A. WebSocket Support (BEST)
echo    - If codebuddy shows websocket options
echo    - Recommended: Use WebSocket communication
echo.
echo B. JSON Output (GOOD)
echo    - If codebuddy --p outputs JSON format
echo    - Recommended: Parse stdout JSON output
echo.
echo C. Special Markers (ACCEPTABLE)
echo    - If output contains @@UI_DATA@@ markers
echo    - Recommended: Parse special markers
echo.
echo D. File I/O (FALLBACK)
echo    - If none of the above work
echo    - Recommended: Use file system communication
echo.
echo.

:end
echo.
echo Verification script completed.
echo.
pause
