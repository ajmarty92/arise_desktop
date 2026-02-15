#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WebviewWindow};
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, FindWindowA, FindWindowExA, SendMessageTimeoutA, SetParent, 
    SMTO_NORMAL
};

// --- WorkerW Hack Implementation ---
#[tauri::command]
fn attach_to_desktop(window: WebviewWindow) {
    let window_hwnd = window.hwnd().expect("Failed to get window handle");

    unsafe {
        let progman = FindWindowA(
            windows::core::PCSTR("Progman\0".as_ptr()), 
            windows::core::PCSTR::null()
        ).unwrap_or(HWND(std::ptr::null_mut()));

        let _ = SendMessageTimeoutA(
            progman, 
            0x052C, 
            WPARAM(0), 
            LPARAM(0), 
            SMTO_NORMAL, 
            1000, 
            None
        );

        let mut worker_w: HWND = HWND(std::ptr::null_mut());
        
        unsafe extern "system" fn enum_window_callback(handle: HWND, lparam: LPARAM) -> BOOL {
            let p_worker_w = lparam.0 as *mut HWND;
            
            let shell_dll = FindWindowExA(
                handle, 
                HWND(std::ptr::null_mut()), 
                windows::core::PCSTR("SHELLDLL_DefView\0".as_ptr()), 
                windows::core::PCSTR::null()
            );

            if let Ok(shell_handle) = shell_dll {
                if shell_handle.0 != std::ptr::null_mut() {
                    let worker = FindWindowExA(
                        HWND(std::ptr::null_mut()), 
                        handle, 
                        windows::core::PCSTR("WorkerW\0".as_ptr()), 
                        windows::core::PCSTR::null()
                    );

                    if let Ok(w_handle) = worker {
                         *p_worker_w = w_handle;
                    }
                    return BOOL(0);
                }
            }
            BOOL(1)
        }

        EnumWindows(Some(enum_window_callback), LPARAM(&mut worker_w as *mut _ as isize));

        if worker_w.0 != std::ptr::null_mut() {
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