@echo off
chcp 65001 >nul
title Удаление Capslock Switcher

net session >nul 2>&1
if %errorLevel% neq 0 (
    powershell -Command "Start-Process '%~f0' -Verb RunAs"
    exit /b
)

echo Остановка программы...
taskkill /F /IM capslock_switcher.exe >nul 2>&1

echo Удаление задачи из Планировщика...
schtasks /Delete /TN "CapslockSwitcher" /F >nul 2>&1

echo Удаление из "Программы и компоненты"...
reg delete "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\CapslockSwitcher" /f >nul 2>&1

echo Удаление файлов...
set "TARGET_DIR=C:\Program Files\CapslockSwitcher"
if exist "%TARGET_DIR%" rmdir /S /Q "%TARGET_DIR%" >nul

echo.
echo ==========================================
echo  Программа полностью удалена из системы.
echo ==========================================
echo.
pause
