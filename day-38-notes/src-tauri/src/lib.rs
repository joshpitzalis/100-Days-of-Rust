use tauri::{AppHandle, Manager, WebviewWindowBuilder};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, new_window])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
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
