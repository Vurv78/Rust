@echo off
echo Building eratosthenes.exe
cargo build --release
echo Running eratosthenes.exe
target\release\eratosthenes.exe
pause