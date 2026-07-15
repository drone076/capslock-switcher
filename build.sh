#!/bin/bash
set -e

echo "🧹 [1/5] Очистка предыдущих сборок..."
cargo clean

echo "⚙️ [2/5] Компиляция под Windows x64..."
cargo build --release --target x86_64-pc-windows-gnu

echo "✂️ [3/5] Оптимизация размера бинарного файла (strip)..."
x86_64-w64-mingw32-strip target/x86_64-pc-windows-gnu/release/capslock_switcher.exe

echo "📦 [4/5] Подготовка директории dist/ и копирование файлов..."
mkdir -p dist
cp target/x86_64-pc-windows-gnu/release/capslock_switcher.exe dist/
cp install.bat dist/
cp uninstall.bat dist/
cp README.txt dist/
cp README.en.txt dist/

if command -v unix2dos &> /dev/null; then
    unix2dos dist/*.bat > /dev/null 2>&1
    unix2dos dist/*.txt > /dev/null 2>&1
    echo "   ↳ Конвертация окончаний строк в .bat файлах выполнена."
else
    echo "   ⚠️ Утилита 'unix2dos' не найдена. Рекомендуется установить её (sudo apt install dos2unix), чтобы .bat файлы корректно работали в Windows."
fi

echo "🗜️ [5/5] Создание ZIP-архива для дистрибуции..."
cd dist
zip -j CapslockSwitcher_Setup.zip capslock_switcher.exe install.bat uninstall.bat README.txt README.en.txt > /dev/null
cd ..

echo "✅ Готово! Дистрибутив успешно создан:"
ls -lh dist/
