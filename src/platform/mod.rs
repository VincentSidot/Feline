use egui::Pos2;
use winit::window::Window;

#[cfg(target_os = "macos")]
mod macos;

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

#[cfg(not(target_os = "macos"))]
impl OverlayWindowPlatformExt for Window {
    fn configure_overlay_window(&self) {}

    fn cursor_position_in_window(&self) -> Option<Pos2> {
        None
    }
}
