use std::ffi::OsString;
use std::io::{self, Write};
use std::os::windows::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use regex::Regex;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowTextLengthW, GetWindowTextW, IsWindowVisible, PostMessageW,
    WM_LBUTTONDOWN, WM_LBUTTONUP,
};

fn get_window_text(hwnd: HWND) -> Option<String> {
    unsafe {
        let len = GetWindowTextLengthW(hwnd);
        if len == 0 {
            return None;
        }
        let mut buf: Vec<u16> = vec![0u16; (len + 1) as usize];
        let read = GetWindowTextW(hwnd, &mut buf);
        if read == 0 {
            return None;
        }
        OsString::from_wide(&buf[..read as usize])
            .into_string()
            .ok()
    }
}

fn find_minecraft_window() -> Option<HWND> {
    let found_window: Arc<std::sync::Mutex<Option<HWND>>> =
        Arc::new(std::sync::Mutex::new(None));
    let found_window_clone = found_window.clone();
    let re = Regex::new(r"(?i)^minecraft\s+\d+(\.\d+)*").unwrap();

    unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let ptr = lparam.0 as *mut (Arc<std::sync::Mutex<Option<HWND>>>, Regex);
        if ptr.is_null() {
            return BOOL(1);
        }
        let (arc, re) = &*ptr;

        if IsWindowVisible(hwnd).as_bool() {
            if let Some(title) = get_window_text(hwnd) {
                if re.is_match(title.trim()) {
                    let mut found = arc.lock().unwrap();
                    *found = Some(hwnd);
                    return BOOL(0);
                }
            }
        }

        BOOL(1)
    }

    let pair = (found_window_clone.clone(), re);
    let ptr = &pair as *const _ as isize;

    unsafe {
        EnumWindows(Some(enum_proc), LPARAM(ptr));
    }

    let result = found_window.lock().unwrap().clone();
    result
}

fn post_left_click(hwnd: HWND) {
    unsafe {
        let _ = PostMessageW(hwnd, WM_LBUTTONDOWN, WPARAM(0), LPARAM(0));
        thread::sleep(Duration::from_millis(50));
        let _ = PostMessageW(hwnd, WM_LBUTTONUP, WPARAM(0), LPARAM(0));
    }
}

fn main() {
    let hwnd = match find_minecraft_window() {
        Some(h) => {
            println!("Minecraft window detected.");
            h
        }
        None => {
            eprintln!("Minecraft window not found.");
            std::process::exit(1);
        }
    };

    let interval_secs: f64 = loop {
        print!("Enter click interval in seconds (example: 0.5): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<f64>() {
            Ok(v) if v > 0.0 => break v,
            _ => {
                println!("Invalid number, try again.");
                continue;
            }
        }
    };

    let running = Arc::new(AtomicBool::new(true));
    {
        let running = running.clone();
        ctrlc::set_handler(move || {
            running.store(false, Ordering::SeqCst);
        })
            .expect("Failed to set Ctrl-C handler");
    }

    println!("Starting in 5 seconds...");
    thread::sleep(Duration::from_secs(5));

    println!(
        "Auto-clicking every {:.2} seconds. Press Ctrl+C to stop.",
        interval_secs
    );

    while running.load(Ordering::SeqCst) {
        post_left_click(hwnd);
        thread::sleep(Duration::from_millis((interval_secs * 1000.0) as u64));
    }

    println!("Stopped.");
}
