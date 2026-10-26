
---

## 📄 README.en.md (English version)

# ⌨️ Capslock Switcher

**A minimalist utility for Windows 11 that switches keyboard layout with `CapsLock`.**

[![Rust](https://img.shields.io/badge/Rust-1.70+-000000?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Windows](https://img.shields.io/badge/Windows-10/11-0078D6?style=flat-square&logo=windows&logoColor=white)](https://www.microsoft.com/windows)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)](LICENSE)
[![Size](https://img.shields.io/badge/Size-~240KB-blue?style=flat-square)](https://github.com/drone076/capslock-switcher/releases)
[![Stars](https://img.shields.io/github/stars/drone076/capslock-switcher?style=social)](https://github.com/drone076/capslock-switcher/stargazers)

[Русская версия](README.md)

---

## 🎯 The Problem

`CapsLock` is the most useless key on the keyboard. It occupies a convenient spot under your pinky finger, but it's only used for TYPING IN ALL CAPS (which you can do with `Shift` anyway).

Yet it's the perfect place for a layout switcher!

### Why not existing solutions?

**Punto Switcher:**
- ❌ Bloated (hundreds of megabytes)
- ❌ Interferes with clipboard and auto-correction
- ❌ Requires .NET Framework
- ❌ Unstable on Windows 11

**PowerToys Keyboard Manager:**
- ❌ Requires installing the entire PowerToys suite
- ❌ Overly complicated for such a simple task

**Other alternatives:**
- ❌ Consume memory
- ❌ Show tray icons
- ❌ Send telemetry

---

## ✨ The Solution

**Capslock Switcher** turns the useless `CapsLock` key into the most convenient layout switcher you've ever had.

- ✅ **Only ~240 KB** (ZIP archive ~130 KB)
- ✅ **Written in Rust** — fast, safe, no dependencies
- ✅ **Works everywhere** — in all applications, including system windows
- ✅ **Invisible** — no window, no tray icon, no notifications
- ✅ **Simple** — does one thing and does it perfectly
- ✅ **Zero resource consumption** in the background
- ✅ **Instant response** (< 1 ms)

---

## 🚀 Installation

### Quick installation (recommended)

1. Download the latest release: [**CapslockSwitcher_Setup.zip**](https://github.com/drone076/capslock-switcher/releases/latest)
2. Extract the archive to any folder
3. Run `install.bat`
4. Confirm the administrator privileges request (UAC)
5. Done! ✨

The program will automatically:
- Copy itself to a system folder
- Add itself to startup
- Configure launch with the necessary privileges

At the next system login, it will quietly start in the background and work.

### Manual installation

If you prefer to control everything manually:

1. Download `capslock_switcher.exe` from [releases](https://github.com/drone076/capslock-switcher/releases/latest)
2. Copy it to a convenient location (e.g., `C:\Utils\`)
3. Create a shortcut and place it in the startup folder:
   - Press `Win + R`, type `shell:startup`, press Enter
   - Copy the shortcut to the opened folder
4. Right-click the shortcut → **Properties** → **Advanced** → ✅ **Run as administrator**

---

## 📖 How to use

Simply press `CapsLock` instead of your Windows layout shortcut — `Alt+Shift`, `Ctrl+Shift`, or `Win+Space`. The utility detects the configured shortcut automatically.

**Important:** After installing the program, the `CapsLock` key no longer enables uppercase mode. If you need to type something in uppercase, use `Shift` + the desired keys (or `Shift+CapsLock`).

If you change the shortcut in Windows settings, restart the utility.

---

## ❓ FAQ

### How do I uninstall the program?
Run `uninstall.bat` from the distribution, or manually remove:
- The file `capslock_switcher.exe` from `C:\Program Files\CapslockSwitcher\`
- The shortcut from the startup folder (`shell:startup`)

### The program is not visible in the tray. Is this normal?
Yes, this is intentional. The program works completely in the background, consumes no resources, and doesn't distract attention. It just does its job.

### Is Windows 10 supported?
Yes, the program works on Windows 10 and Windows 11.

### Can I use CapsLock for both switching and uppercase letters?
Yes. A normal `CapsLock` press switches the layout, while `Shift+CapsLock` enables uppercase mode.

---

## 🛠️ For Developers

If you want to build the project yourself or make changes:

### Requirements
- Rust 1.70+ ([rustup](https://rustup.rs/))
- MinGW-w64 (for cross-compilation from Linux)
- zip, dos2unix (for creating the distribution)

### Installing dependencies (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install mingw-w64 zip dos2unix
```

### Adding Windows target to Rust
```bash
rustup target add x86_64-pc-windows-gnu
```

### Building
```bash
# Configure linker
mkdir -p .cargo
cat > .cargo/config.toml << EOF
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
EOF

# Build the project
cargo build --release --target x86_64-pc-windows-gnu

# Reduce size
x86_64-w64-mingw32-strip target/x86_64-pc-windows-gnu/release/capslock_switcher.exe
```

The ready .exe will be in target/x86_64-pc-windows-gnu/release/

### Automatic distribution build
```bash
chmod +x build.sh
./build.sh
```

This will create a dist/ folder with a ready ZIP archive for distribution.

### 🤝 Contributing
Bugs, ideas, and pull requests are welcome!
If you like the project, give it a ⭐ star — it motivates further development!

### 📄 License
MIT License — use freely for any purpose.

### 🙏 Acknowledgments
Rust — for the magnificent language
windows-rs — for excellent Win32 API bindings
Everyone who's tired of CapsLock and wants to make the keyboard more convenient

---
Made with ❤️  for those who appreciate minimalism and efficiency.


