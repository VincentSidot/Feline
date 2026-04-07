use egui::Pos2;
use std::{
    ffi::{c_char, c_void},
    ptr,
};
use winit::{
    raw_window_handle::{HasWindowHandle, RawWindowHandle},
    window::Window,
};

use super::window_content_position;

const NS_WINDOW_COLLECTION_BEHAVIOR_CAN_JOIN_ALL_SPACES: usize = 1 << 0;
const NS_WINDOW_COLLECTION_BEHAVIOR_STATIONARY: usize = 1 << 4;
const NS_WINDOW_COLLECTION_BEHAVIOR_FULLSCREEN_AUXILIARY: usize = 1 << 8;

#[repr(C)]
pub struct CGPoint {
    pub x: f64,
    pub y: f64,
}

type CGEventRef = *mut c_void;
type Sel = *const c_void;

#[link(name = "CoreGraphics", kind = "framework")]
unsafe extern "C" {
    fn CGEventCreate(source: *const c_void) -> CGEventRef;
    fn CGEventGetLocation(event: CGEventRef) -> CGPoint;
}

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    fn CFRelease(cf: *const c_void);
}

#[link(name = "objc")]
unsafe extern "C" {
    fn sel_registerName(name: *const c_char) -> Sel;
    fn objc_msgSend();
}

pub fn configure_overlay_window(window: &Window) {
    let Ok(window_handle) = window.window_handle() else {
        log::warn!("Failed to get raw window handle for macOS overlay configuration");
        return;
    };

    let RawWindowHandle::AppKit(handle) = window_handle.as_raw() else {
        return;
    };

    let ns_view = handle.ns_view.as_ptr();
    let ns_window = unsafe { objc_msg_send_id(ns_view, selector(c"window".as_ptr())) };
    if ns_window.is_null() {
        log::warn!("Failed to get NSWindow for macOS overlay configuration");
        return;
    }

    let current_behavior =
        unsafe { objc_msg_send_usize(ns_window, selector(c"collectionBehavior".as_ptr())) };
    let overlay_behavior = NS_WINDOW_COLLECTION_BEHAVIOR_CAN_JOIN_ALL_SPACES
        | NS_WINDOW_COLLECTION_BEHAVIOR_STATIONARY
        | NS_WINDOW_COLLECTION_BEHAVIOR_FULLSCREEN_AUXILIARY;

    unsafe {
        objc_msg_send_void_usize(
            ns_window,
            selector(c"setCollectionBehavior:".as_ptr()),
            current_behavior | overlay_behavior,
        );
    }
}

pub fn cursor_position_in_window(window: &Window) -> Option<Pos2> {
    let cursor_position = cursor_position()?;
    let window_position = window_content_position(window)?;

    Some(Pos2::new(
        (cursor_position.x - window_position.x) as f32,
        (cursor_position.y - window_position.y) as f32,
    ))
}

fn cursor_position() -> Option<CGPoint> {
    let event = unsafe { CGEventCreate(ptr::null()) };
    if event.is_null() {
        return None;
    }

    let position = unsafe { CGEventGetLocation(event) };
    unsafe { CFRelease(event.cast_const()) };

    Some(position)
}

fn selector(name: *const c_char) -> Sel {
    unsafe { sel_registerName(name) }
}

unsafe fn objc_msg_send_id(receiver: *mut c_void, selector: Sel) -> *mut c_void {
    let msg_send: unsafe extern "C" fn(*mut c_void, Sel) -> *mut c_void =
        unsafe { std::mem::transmute(objc_msgSend as *const ()) };
    unsafe { msg_send(receiver, selector) }
}

unsafe fn objc_msg_send_usize(receiver: *mut c_void, selector: Sel) -> usize {
    let msg_send: unsafe extern "C" fn(*mut c_void, Sel) -> usize =
        unsafe { std::mem::transmute(objc_msgSend as *const ()) };
    unsafe { msg_send(receiver, selector) }
}

unsafe fn objc_msg_send_void_usize(receiver: *mut c_void, selector: Sel, arg: usize) {
    let msg_send: unsafe extern "C" fn(*mut c_void, Sel, usize) =
        unsafe { std::mem::transmute(objc_msgSend as *const ()) };
    unsafe { msg_send(receiver, selector, arg) };
}
