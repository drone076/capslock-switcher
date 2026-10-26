#![windows_subsystem = "windows"]

use std::sync::atomic::{AtomicBool, Ordering};
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::System::Registry::*;
use windows::Win32::System::Threading::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;

const WM_SWITCH_LAYOUT: u32 = WM_APP + 1;

static NEED_SWITCH: AtomicBool = AtomicBool::new(false);
static CAPS_WAS_BLOCKED: AtomicBool = AtomicBool::new(false);

fn make_input(vk: VIRTUAL_KEY, flags: KEYBD_EVENT_FLAGS) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                wScan: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LayoutHotkey {
    AltShift,
    CtrlShift,
    Grave,
    WinSpace,
}

unsafe fn read_toggle_value(name: PCWSTR) -> Option<u32> {
    let mut data = [0u16; 32];
    let mut size = std::mem::size_of_val(&data) as u32;
    let mut kind = REG_VALUE_TYPE::default();

    unsafe {
        RegGetValueW(
            HKEY_CURRENT_USER,
            w!("Keyboard Layout\\Toggle"),
            name,
            RRF_RT_REG_SZ | RRF_RT_REG_DWORD,
            Some(&mut kind),
            Some(data.as_mut_ptr() as *mut _),
            Some(&mut size),
        )
        .ok()?;
    }

    if kind == REG_SZ {
        let len = (size as usize / 2).saturating_sub(1);
        String::from_utf16_lossy(&data[..len]).trim().parse().ok()
    } else if kind == REG_DWORD {
        let value = u32::from_ne_bytes([
            (data[0] & 0xff) as u8,
            (data[0] >> 8) as u8,
            (data[1] & 0xff) as u8,
            (data[1] >> 8) as u8,
        ]);
        Some(value)
    } else {
        None
    }
}

fn hotkey_from_values(
    language: Option<u32>,
    layout: Option<u32>,
    legacy: Option<u32>,
) -> LayoutHotkey {
    if language.is_some() || layout.is_some() {
        let mut disabled = false;

        for value in [language, layout].into_iter().flatten() {
            match value {
                1 => return LayoutHotkey::AltShift,
                2 => return LayoutHotkey::CtrlShift,
                3 => disabled = true,
                4 => return LayoutHotkey::Grave,
                _ => {}
            }
        }

        if disabled {
            // Win+Space is always available on Windows 10/11.
            return LayoutHotkey::WinSpace;
        }

        return LayoutHotkey::AltShift;
    }

    match legacy {
        Some(1) => LayoutHotkey::AltShift,
        Some(2) => LayoutHotkey::CtrlShift,
        Some(3) => LayoutHotkey::WinSpace,
        Some(4) => LayoutHotkey::Grave,
        _ => LayoutHotkey::AltShift,
    }
}

unsafe fn read_layout_hotkey() -> LayoutHotkey {
    // HKCU\Keyboard Layout\Toggle: 1 = Alt+Shift, 2 = Ctrl+Shift,
    // 3 = disabled, 4 = Grave accent.
    let language = unsafe { read_toggle_value(w!("Language Hotkey")) };
    let layout = unsafe { read_toggle_value(w!("Layout Hotkey")) };

    // The legacy value is only used when the modern values are absent;
    // otherwise it could override an explicit "disabled" setting.
    let legacy = if language.is_none() && layout.is_none() {
        unsafe { read_toggle_value(w!("Hotkey")) }
    } else {
        None
    };

    hotkey_from_values(language, layout, legacy)
}

