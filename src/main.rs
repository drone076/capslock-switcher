#![windows_subsystem = "windows"]

use std::sync::atomic::{AtomicBool, Ordering};
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::System::Threading::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;

// Кастомное сообщение для нашего главного цикла
const WM_SWITCH_LAYOUT: u32 = WM_APP + 1;

// Флаги состояния
static NEED_SWITCH: AtomicBool = AtomicBool::new(false);
static CAPS_WAS_BLOCKED: AtomicBool = AtomicBool::new(false);

/// Вспомогательная функция для чистоты кода
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

/// Сама симуляция Alt+Shift (теперь вызывается из главного потока, а не из хука!)
unsafe fn simulate_alt_shift() {
    let inputs = [
        make_input(VK_MENU, KEYBD_EVENT_FLAGS(0)),
        make_input(VK_SHIFT, KEYBD_EVENT_FLAGS(0)),
        make_input(VK_SHIFT, KEYEVENTF_KEYUP),
        make_input(VK_MENU, KEYEVENTF_KEYUP),
    ];
    let _ = SendInput(&inputs);
}

unsafe extern "system" fn keyboard_hook(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code == HC_ACTION as i32 {
        let kb_struct = unsafe { &*(l_param.0 as *const KBDLLHOOKSTRUCT) };

        if kb_struct.vkCode == VK_CAPITAL.0 as u32 {
            
            // 1. Обработка нажатия (KeyDown)
            if w_param.0 == WM_KEYDOWN as usize || w_param.0 == WM_SYSKEYDOWN as usize {
                // Игнорируем повторы (зажатие клавиши)
                let is_down = (GetKeyState(VK_CAPITAL.0 as i32) as u16 & 0x8000) != 0;
                if is_down {
                    return LRESULT(1); // Блокируем повтор
                }

                let shift_pressed = (GetKeyState(VK_SHIFT.0 as i32) as u16 & 0x8000) != 0;

                if shift_pressed {
                    // Shift + CapsLock = обычное поведение CapsLock
                    CAPS_WAS_BLOCKED.store(false, Ordering::SeqCst);
                    // Пропускаем дальше
                } else {
                    // CapsLock без Shift = переключаем раскладку
                    CAPS_WAS_BLOCKED.store(true, Ordering::SeqCst);
                    NEED_SWITCH.store(true, Ordering::SeqCst);
                    
                    // МГНОВЕННО отправляем сообщение в главный поток и возвращаем управление!
                    // Это защищает хук от таймаута Windows.
                    let _ = PostThreadMessageW(GetCurrentThreadId(), WM_SWITCH_LAYOUT, WPARAM(0), LPARAM(0));
                    
                    return LRESULT(1); // Блокируем оригинальный CapsLock
                }
            } 
            // 2. Обработка отпускания (KeyUp)
            else if w_param.0 == WM_KEYUP as usize || w_param.0 == WM_SYSKEYUP as usize {
                if CAPS_WAS_BLOCKED.load(Ordering::SeqCst) {
                    CAPS_WAS_BLOCKED.store(false, Ordering::SeqCst);
                    return LRESULT(1); // Блокируем KeyUp, если мы заблокировали KeyDown
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
        
        // Главный цикл сообщений
        while GetMessageW(&mut msg, None, 0, 0).into() {
            // Если пришло наше кастомное сообщение — делаем симуляцию
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
