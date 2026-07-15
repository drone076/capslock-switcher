================================================================================
                         CAPSLOCK SWITCHER - USER GUIDE
================================================================================

WHAT IS THIS PROGRAM?
---------------------
Capslock Switcher is a minimalist utility for Windows that turns the useless
CapsLock key into a convenient keyboard layout switcher.

Instead of reaching for Alt+Shift or Ctrl+Shift, you simply press CapsLock with
one finger — and the layout switches instantly.


HOW DOES IT WORK?
-----------------
- Press CapsLock → switches layout (RU ↔ EN)
- Shift + CapsLock → toggles uppercase mode (regular CapsLock)

The program runs completely in the background, is invisible, consumes no
resources, and doesn't show any windows or tray icons.


INSTALLATION
------------
1. Extract the CapslockSwitcher_Setup.zip archive to any folder
2. Run the install.bat file
3. Confirm the administrator privileges request (click "Yes" in the UAC window)
4. Done! The program will install and launch automatically

What happens during installation:
- The program is copied to C:\Program Files\CapslockSwitcher\
- A task is created in Windows Task Scheduler for auto-start
- The program is registered in "Programs and Features"
- The program launches immediately after installation


USAGE
-----
Simply press CapsLock to switch the layout.

If you need to type text in uppercase:
- Hold Shift and press the desired letters
- Or press Shift + CapsLock to enable uppercase mode

The program works in all applications, including:
- Browsers
- Text editors
- Task Manager
- Windows system windows


UNINSTALLATION
--------------
Method 1 (recommended):
1. Open "Settings" → "Apps" → "Installed apps"
   (or "Control Panel" → "Programs and Features")
2. Find "Capslock Switcher" in the list
3. Click "Uninstall"

Method 2:
1. Go to C:\Program Files\CapslockSwitcher\
2. Run uninstall.bat
3. Confirm the administrator privileges request

What happens during uninstallation:
- The program stops
- The task is removed from Task Scheduler
- The entry is removed from "Programs and Features"
- All program files are deleted


FREQUENTLY ASKED QUESTIONS
---------------------------
Q: The program is not visible in the tray. Is this normal?
A: Yes, this is intentional. The program runs completely in the background and
   doesn't distract attention.

Q: Why does a UAC window appear when launching the program?
A: This is normal only during first launch or manual installation. After
   installation, the program will start automatically without UAC prompts.

Q: Can I use CapsLock for both layout switching and uppercase letters at the
   same time?
A: Yes! Plain CapsLock switches the layout, while Shift + CapsLock enables
   uppercase mode.

Q: Does the program work on Windows 10?
A: Yes, Windows 10 and Windows 11 are supported.

Q: How do I check if the program is running?
A: Open Task Manager (Ctrl+Shift+Esc), go to the "Details" tab, and find
   capslock_switcher.exe in the process list.


TECHNICAL INFORMATION
---------------------
Program size: ~150 KB
Memory usage: ~2 MB
CPU usage: 0%
Programming language: Rust
License: MIT

Homepage: https://github.com/drone076/capslock-switcher


SUPPORT
-------
If you found a bug or have suggestions for improvement, please create an issue
on GitHub:
https://github.com/drone076/capslock-switcher/issues


ACKNOWLEDGMENTS
---------------
Thank you for using Capslock Switcher!

This program is made for those who appreciate minimalism and efficiency.

================================================================================
                       https://github.com/drone076 © 2026
================================================================================
