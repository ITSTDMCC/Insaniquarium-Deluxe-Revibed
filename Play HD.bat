@echo off
rem Starts the port with the HD art made by tools\upscale_art.py (see README.md, "HD art").
rem If your game is not in the "Insaniquarium Deluxe" folder next to this one, put its path here:
set "GAME=%~dp0..\Insaniquarium Deluxe"
start "" "%~dp0target\release\winfish_rs.exe" "%GAME%" --hd
