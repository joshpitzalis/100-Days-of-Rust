use tauri::{AppHandle, Manager, WebviewWindowBuilder};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(target_os = "macos")]
    let summon = Shortcut::new(Some(Modifiers::META | Modifiers::SHIFT), Code::Minus);

    #[cfg(any(target_os = "windows", target_os = "linux"))]
    let summon = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::Minus);

    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if shortcut == &summon {
                        match event.state() {
                            ShortcutState::Pressed => {
                                for (_, webview) in app.webview_windows().iter() {
                                    webview.set_focus().ok();
                                }
                            }
                            ShortcutState::Released => (),
                        }
                    }
                })
                .build(),
        )
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, new_window])
        .setup(move |app| {
            #[cfg(debug_assertions)]
            {
                if let Some(window) = app.get_webview_window("main") {
                    window.open_devtools();
                }
            }
            #[cfg(desktop)]
            {
                app.global_shortcut().register(summon)?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn new_window(app: AppHandle) -> tauri::Result<()> {
    let windows = app.webview_windows();
    let len = windows.len();

    WebviewWindowBuilder::new(
        &app,
        &format!(
            "my-window-{len
          }"
        ),
        tauri::WebviewUrl::App("index.html".into()),
    )
    .inner_size(1280.0, 1040.0)
    .build()?;
    Ok(())
}
