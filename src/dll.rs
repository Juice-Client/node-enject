#![allow(non_snake_case)]
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::mem::transmute;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::SystemServices::MK_LBUTTON;
use windows::Win32::UI::WindowsAndMessaging::{
    CallWindowProcW, EnumChildWindows, GetClassNameW, GetWindowLongPtrW, SetWindowLongPtrW,
    GWLP_WNDPROC, WM_LBUTTONDBLCLK, WM_LBUTTONDOWN, WM_MOUSEMOVE, WM_RBUTTONDBLCLK, WM_RBUTTONDOWN,
    WNDPROC,
};

static mut PREV_WNDPROC: WNDPROC = None;

struct HwndHolder(HWND);

#[napi]
pub fn start_hook(hwnd_val: u32) -> Result<()> {
    unsafe {
        let top_hwnd = HWND(hwnd_val as usize as *mut core::ffi::c_void);
        attach_hook(top_hwnd);
    }
    Ok(())
}

unsafe fn attach_hook(top_hwnd: HWND) {
    let mut holder = HwndHolder(HWND::default());
    let _ = EnumChildWindows(
        Some(top_hwnd),
        Some(find_render_widget),
        LPARAM(&mut holder as *mut _ as _),
    );
    let target_hwnd = holder.0;
    if target_hwnd.0.is_null() {
        return;
    }

    let orig = GetWindowLongPtrW(target_hwnd, GWLP_WNDPROC);
    PREV_WNDPROC = transmute(orig);
    SetWindowLongPtrW(target_hwnd, GWLP_WNDPROC, wnd_proc as isize);
}

unsafe extern "system" fn find_render_widget(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let buf = &mut [0u16; 256];
    let len = GetClassNameW(hwnd, buf);
    let class = String::from_utf16_lossy(&buf[..len as usize]);
    if class == "Chrome_RenderWidgetHostHWND" {
        *(lparam.0 as *mut HWND) = hwnd;
        return BOOL(0);
    }
    BOOL(1)
}

#[no_mangle]
extern "system" fn wnd_proc(
    window: HWND,
    message: u32,
    mut wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        match message {
            WM_MOUSEMOVE | WM_LBUTTONDOWN | WM_LBUTTONDBLCLK | WM_RBUTTONDOWN
            | WM_RBUTTONDBLCLK => {
                wparam = WPARAM(wparam.0 & !MK_LBUTTON.0 as usize);
            }
            _ => {}
        }
        CallWindowProcW(PREV_WNDPROC, window, message, wparam, lparam)
    }
}
