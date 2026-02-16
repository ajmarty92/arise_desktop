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
    // Convert Tauri's handle to our local HWND type safely
    let window_hwnd: HWND = unsafe { std::mem::transmute(tauri_handle) };

    unsafe {
        // 3. Find Progman
        // FindWindowA returns HWND directly (not Option)
        let progman = FindWindowA(
            windows::core::PCSTR("Progman\0".as_ptr()), 
            windows::core::PCSTR::null()
        );

        // 4. Send 0x052C to Progman to spawn the WorkerW
        // SendMessageTimeoutA takes HWND directly
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
        let mut worker_w: HWND = HWND(std::ptr::null_mut());
        
        unsafe extern "system" fn enum_window_callback(handle: HWND, lparam: LPARAM) -> BOOL {
            let p_worker_w = lparam.0 as *mut HWND;
            
            // FindWindowExA requires Option<HWND> for parent and child_after
            // We pass Some(handle) as parent, and None (null) as child_after
            let shell_dll = FindWindowExA(
                Some(handle), 
                None, 
                windows::core::PCSTR("SHELLDLL_DefView\0".as_ptr()), 
                windows::core::PCSTR::null()
            );

            // If we found SHELLDLL_DefView inside this handle
            if shell_dll.0 != std::ptr::null_mut() {
                // Look for the WorkerW sibling
                let worker = FindWindowExA(
                    None, 
                    Some(handle), 
                    windows::core::PCSTR("WorkerW\0".as_ptr()), 
                    windows::core::PCSTR::null()
                );

                if worker.0 != std::ptr::null_mut() {
                     *p_worker_w = worker;
                }
                return BOOL(0); // Stop enumerating (False)
            }
            BOOL(1) // Continue enumerating (True)
        }

        // Pass the pointer to our worker_w variable into the callback
        let _ = EnumWindows(Some(enum_window_callback), LPARAM(&mut worker_w as *mut _ as isize));

        // 6. Glue our window to the WorkerW
        // SetParent requires Option<HWND> for the new parent
        if worker_w.0 != std::ptr::null_mut() {
            let _ = SetParent(window_hwnd, Some(worker_w));
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