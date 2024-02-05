use serde::{Deserialize, Serialize};
use tauri::{Manager, PhysicalPosition};
use winapi::{
    shared::windef::POINT,
    um::winuser::{GetAsyncKeyState, GetCursorPos},
};

use super::{handle::Handle, window_manager::WindowType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color(u8, u8, u8, u8);

#[derive(Debug, Serialize, Deserialize)]
pub struct PColor {
    pos: (i32, i32),
    colors: Vec<Color>,
    color: Color,
}

impl PColor {
    pub fn cur() -> Self {
        let mut p = POINT { x: 0, y: 0 };

        unsafe {
            // 32*32
            let space: u32 = 4;
            GetCursorPos(&mut p);

            let screen = screenshots::Screen::from_point(p.x, p.y).expect("cant find screen");

            let width_and_height: u32 = space * 2 + 1;
            let image = screen
                .capture_area(
                    p.x - screen.display_info.x - space as i32,
                    p.y - screen.display_info.y - space as i32,
                    width_and_height,
                    width_and_height,
                )
                .expect("screenshot error");

            let colors = image
                .pixels()
                .map(|c| Color(c[0], c[1], c[2], c[3]))
                .collect::<Vec<Color>>();

            let color = colors[colors.len() / 2].clone();
            PColor {
                pos: (p.x, p.y),
                colors,
                color,
            }
        }
    }

    pub fn start() {
        // unsafe {
        //     ShowCursor(false);
        // }

        println!("start!!!!!");
        Handle::open_window(WindowType::Main);
        Handle::open_window(WindowType::Picker);

        let picker_window = Handle::global()
            .app_handle
            .lock()
            .as_ref()
            .and_then(|a| a.get_window("picker"))
            .unwrap();

        let mut _color = Color(0, 0, 0, 0);
        loop {
            // sleep 20ms
            std::thread::sleep(std::time::Duration::from_millis(10));
            let p_color = PColor::cur();
            picker_window.set_focus().unwrap();
            picker_window.emit("p_color", &p_color).unwrap();
            let _ = picker_window.set_position(PhysicalPosition {
                x: p_color.pos.0 - 55,
                y: p_color.pos.1 - 20,
            });
            unsafe {
                // 检查鼠标左键是否按下
                if GetAsyncKeyState(1) != 0 {
                    // ShowCursor(true);
                    _color = p_color.color;
                    break;
                }
            }

            // 检查鼠标左键是否按下
        }
        Handle::open_window(WindowType::Picker);
        Handle::open_window(WindowType::Main);
        let _ = Handle::global()
            .get_window()
            .unwrap()
            .emit("select_color", _color);
    }
}
