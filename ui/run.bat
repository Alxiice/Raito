@echo off
setlocal

cd %~dp0
cd ..

echo Checking Python installation...
python --version >nul 2>&1
if errorlevel 1 (
    echo Python is not installed or not in PATH
    echo Please install Python 3.9 or later
    pause
    exit /b 1
)

echo Checking PySide6 installation...
python -c "import PySide6" >nul 2>&1
if errorlevel 1 (
    echo Installing PySide6...
    python -m pip install PySide6 --user
)

echo Starting Raito UI...
echo.
python -m ui %*

endlocal
