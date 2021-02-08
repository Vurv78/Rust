@echo off
cargo build --release
echo Running happy_numbers.exe
target\release\happy_numbers.exe
pause