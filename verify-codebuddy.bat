@echo off
chcp 65001 > nul
echo ==========================================
echo CodeBuddy Code Verification Script
echo ==========================================
echo.

echo [1/6] Check CodeBuddy Code installation status...
echo.
where codebuddy >nul 2>&1
if %errorlevel% equ 0 (
    echo [OK] CodeBuddy Code is installed
    codebuddy --version
    echo.
) else (
    echo [FAIL] CodeBuddy Code is not installed
    echo.
    echo Please install CodeBuddy Code first:
    echo   npm install -g @tencent-ai/codebuddy-code
    echo.
    pause
    exit /b 1
)

echo [2/6] View CodeBuddy Code help documentation...
echo.
codebuddy --help
echo.
echo Press any key to continue...
pause > nul
echo.

echo [3/6] Check CodeBuddy Code MCP related commands...
echo.
echo Checking mcp subcommand:
codebuddy mcp --help 2>nul
if %errorlevel% equ 0 (
    echo [OK] codebuddy mcp command is available
    codebuddy mcp --help
) else (
    echo [FAIL] codebuddy mcp command is not available
)
echo.
echo Press any key to continue...
pause > nul
echo.

echo [4/6] Test basic code generation functionality...
echo.
echo Creating test file...
echo console.log("Hello, CodeBuddy!"); > test_input.js
echo.
echo Execute codebuddy print command...
codebuddy -p "Generate a simple TypeScript interface with id and name fields" --model sonnet
echo.
echo Delete test file...
del test_input.js
echo.
echo Press any key to continue...
pause > nul
echo.

echo [5/6] View available MCP servers...
echo.
codebuddy mcp list 2>nul
if %errorlevel% equ 0 (
    echo [OK] Successfully retrieved MCP server list
) else (
    echo [WARN] Could not retrieve MCP server list
    echo This might be normal depending on configuration
)
echo.
echo Press any key to continue...
pause > nul
echo.

echo [6/6] Test interactive mode (if supported)...
echo.
echo Starting interactive test (auto-exit after 10 seconds)...
echo Note: This may require login or configuration
echo.
start /b cmd /c "echo test | timeout /t 10 > nul & codebuddy"
timeout /t 12 > nul
echo.
echo ==========================================
echo Verification Complete!
echo ==========================================
echo.
echo Please review the output above and record the following information:
echo 1. CodeBuddy Code version number
echo 2. Whether mcp subcommand is supported
echo 3. Output format (JSON/Plain text)
echo 4. Whether command line parameters are supported (e.g. --model, --print)
echo 5. Interactive mode output format
echo.
pause
