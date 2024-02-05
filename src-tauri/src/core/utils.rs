use winapi::um::winuser::{GetForegroundWindow, GetWindowThreadProcessId};


pub fn get_active_process_id() -> i32 {
  #[cfg(target_os = "macos")]
  {
      match get_active_window() {
          Ok(active_window) => {
              let process_id: i32 = active_window.process_id.try_into().unwrap();
              process_id
          },
          Err(()) => {
              println!("error occurred while getting the active window");
              0
          }
      }
  }

  #[cfg(target_os = "windows")]
  unsafe {
      let hwnd = GetForegroundWindow();
     
      let mut process_id: u32 = 0;

      GetWindowThreadProcessId(hwnd, &mut process_id );

      if process_id == 0 {
          println!("Could not get process id of active window");
          return 0;
      }

      process_id as i32
  }
}
