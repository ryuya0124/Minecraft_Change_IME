use std::{thread, time::Duration};
use windows::Win32::{
    UI::{
        Input::KeyboardAndMouse::{
            INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, SendInput, VIRTUAL_KEY, VK_MENU, VK_LSHIFT
        },
        WindowsAndMessaging::{GetForegroundWindow, GetWindowTextA},
    },
};

const TARGET_WINDOW_TITLE: &str = "Minecraft";
const CHECK_INTERVAL: Duration = Duration::from_millis(100);

fn send_key_combination(key1: VIRTUAL_KEY, key2: VIRTUAL_KEY, key3: VIRTUAL_KEY) {
    let inputs = vec![
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: key1, ..Default::default() } },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: key2, ..Default::default() } },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: key3, ..Default::default() } },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: key3, dwFlags: KEYEVENTF_KEYUP, ..Default::default() } },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: key2, dwFlags: KEYEVENTF_KEYUP, ..Default::default() } },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: key1, dwFlags: KEYEVENTF_KEYUP, ..Default::default() } },
        },
    ];
    unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32); }
}

fn get_foreground_window_title() -> Option<String> {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0.is_null() { 
            return None;
        }
        let mut buffer = [0u8; 256];
        let len = GetWindowTextA(hwnd, &mut buffer);
        if len > 0 {
            Some(String::from_utf8_lossy(&buffer[..len as usize]).to_string())
        } else {
            None
        }
    }
}

fn main() {
    let mut was_active = false;
    loop {
        if let Some(title) = get_foreground_window_title() {
            if title == TARGET_WINDOW_TITLE {
                if !was_active {
                    send_key_combination(VK_MENU, VK_LSHIFT, VIRTUAL_KEY(0x32)); // Alt + Shift + 2
                    was_active = true;
                }
            } else {
                if was_active {
                    send_key_combination(VK_MENU, VK_LSHIFT, VIRTUAL_KEY(0x31)); // Alt + Shift + 1
                    was_active = false;
                }
            }
        }
        thread::sleep(CHECK_INTERVAL);
    }
}
