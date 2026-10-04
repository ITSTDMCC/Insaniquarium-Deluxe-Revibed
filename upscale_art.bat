@echo off
setlocal
rem Upscales the game's art with Real-ESRGAN into "<game>\hd". Double-click to run.
rem Asks for each folder; press Enter to keep the default shown in brackets.

call :ask GAME   "Insaniquarium Deluxe game folder"          "%~dp0Insaniquarium Deluxe" "properties\resources.xml"
call :ask ESRGAN "Real-ESRGAN folder (holds the exe)"        "%~dp0upscale\realesrgan"   "realesrgan-ncnn-vulkan.exe"

echo.
python tools\upscale_art.py --game "%GAME%" --esrgan "%ESRGAN%" %*
echo.
echo Finished with exit code %ERRORLEVEL%.
pause
exit /b

rem :ask VAR "prompt" "default" "file that must exist inside the folder"
:ask
set "%~1="
set /p "%~1=%~2 [%~3]: "
if not defined %~1 set "%~1=%~3"
call set "%~1=%%%~1:"=%%"
call set "_dir=%%%~1%%"
if exist "%_dir%\%~4" exit /b
echo   "%_dir%\%~4" was not found, try again.
goto ask
