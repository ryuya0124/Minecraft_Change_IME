@echo off
setlocal

:: �v���W�F�N�g��
set PROJECT_NAME=rust

echo �r���h�J�n...
cargo build --release
if %ERRORLEVEL% neq 0 (
    echo �r���h�Ɏ��s���܂����B
    exit /b %ERRORLEVEL%
)

echo �����I
echo �o�̓t�@�C��: %EXE_PATH%
pause
