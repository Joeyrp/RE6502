
@echo off

REM Usage: Drop the win2c64 directory into the same directory with this script
REM		Directory structure should be: ./assemble.bat ./win2c64/win2c64.exe ./bin


set src_file=%1
echo assembling %src_file%
win2c64\win2c64 -R %src_file%
if %ERRORLEVEL% NEQ 0 (
	echo failed to assemble %src_file%
	goto EXIT
)
set out_file=%src_file:~0,-4%
set out_file=%out_file%.rw

xcopy /y %out_file% bin\
echo generated bin\%out_file%

:EXIT