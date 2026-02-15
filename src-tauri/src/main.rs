#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Window};
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, FindWindowA, FindWindowExA, SendMessageTimeoutA, SetParent, 
    SMTO_NORMAL
};

// --- WorkerW Hack Implementation ---
// This function finds the hidden "WorkerW" window behind desktop icons 
// and glues your app window to it.
#[tauri::command]
fn attach_to_desktop(window: Window) {
    let window_handle = window.hwnd().unwrap().0;

    unsafe {
        // 1. Ask Windows Program Manager (Progman) to spawn a WorkerW
        let progman = FindWindowA(windows::core::PCSTR("Progman\0".as_ptr()), windows::core::PCSTR::null());
        SendMessageTimeoutA(progman, 0x052C, WPARAM(0), LPARAM(0), SMTO_NORMAL, 1000, None);

        // 2. Hunt for the WorkerW that is actually behind the icons
        // (It is the sibling of SHELLDLL_DefView)
        let mut worker_w: HWND = HWND(0);
        
        unsafe extern "system" fn enum_window_callback(handle: HWND, lparam: LPARAM) -> BOOL {
            let p_worker_w = lparam.0 as *mut HWND;
            let shell_dll = FindWindowExA(handle, HWND(0), windows::core::PCSTR("SHELLDLL_DefView\0".as_ptr()), windows::core::PCSTR::null());
            
            if shell_dll.0 != 0 {
                *p_worker_w = FindWindowExA(HWND(0), handle, windows::core::PCSTR("WorkerW\0".as_ptr()), windows::core::PCSTR::null());
                return BOOL(0); // Found it, stop searching
            }
            BOOL(1) // Keep searching
        }
        EnumWindows(Some(enum_window_callback), LPARAM(&mut worker_w as *mut _ as isize));

        // 3. Glue our window to the WorkerW
        if worker_w.0 != 0 {
            SetParent(HWND(window_handle as isize), worker_w);
        }
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![attach_to_desktop])
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            
            // Automatically attach to desktop on launch
            // We verify if we are in development or production to avoid weird behavior during debugging
            #[cfg(not(debug_assertions))] 
            attach_to_desktop(main_window);

            // In dev mode, we might want to keep it floating to see errors, 
            // but you can uncomment the line below to test the hack in dev:
            // attach_to_desktop(main_window);
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}