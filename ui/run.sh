#!/bin/bash

cd "$(dirname "$0")"
cd ..

echo "Checking Python installation..."
if ! command -v python3 &> /dev/null; then
    echo "Python is not installed"
    echo "Please install Python 3.9 or later"
    exit 1
fi

echo "Checking PySide6 installation..."
if ! python3 -c "import PySide6" &> /dev/null; then
    echo "Installing PySide6..."
    python3 -m pip install PySide6 --user
fi

echo "Starting Raito UI..."
echo ""
python3 -m ui "$@"
