use std::env;
#[cfg(not(debug_assertions))]
use tauri::Manager;
#[cfg(not(debug_assertions))]
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
mod mqtt;
mod serial;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(debug_assertions)]
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_opener::init());

    // only enabled in release mode
    #[cfg(not(debug_assertions))]
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ));

    builder
        .setup(|app| {
            let mqtt_id: String = env::var("MQTT_ID").unwrap();
            let mqtt_host: String = env::var("MQTT_HOST").unwrap();
            let mqtt_port: u16 = env::var("MQTT_PORT").unwrap().parse().unwrap();
            let serial_port: String = env::var("SERIAL_PORT").unwrap();

            let app_handle = app.handle();
            let app_handle_clone = app_handle.clone();

            // only enabled in release mode
            #[cfg(not(debug_assertions))]
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_fullscreen(true);
            }

            tauri::async_runtime::spawn(async move {
                if let Some(client) = mqtt::connect_mqtt(mqtt_id, mqtt_host, mqtt_port).await {
                    // cu.usbserial-110
                    // serial0
                    serial::start_serial_reader(app_handle_clone, serial_port, client).await;
                } else {
                    serial::start_serial_reader_without_mqtt(app_handle_clone, serial_port).await;
                }
            });

            // only enabled in release mode
            #[cfg(not(debug_assertions))]
            {
                let autostart_manager = app.autolaunch();
                let _ = autostart_manager.enable();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
