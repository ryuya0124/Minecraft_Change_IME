@echo off
setlocal

:: �v���W�F�N�g��
set PROJECT_NAME=Minecraft_Change_IME_for_Rust

echo �r���h�J�n...
cargo build --release
if %ERRORLEVEL% neq 0 (
    echo �r���h�Ɏ��s���܂����B
    exit /b %ERRORLEVEL%
)

echo �����I
echo �o�̓t�@�C��: %EXE_PATH%
pause
