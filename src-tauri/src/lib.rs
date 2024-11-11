use crate::app::app_service::{download_latest_kernel, start_kernel, download_subscription, stop_kernel};

mod app;
mod entity;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![start_kernel,download_latest_kernel,download_subscription,stop_kernel])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
