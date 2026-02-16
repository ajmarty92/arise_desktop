#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WebviewWindow};
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM, BOOL};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, FindWindowA, FindWindowExA, SendMessageTimeoutA, SetParent, 
    SMTO_NORMAL
};

// --- WorkerW Hack Implementation ---
#[tauri::command]
fn attach_to_desktop(window: WebviewWindow) {
    // 1. Get the handle from Tauri
    let tauri_handle = window.hwnd().expect("Failed to get window handle");

    // 2. FORCE CAST the handle
    // Convert Tauri's handle to our local HWND type safely using transmute
    // This bypasses the "Type Mismatch" error between crate versions
    let window_hwnd: HWND = unsafe { std::mem::transmute(tauri_handle) };

    unsafe {
        // 3. Find Progman
        // In windows 0.58, FindWindowA returns HWND (no Result/Option wrapper)
        let progman = FindWindowA(
            windows::core::PCSTR("Progman\0".as_ptr()), 
            windows::core::PCSTR::null()
        );

        // 4. Send 0x052C to Progman to spawn the WorkerW
        let _ = SendMessageTimeoutA(
            progman, 
            0x052C, 
            WPARAM(0), 
            LPARAM(0), 
            SMTO_NORMAL, 
            1000, 
            None
        );

        // 5. Find the WorkerW sibling of SHELLDLL_DefView
        let mut worker_w: HWND = HWND(0);
        
        unsafe extern "system" fn enum_window_callback(handle: HWND, lparam: LPARAM) -> BOOL {
            let p_worker_w = lparam.0 as *mut HWND;
            
            // In 0.58, FindWindowExA takes HWND directly. Use HWND(0) for NULL.
            let shell_dll = FindWindowExA(
                handle, 
                HWND(0), 
                windows::core::PCSTR("SHELLDLL_DefView\0".as_ptr()), 
                windows::core::PCSTR::null()
            );

            if shell_dll.0 != 0 {
                let worker = FindWindowExA(
                    HWND(0), 
                    handle, 
                    windows::core::PCSTR("WorkerW\0".as_ptr()), 
                    windows::core::PCSTR::null()
                );

                if worker.0 != 0 {
                     *p_worker_w = worker;
                }
                return BOOL(0); // Stop enumerating
            }
            BOOL(1) // Continue enumerating
        }

        // Pass the pointer to our worker_w variable into the callback
        let _ = EnumWindows(Some(enum_window_callback), LPARAM(&mut worker_w as *mut _ as isize));

        // 6. Glue our window to the WorkerW
        if worker_w.0 != 0 {
            let _ = SetParent(window_hwnd, worker_w);
        }
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![attach_to_desktop])
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            attach_to_desktop(main_window);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}