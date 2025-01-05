@echo off

call "%PROGRAMFILES%\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat"

set NAME=launcher

cp launcher.res target/release
cd target/release

rh -open %NAME%.exe -save backup.exe -action addskip -res %NAME%.res
rm -f %NAME%.exe
mv backup.exe %NAME%.exe

editbin /SUBSYSTEM:Windows %NAME%.exe

upx --ultra-brute --no-lzma %NAME%.exe

pause
