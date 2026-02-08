use tauri::{AppHandle, Manager, Runtime, WebviewWindow, command, State};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Shortcut, ShortcutState};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

// --- Data Structure ---

#[derive(Serialize, Deserialize, Clone)] 
struct Config {
    deepseek: DeepSeekConfig,
}

#[derive(Serialize, Deserialize, Clone)]
struct DeepSeekConfig {
    api_key: String,
    api_url: String,
    model: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

// --- Logic Helper Function ---

fn load_config() -> Result<Config, String> {
    let paths = ["./src-tauri/config.json", "config.json"];
    let mut config_str = String::new();
    
    for path in paths {
        if let Ok(content) = fs::read_to_string(Path::new(path)) {
            config_str = content;
            break;
        }
    }

    if config_str.is_empty() {
        return Err("找不到 config.json。请确保文件存在于 src-tauri 目录下。".to_string());
    }
    
    serde_json::from_str(&config_str).map_err(|e| format!("解析配置文件失败: {}", e))
}

// --- Tauri Commands ---

#[command]
async fn call_deepseek(term: String, mode: String, config: State<'_, Config>) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;
    
    let prompt = match mode.as_str() {
        "tr" => format!(
            "你是一个翻译人员，请对“{}”进行翻译。如果内容是中（英）文，请翻译为英（中）文。只返回翻译结果，不要添加任何解释或多余文本。",
            term
        ),
        "m" => format!(
            "你是一位技术专家。请用最简短直白的方式解释“{}”并举例。如果是技术名词，请说明常见注意事项。严禁使用列表或分点符号，必须以纯段落文字形式返回。",
            term
        ),
        "py" => format!(
            "你是一个软件工程师。请只提供一个在 Python 中实现“{}”时最常用、最标准的类或函数。只给出：名称、主要参数、返回值，以及一个最小可用示例，不要提供其他方案或额外解释。",
            term
        ),
        _ => format!(
            "你是一个软件工程师。请只提供一个在 C++ 中实现“{}”时最常用、最标准的类或函数。只给出：名称、主要参数、返回值，以及一个最小可用示例，不要提供其他方案或额外解释。",
            term
        )
    };

    let res = client
        .post(&config.deepseek.api_url)
        .header("Authorization", format!("Bearer {}", config.deepseek.api_key))
        .json(&ChatRequest {
            model: config.deepseek.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
        })
        .send()
        .await
        .map_err(|e| format!("网络请求失败: {}", e))?;

    let data: ChatResponse = res.json().await.map_err(|e| format!("解析返回内容失败: {}", e))?;
    
    if data.choices.is_empty() {
        return Err("AI 未返回任何有效内容".to_string());
    }

    Ok(data.choices[0].message.content.clone())
}

// --- Window Optimization---

#[cfg(target_os = "windows")]
fn disable_window_transitions<R: Runtime>(window: &WebviewWindow<R>) {
    use windows_sys::Win32::Graphics::Dwm::{DwmSetWindowAttribute, DWMWA_TRANSITIONS_FORCEDISABLED};
    
    if let Ok(hwnd) = window.hwnd() {
        let is_off: i32 = 1; 
        unsafe {
            let _ = DwmSetWindowAttribute(
                hwnd.0 as _, 
                DWMWA_TRANSITIONS_FORCEDISABLED as u32, 
                &is_off as *const i32 as *const _,
                std::mem::size_of::<i32>() as u32,
            );
        }
    }
}

// --- main entry ---

pub fn run() {
    let config = match load_config() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("启动失败: {}", e);
            return;
        }
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(config)
        .invoke_handler(tauri::generate_handler![call_deepseek])
        .setup(|app| {
            let window = app.get_webview_window("main").expect("找不到主窗口");

            #[cfg(target_os = "windows")]
            disable_window_transitions(&window);

            let f2 = Shortcut::new(None, Code::F2);

            app.global_shortcut().on_shortcut(f2, move |app_handle: &AppHandle, _shortcut, event| {
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
            }).expect("绑定热键监听器失败");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("应用运行失败");
}