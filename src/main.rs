#![windows_subsystem = "windows"]

use std::sync::atomic::{AtomicBool, Ordering};
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::*;
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

fn simulate_alt_shift() {
    let inputs = [
        make_input(VK_MENU, KEYBD_EVENT_FLAGS(0)),
        make_input(VK_SHIFT, KEYBD_EVENT_FLAGS(0)),
        make_input(VK_SHIFT, KEYEVENTF_KEYUP),
        make_input(VK_MENU, KEYEVENTF_KEYUP),
    ];
    
    unsafe {
        let _ = SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
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
        let h_instance = GetModuleHandleW(None)?;
        let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook), h_instance, 0)?;

        let mut msg = MSG::default();

        PeekMessageW(&mut msg, None, 0, 0, PM_NOREMOVE);
        
        while GetMessageW(&mut msg, None, 0, 0).into() {
            if msg.message == WM_SWITCH_LAYOUT && NEED_SWITCH.swap(false, Ordering::SeqCst) {
                simulate_alt_shift();
            } else {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }

        UnhookWindowsHookEx(hook)?;
    }
    Ok(())
}
