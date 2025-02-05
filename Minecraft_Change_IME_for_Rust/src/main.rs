use std::thread;
use std::time::Duration;
use std::ptr;
use windows::Win32::Foundation::{HWND};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowA, GetForegroundWindow, GetWindowTextA};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_KEYBOARD, KEYBDINPUT, VIRTUAL_KEY, KEYEVENTF_KEYUP, SendInput
};
use muda::{MenuItem, TrayIconBuilder};

fn send_key_combo(key1: u16, key2: u16, key3: u16) {
    let inputs = [
        INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VIRTUAL_KEY(key1), ..Default::default() } } },
        INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VIRTUAL_KEY(key2), ..Default::default() } } },
        INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VIRTUAL_KEY(key3), ..Default::default() } } },
        INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VIRTUAL_KEY(key3), dwFlags: KEYEVENTF_KEYUP, ..Default::default() } } },
        INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VIRTUAL_KEY(key2), dwFlags: KEYEVENTF_KEYUP, ..Default::default() } } },
        INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: VIRTUAL_KEY(key1), dwFlags: KEYEVENTF_KEYUP, ..Default::default() } } },
    ];
    unsafe {
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}

fn main() {
    let exit_item = MenuItem::new("Exit", true, None);
    
    let _tray_icon = TrayIconBuilder::new()
        .with_tooltip("Minecraft IME Switcher")
        .build()
        .expect("Failed to create tray icon");

    let minecraft_window_name = "Minecraft"; // ここは適宜修正
    let minecraft_hwnd = unsafe { FindWindowA(None, minecraft_window_name) };

    loop {
        let foreground_hwnd = unsafe { GetForegroundWindow() };

        if foreground_hwnd == minecraft_hwnd {
            println!("Minecraft is active. Switching IME...");
            send_key_combo(0x12, 0x10, 0x32); // Alt+Shift+2
        } else {
            println!("Minecraft is inactive. Reverting IME...");
            send_key_combo(0x12, 0x10, 0x31); // Alt+Shift+1
        }

        thread::sleep(Duration::from_secs(1));
    }
}