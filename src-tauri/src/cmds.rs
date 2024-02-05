

use crate::core::pick_color::PColor;

#[tauri::command]
pub fn get_p_color() {
    PColor::cur();
}

#[tauri::command]
pub async fn color_pick() {
    PColor::start();
}
