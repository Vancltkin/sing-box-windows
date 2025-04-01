use crate::app::kernel_service::{
    check_kernel_version, download_latest_kernel, get_process_status,
    restart_kernel, start_kernel, start_websocket_relay, stop_kernel,
};
use crate::app::proxy_service::{
    change_proxy, get_proxies, get_rules, get_version_info, set_system_proxy,
    set_tun_proxy, test_group_delay, toggle_ip_version,
};
use crate::app::subscription_service::{
    add_manual_subscription, download_subscription, get_current_config, get_current_proxy_mode,
    toggle_proxy_mode,
};
use crate::app::system_service::{check_admin, restart_as_admin};
use crate::app::update_service::{check_update, download_and_install_update};
use tauri::{AppHandle, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
use tauri_plugin_autostart::LaunchStrategy;

pub mod app;
pub mod config;
pub mod entity;
pub mod process;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = show_window(app);
        }))
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            LaunchStrategy::default(),
            Some(vec!["--hide"]),
        ))
        .system_tray(SystemTray::new().with_menu(
            SystemTrayMenu::new()
                .add_item(SystemTrayMenuItem::new("Show", "show"))
                .add_item(SystemTrayMenuItem::new("Exit", "exit")),
        ))
        .on_system_tray_event(|app, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    "show" => {
                        let window = app.get_window("main").unwrap();  
                        window.show().unwrap();
                    }
                    "exit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
        })
        .setup(|app| {
            let app_handle = app.handle().clone();
            app.listen_global("tauri://close-requested", move |_event| {
                let window = app_handle.get_window("main").unwrap();
                window.hide().unwrap(); // Прячет окно вместо выхода
            });

            let args: Vec<String> = std::env::args().collect();
            if args.len() > 1 && args[1] == "--hide" {
                let window = app.get_window("main").unwrap();
                window.hide().unwrap();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_kernel,
            download_latest_kernel,
            download_subscription,
            add_manual_subscription,
            get_current_config,
            stop_kernel,
            set_system_proxy,
            set_tun_proxy,
            check_admin,
            restart_as_admin,
            restart_kernel,
            toggle_ip_version,
            check_update,
            download_and_install_update,
            get_process_status,
            check_kernel_version,
            toggle_proxy_mode,
            get_current_proxy_mode,
            get_proxies,
            change_proxy,
            test_group_delay,
            get_version_info,
            get_rules,
            start_websocket_relay,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn show_window(app: &AppHandle) {
    let windows = app.webview_windows();
    windows
        .values()
        .next()
        .expect("Sorry, no window found")
        .set_focus()
        .expect("Can't Bring Window to Focus");
}
