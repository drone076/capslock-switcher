@echo off
chcp 65001 >nul
title Установка Capslock Switcher

:: 1. Запрос прав администратора
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo Запрос прав администратора...
    powershell -Command "Start-Process '%~f0' -Verb RunAs"
    exit /b
)

echo.
echo [1/5] Копирование файлов...
set "TARGET_DIR=C:\Program Files\CapslockSwitcher"
if not exist "%TARGET_DIR%" mkdir "%TARGET_DIR%"
copy /Y "%~dp0capslock_switcher.exe" "%TARGET_DIR%" >nul
copy /Y "%~dp0uninstall.bat" "%TARGET_DIR%\uninstall.bat" >nul

echo [2/5] Создание задачи в Планировщике заданий...
:: Удаляем старую задачу, если есть
schtasks /Delete /TN "CapslockSwitcher" /F >nul 2>&1

:: Создаем новую задачу с запуском при входе в систему с наивысшими привилегиями
schtasks /Create /TN "CapslockSwitcher" /TR "\"%TARGET_DIR%\capslock_switcher.exe\"" /SC ONLOGON /RL HIGHEST /F >nul

echo [3/5] Регистрация в "Программы и компоненты"...
set "APP_NAME=Capslock Switcher"
set "APP_VERSION=1.0.0"
set "APP_PUBLISHER=Drone, Inc"
set "APP_URL=https://github.com/drone076/capslock-switcher"
set "UNINSTALL_STRING=cmd /c \"%TARGET_DIR%\uninstall.bat\""

reg add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\CapslockSwitcher" /v "DisplayName" /t REG_SZ /d "%APP_NAME%" /f >nul
reg add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\CapslockSwitcher" /v "DisplayVersion" /t REG_SZ /d "%APP_VERSION%" /f >nul
reg add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\CapslockSwitcher" /v "Publisher" /t REG_SZ /d "%APP_PUBLISHER%" /f >nul
reg add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\CapslockSwitcher" /v "URLInfoAbout" /t REG_SZ /d "%APP_URL%" /f >nul
reg add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\CapslockSwitcher" /v "UninstallString" /t REG_SZ /d "%UNINSTALL_STRING%" /f >nul
reg add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\CapslockSwitcher" /v "NoModify" /t REG_DWORD /d 1 /f >nul
reg add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\CapslockSwitcher" /v "NoRepair" /t REG_DWORD /d 1 /f >nul

echo [4/5] Запуск программы...
start "" "%TARGET_DIR%\capslock_switcher.exe"

echo [5/5] Готово!
echo.
echo ==========================================
echo  Установка завершена успешно! 
echo  Программа установлена в: %TARGET_DIR%
echo  Она будет автоматически запускаться
echo  при каждом входе в систему с нужными
echo  правами (без UAC-запросов).
echo ==========================================
echo.
pause
