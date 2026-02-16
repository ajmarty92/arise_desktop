#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WebviewWindow};
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, FindWindowA, FindWindowExA, SendMessageTimeoutA, SetParent, 
    SMTO_NORMAL
};

#[tauri::command]
fn attach_to_desktop(window: WebviewWindow) {
    let tauri_handle = window.hwnd().expect("Failed to get window handle");
    
    // Safety cast: Ensures the HWND type matches exactly, regardless of minor version mismatches
    let window_hwnd: HWND = unsafe { std::mem::transmute(tauri_handle) };

    unsafe {
        // 1. Find Progman
        // In 0.52.0, FindWindowA returns Result<HWND>.
        // We use .unwrap_or to handle failure gracefully.
        let progman = FindWindowA(
            windows::core::PCSTR("Progman\0".as_ptr()), 
            windows::core::PCSTR::null()
        ).unwrap_or(HWND(0));

        // 2. Spawn WorkerW
        let _ = SendMessageTimeoutA(
            progman, 
            0x052C, 
            WPARAM(0), 
            LPARAM(0), 
            SMTO_NORMAL, 
            1000, 
            None
        );

        // 3. Find the correct WorkerW
        // In 0.52.0, HWND wraps an isize/pointer, HWND(0) is the null handle.
        let mut worker_w: HWND = HWND(0);
        
        unsafe extern "system" fn enum_window_callback(handle: HWND, lparam: LPARAM) -> BOOL {
            let p_worker_w = lparam.0 as *mut HWND;
            
            // Search for SHELLDLL_DefView
            let shell_dll = FindWindowExA(
                handle, 
                HWND(0), 
                windows::core::PCSTR("SHELLDLL_DefView\0".as_ptr()), 
                windows::core::PCSTR::null()
            );

            // Check if Result is Ok and handle is not 0
            if let Ok(shell_handle) = shell_dll {
                if shell_handle.0 != 0 {
                    // Look for the WorkerW sibling
                    let worker = FindWindowExA(
                        HWND(0), 
                        handle, 
                        windows::core::PCSTR("WorkerW\0".as_ptr()), 
                        windows::core::PCSTR::null()
                    );

                    if let Ok(w) = worker {
                        if w.0 != 0 {
                             *p_worker_w = w;
                        }
                    }
                    return BOOL(0); // Stop enumerating (False)
                }
            }
            BOOL(1) // Continue enumerating (True)
        }

        let _ = EnumWindows(Some(enum_window_callback), LPARAM(&mut worker_w as *mut _ as isize));

        // 4. Glue window
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