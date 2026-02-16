#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WebviewWindow};
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
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
        // FindWindowA returns Result<HWND>. We unwrap it, defaulting to null if it fails.
        let progman = FindWindowA(
            windows::core::PCSTR("Progman\0".as_ptr()), 
            windows::core::PCSTR::null()
        ).unwrap_or(HWND(std::ptr::null_mut()));

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
        let mut worker_w: HWND = HWND(std::ptr::null_mut());
        
        unsafe extern "system" fn enum_window_callback(handle: HWND, lparam: LPARAM) -> windows::Win32::Foundation::BOOL {
            let p_worker_w = lparam.0 as *mut HWND;
            
            // FindWindowExA returns Result<HWND>
            let shell_dll = FindWindowExA(
                Some(handle), 
                None, 
                windows::core::PCSTR("SHELLDLL_DefView\0".as_ptr()), 
                windows::core::PCSTR::null()
            );

            // Check if the Result is Ok and the handle inside is valid
            if let Ok(shell_handle) = shell_dll {
                if shell_handle.0 != std::ptr::null_mut() {
                    // Look for the WorkerW sibling
                    let worker_res = FindWindowExA(
                        None, 
                        Some(handle), 
                        windows::core::PCSTR("WorkerW\0".as_ptr()), 
                        windows::core::PCSTR::null()
                    );

                    if let Ok(worker_handle) = worker_res {
                        if worker_handle.0 != std::ptr::null_mut() {
                             *p_worker_w = worker_handle;
                        }
                    }
                    return windows::Win32::Foundation::BOOL(0); // Stop enumerating (False)
                }
            }
            windows::Win32::Foundation::BOOL(1) // Continue enumerating (True)
        }

        // Pass the pointer to our worker_w variable into the callback
        let _ = EnumWindows(Some(enum_window_callback), LPARAM(&mut worker_w as *mut _ as isize));

        // 6. Glue our window to the WorkerW
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