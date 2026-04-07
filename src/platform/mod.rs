use egui::Pos2;
use winit::{dpi::LogicalPosition, window::Window};

use crate::constants;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

fn window_content_position(window: &Window) -> Option<LogicalPosition<f64>> {
    let position = if constants::window::DECORATIONS {
        window.inner_position().ok()?
    } else {
        window.outer_position().ok()?
    };

    Some(position.to_logical::<f64>(window.scale_factor()))
}

pub trait OverlayWindowPlatformExt {
    fn configure_overlay_window(&self);
    fn cursor_position_in_window(&self) -> Option<Pos2>;
}

#[cfg(target_os = "macos")]
impl OverlayWindowPlatformExt for Window {
    fn configure_overlay_window(&self) {
        macos::configure_overlay_window(self);
    }

    fn cursor_position_in_window(&self) -> Option<Pos2> {
        macos::cursor_position_in_window(self)
    }
}

#[cfg(target_os = "windows")]
impl OverlayWindowPlatformExt for Window {
    fn configure_overlay_window(&self) {
        windows::configure_overlay_window(self);
    }

    fn cursor_position_in_window(&self) -> Option<Pos2> {
        windows::cursor_position_in_window(self)
    }
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
impl OverlayWindowPlatformExt for Window {
    fn configure_overlay_window(&self) {}

    fn cursor_position_in_window(&self) -> Option<Pos2> {
        None
    }
}
