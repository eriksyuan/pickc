use crate::core::pick_color::PColor;
use crate::PreviousProcessId;

use parking_lot::Mutex;
use std::sync::Arc;

use anyhow::{bail, Result};
use once_cell::sync::OnceCell;
use tauri::{AppHandle, GlobalShortcutManager, Manager, State, Window};
use window_shadows::set_shadow;

use super::{
    utils,
    window_manager::WindowType,
};

pub struct Handle {
    pub app_handle: Arc<Mutex<Option<AppHandle>>>,
}

impl Handle {
    pub fn global() -> &'static Handle {
        static HANDLE: OnceCell<Handle> = OnceCell::new();
        HANDLE.get_or_init(|| Handle {
            app_handle: Arc::new(Mutex::new(None)),
        })
    }

    pub fn init(&self, handle: AppHandle) {
        *self.app_handle.lock() = Some(handle);
    }

    fn get_manager(&self) -> Result<impl GlobalShortcutManager> {
        let app_handle = self.app_handle.lock();
        if app_handle.is_none() {
            bail!("failed to get the hotkey manager");
        }
        Ok(app_handle.as_ref().unwrap().global_shortcut_manager())
    }

    pub fn refresh_global_shortcut(&self) {
        let mut manager = self.get_manager().unwrap();
        let _ = manager.register("CommandOrControl+Shift+D", || PColor::start());
    }

    pub fn get_window(&self) -> Option<Window> {
        self.app_handle
            .lock()
            .as_ref()
            .and_then(|a| a.get_window("main"))
    }

    pub fn update_previous_process_id() {
        let binding = Self::global().app_handle.lock();
        let previous_process_id: State<PreviousProcessId> =
            binding.as_ref().expect("Couldn't get app_handle").state();
        let mut p_id = previous_process_id.0.lock().unwrap();
        *p_id = utils::get_active_process_id();
    }

    pub fn open_window(window_type: WindowType) {
        Self::update_previous_process_id();
        let binding = Self::global().app_handle.lock();
        let app_handle = binding.as_ref().unwrap();

        let (label, index) = match window_type {
            WindowType::Picker => ("picker", 1),
            WindowType::Main => ("main", 0),
        };

        if let Some(window) = app_handle.get_window(label) {
            println!("windows is visible: {},{}", window.is_visible().unwrap(),label);
            if window.is_visible().unwrap() {
                let _ = window.hide();
                return;
            }
            let _ = window.unminimize();
            let _ = window.show();
            let _ = window.set_focus();
            return;
        }

        println!("create_window: {}", label);

        let new_window = tauri::window::WindowBuilder::from_config(
            app_handle,
            app_handle
                .config()
                .tauri
                .windows
                .get(index)
                .unwrap()
                .clone(),
        )
        .build();
        match new_window {
            Ok(window) => {
                let _ = window.show();
                let _ = window.set_focus();
                if let WindowType::Main = window_type {
                    set_shadow(&window, true).expect("Unsupported platform!");
                }
            }
            Err(e) => {
                println!("create_window error: {}", e);
            }
        }
    }

}
