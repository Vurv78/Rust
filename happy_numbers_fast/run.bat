@echo off
echo Building happy_numbers_fast.exe
cargo build --release
echo Running happy_numbers_fast.exe
target\release\happy_numbers_fast.exe
pause