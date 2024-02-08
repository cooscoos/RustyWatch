#!/bin/bash

# Check if python is installed
if command -v python3 &>/dev/null; then
    python3 tally.py
elif command -v python &>/dev/null; then
    python tally.py
else
    echo "Python is not installed. Please install Python to run this script."
    exit 1
fi