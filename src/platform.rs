use egui::Pos2;
use winit::window::Window;

#[cfg(target_os = "macos")]
pub fn cursor_position_in_window(window: &Window) -> Option<Pos2> {
    let cursor_position = macos::cursor_position()?;
    let window_position = window
        .inner_position()
        .ok()?
        .to_logical::<f64>(window.scale_factor());

    Some(Pos2::new(
        (cursor_position.x - window_position.x) as f32,
        (cursor_position.y - window_position.y) as f32,
    ))
}

#[cfg(not(target_os = "macos"))]
pub fn cursor_position_in_window(_window: &Window) -> Option<Pos2> {
    None
}

#[cfg(target_os = "macos")]
mod macos {
    use std::{ffi::c_void, ptr};

    #[repr(C)]
    pub struct CGPoint {
        pub x: f64,
        pub y: f64,
    }

    type CGEventRef = *mut c_void;

    #[link(name = "CoreGraphics", kind = "framework")]
    unsafe extern "C" {
        fn CGEventCreate(source: *const c_void) -> CGEventRef;
        fn CGEventGetLocation(event: CGEventRef) -> CGPoint;
    }

    #[link(name = "CoreFoundation", kind = "framework")]
    unsafe extern "C" {
        fn CFRelease(cf: *const c_void);
    }

    pub fn cursor_position() -> Option<CGPoint> {
        let event = unsafe { CGEventCreate(ptr::null()) };
        if event.is_null() {
            return None;
        }

        let position = unsafe { CGEventGetLocation(event) };
        unsafe { CFRelease(event.cast_const()) };

        Some(position)
    }
}
