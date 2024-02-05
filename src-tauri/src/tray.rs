use crate::core::window_manager::WindowType;

use super::core::handle::Handle;

use tauri::{AppHandle, CustomMenuItem, Manager, SystemTrayEvent, SystemTrayMenu};

pub fn on_system_tray_event(app_handle: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "exit" => {
                app_handle.exit(0);
                std::process::exit(0);
            }
            "show" => {
                Handle::open_window(WindowType::Main);
            }
            "hide" => {
                let window = app_handle.get_window("main");
                if let Some(window) = window {
                    window.hide().unwrap();
                }
            }
            _ => {}
        },
        SystemTrayEvent::LeftClick { .. } => {
            if let Some(window) = app_handle.get_window("main") {
                if let Err(err) = window.show() {
                    println!("Failed to show window: {}", err);
                }
                if let Err(err) = window.set_focus() {
                    println!("Failed to set focus on window: {}", err);
                }
            }
        }
        _ => {}
    }
}

pub fn tary_menu_init(app_handle: &AppHandle) {
    let menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show", "显示"))
        .add_item(CustomMenuItem::new("hide", "隐藏"))
        .add_item(CustomMenuItem::new("exit", "退出"));
    app_handle.tray_handle().set_menu(menu).unwrap();
}
