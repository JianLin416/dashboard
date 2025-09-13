use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

mod serial;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn my_custom_command(name: &str) -> String {
    format!("{} was invoked from JavaScript!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        // .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .setup(|app| {
            let app_handle = app.handle();
            let app_handle_clone = app_handle.clone();

            tauri::async_runtime::spawn(async move {
                serial::start_serial_reader(app_handle_clone, "/dev/serial0").await;
            });

            let autostart_manager = app.autolaunch();
            // 启用 autostart
            let _ = autostart_manager.enable();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