fn send_combo(modifier: VIRTUAL_KEY, key: VIRTUAL_KEY) {
    let inputs = [
        make_input(modifier, KEYBD_EVENT_FLAGS(0)),
        make_input(key, KEYBD_EVENT_FLAGS(0)),
        make_input(key, KEYEVENTF_KEYUP),
        make_input(modifier, KEYEVENTF_KEYUP),
    ];

    unsafe {
        let _ = SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}

fn send_key(key: VIRTUAL_KEY) {
    let inputs = [
        make_input(key, KEYBD_EVENT_FLAGS(0)),
        make_input(key, KEYEVENTF_KEYUP),
    ];

    unsafe {
        let _ = SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}

fn switch_layout(hotkey: LayoutHotkey) {
    match hotkey {
        LayoutHotkey::AltShift => send_combo(VK_LMENU, VK_LSHIFT),
        LayoutHotkey::CtrlShift => send_combo(VK_LCONTROL, VK_LSHIFT),
        LayoutHotkey::Grave => send_key(VK_OEM_3),
        LayoutHotkey::WinSpace => send_combo(VK_LWIN, VK_SPACE),
    }
}

unsafe extern "system" fn keyboard_hook(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code == HC_ACTION as i32 {
        let kb_struct = unsafe { &*(l_param.0 as *const KBDLLHOOKSTRUCT) };

        if kb_struct.vkCode == VK_CAPITAL.0 as u32 {
            
            if w_param.0 == WM_KEYDOWN as usize || w_param.0 == WM_SYSKEYDOWN as usize {

                let shift_pressed = unsafe { (GetKeyState(VK_SHIFT.0 as i32) as u16 & 0x8000) != 0 };

                if shift_pressed {
                    CAPS_WAS_BLOCKED.store(false, Ordering::SeqCst);
                } else {
                    CAPS_WAS_BLOCKED.store(true, Ordering::SeqCst);
                    NEED_SWITCH.store(true, Ordering::SeqCst);
                    
                    unsafe {
                        let _ = PostThreadMessageW(GetCurrentThreadId(), WM_SWITCH_LAYOUT, WPARAM(0), LPARAM(0));
                    }
                    
                    return LRESULT(1);
                }
            } 
            else if w_param.0 == WM_KEYUP as usize || w_param.0 == WM_SYSKEYUP as usize {
                if CAPS_WAS_BLOCKED.load(Ordering::SeqCst) {
                    CAPS_WAS_BLOCKED.store(false, Ordering::SeqCst);
                    return LRESULT(1);
                }
            }
        }
    }
   
    unsafe { CallNextHookEx(None, n_code, w_param, l_param) }
}

fn main() -> windows::core::Result<()> {
    unsafe {
        let _instance_guard = CreateMutexW(None, true, w!("Local\\CapslockSwitcher"))?;
        let already_running = match GetLastError() {
            Err(err) => err.code() == ERROR_ALREADY_EXISTS.to_hresult(),
            Ok(()) => false,
        };
        if already_running {
            return Ok(());
        }

        let layout_hotkey = read_layout_hotkey();

        let h_instance = GetModuleHandleW(None)?;
        let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook), h_instance, 0)?;

        let mut msg = MSG::default();

        PeekMessageW(&mut msg, None, 0, 0, PM_NOREMOVE);
        
        while GetMessageW(&mut msg, None, 0, 0).into() {
            if msg.message == WM_SWITCH_LAYOUT && NEED_SWITCH.swap(false, Ordering::SeqCst) {
                switch_layout(layout_hotkey);
            } else {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }

        UnhookWindowsHookEx(hook)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{hotkey_from_values, LayoutHotkey};

    #[test]
    fn maps_modern_hotkey_values() {
        assert_eq!(
            hotkey_from_values(Some(1), None, None),
            LayoutHotkey::AltShift
        );
        assert_eq!(
            hotkey_from_values(Some(2), None, None),
            LayoutHotkey::CtrlShift
        );
        assert_eq!(
            hotkey_from_values(Some(4), None, None),
            LayoutHotkey::Grave
        );
    }

    #[test]
    fn uses_win_space_when_modern_values_are_disabled() {
        assert_eq!(
            hotkey_from_values(Some(3), Some(3), Some(1)),
            LayoutHotkey::WinSpace
        );
        assert_eq!(
            hotkey_from_values(None, Some(3), None),
            LayoutHotkey::WinSpace
        );
    }

    #[test]
    fn prefers_an_assigned_modern_value() {
        assert_eq!(
            hotkey_from_values(Some(3), Some(2), Some(1)),
            LayoutHotkey::CtrlShift
        );
        assert_eq!(
            hotkey_from_values(Some(3), Some(4), None),
            LayoutHotkey::Grave
        );
    }

    #[test]
    fn maps_legacy_values_and_falls_back_to_alt_shift() {
        assert_eq!(
            hotkey_from_values(None, None, Some(1)),
            LayoutHotkey::AltShift
        );
        assert_eq!(
            hotkey_from_values(None, None, Some(2)),
            LayoutHotkey::CtrlShift
        );
        assert_eq!(
            hotkey_from_values(None, None, Some(3)),
            LayoutHotkey::WinSpace
        );
        assert_eq!(
            hotkey_from_values(None, None, Some(4)),
            LayoutHotkey::Grave
        );
        assert_eq!(
            hotkey_from_values(None, None, None),
            LayoutHotkey::AltShift
        );
        assert_eq!(
            hotkey_from_values(Some(99), None, None),
            LayoutHotkey::AltShift
        );
    }
}
