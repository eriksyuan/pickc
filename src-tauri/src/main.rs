// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::{App, SystemTray};

mod cmds;
mod core;
mod tray;

#[derive(Debug)]
pub struct PreviousProcessId(Mutex<i32>);

fn main() {
    let app = tauri::Builder::default()
        .setup(|app| {
            set_up(app);
            Ok(())
        })
        .manage(PreviousProcessId(Default::default()))
        .system_tray(SystemTray::new().with_tooltip("PickC"))
        .on_system_tray_event(tray::on_system_tray_event)
        .invoke_handler(tauri::generate_handler![
            cmds::get_p_color,
            cmds::color_pick
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        tauri::RunEvent::Exit => {
            app_handle.exit(0);
        }
        _ => {}
    })
}

fn set_up(app: &mut App) {
    let handle = core::handle::Handle::global();
    handle.init(app.handle());
    handle.refresh_global_shortcut();
    tray::tary_menu_init(&app.handle());
}
