use egui::Pos2;
use windows_sys::Win32::{Foundation::POINT, UI::WindowsAndMessaging::GetCursorPos};
use winit::{dpi::PhysicalPosition, window::Window};

use super::window_content_position;

pub fn configure_overlay_window(_window: &Window) {
    // No special configuration needed for Windows
}

pub fn cursor_position_in_window(window: &Window) -> Option<Pos2> {
    let cursor_position = cursor_position()?.to_logical::<f64>(window.scale_factor());
    let window_position = window_content_position(window)?;

    Some(Pos2::new(
        (cursor_position.x - window_position.x) as f32,
        (cursor_position.y - window_position.y) as f32,
    ))
}

fn cursor_position() -> Option<PhysicalPosition<i32>> {
    let mut position = POINT { x: 0, y: 0 };
    if unsafe { GetCursorPos(&mut position) } == 0 {
        return None;
    }

    Some(PhysicalPosition::new(position.x, position.y))
}
