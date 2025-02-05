@echo off
setlocal

:: プロジェクト名
set PROJECT_NAME=rust

echo ビルド開始...
cargo build --release
if %ERRORLEVEL% neq 0 (
    echo ビルドに失敗しました。
    exit /b %ERRORLEVEL%
)

echo 完了！
echo 出力ファイル: %EXE_PATH%
pause
