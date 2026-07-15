#![windows_subsystem = "windows"]

use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;

unsafe fn simulate_alt_shift() {
    let inputs = [
        INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VK_MENU, wScan: 0, dwFlags: KEYBD_EVENT_FLAGS(0), time: 0, dwExtraInfo: 0 } } },
        INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VK_SHIFT, wScan: 0, dwFlags: KEYBD_EVENT_FLAGS(0), time: 0, dwExtraInfo: 0 } } },
        INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VK_SHIFT, wScan: 0, dwFlags: KEYEVENTF_KEYUP, time: 0, dwExtraInfo: 0 } } },
        INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VK_MENU, wScan: 0, dwFlags: KEYEVENTF_KEYUP, time: 0, dwExtraInfo: 0 } } },
    ];
    
    unsafe {
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}

unsafe extern "system" fn keyboard_hook(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code == HC_ACTION as i32 {
        let kb_struct = unsafe { &*(l_param.0 as *const KBDLLHOOKSTRUCT) };
        
        if kb_struct.vkCode == VK_CAPITAL.0 as u32 {
            if w_param.0 == WM_KEYDOWN as usize || w_param.0 == WM_SYSKEYDOWN as usize {
                // Проверяем, нажат ли Shift в данный момент
                let shift_pressed = unsafe { 
		    (GetKeyState(VK_SHIFT.0 as i32) as u16 & 0x8000u16) != 0
                };
                
                if shift_pressed {
                    // Shift + CapsLock: пропускаем событие дальше
                    // Windows сам включит/выключит Caps Lock как обычно
                    return unsafe { CallNextHookEx(None, n_code, w_param, l_param) };
                } else {
                    // Просто CapsLock: переключаем раскладку
                    unsafe {
                        simulate_alt_shift();
                    }
                    return LRESULT(1); 
                }
            }
        }
    }
    
    unsafe {
        CallNextHookEx(None, n_code, w_param, l_param)
    }
}

fn main() -> windows::core::Result<()> {
    unsafe {
        let h_instance = GetModuleHandleW(None)?;
        let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook), h_instance, 0)?;

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        UnhookWindowsHookEx(hook)?;
    }
    Ok(())
}
