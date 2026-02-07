use tauri::{Manager, Runtime, WebviewWindow};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Shortcut, ShortcutState};

// 禁用 Windows 窗口动画
#[cfg(target_os = "windows")]
fn disable_window_transitions<R: Runtime>(window: &WebviewWindow<R>) {
    use windows_sys::Win32::Graphics::Dwm::{DwmSetWindowAttribute, DWMWA_TRANSITIONS_FORCEDISABLED};
    
    if let Ok(hwnd) = window.hwnd() {
        let is_off: i32 = 1; // 1 代表 TRUE (关闭动画)
        unsafe {
            let _ = DwmSetWindowAttribute(
                hwnd.0 as _, // 自动转换为 windows-sys 期望的指针类型
                DWMWA_TRANSITIONS_FORCEDISABLED as u32, // 显式转为 u32
                &is_off as *const i32 as *const _,
                std::mem::size_of::<i32>() as u32,
            );
        }
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let window = app.get_webview_window("main").expect("找不到主窗口");

            // 只有 Windows 下执行禁用动画逻辑
            #[cfg(target_os = "windows")]
            disable_window_transitions(&window);

            // 注册热键
            let f2 = Shortcut::new(None, Code::F2);

            // 监听热键逻辑
            app.global_shortcut().on_shortcut(f2, move |app_handle: &tauri::AppHandle, _shortcut, event| {
                if event.state() == ShortcutState::Pressed {
                    if let Some(main_win) = app_handle.get_webview_window("main") {
                        let is_visible = main_win.is_visible().unwrap_or(false);
                        if is_visible {
                            let _ = main_win.hide();
                        } else {
                            let _ = main_win.show();
                            let _ = main_win.set_focus();
                        }
                    }
                }
            }).expect("注册热键监听失败");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("应用运行失败");
}