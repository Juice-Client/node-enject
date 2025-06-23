use napi_derive::napi;
use windows::Win32::{
    Foundation::{BOOL, HWND, LPARAM, LRESULT, WPARAM},
    System::SystemServices::MK_LBUTTON,
    UI::WindowsAndMessaging::{
        CallWindowProcW, EnumChildWindows, GetClassNameW, GetWindowLongPtrW, SetWindowLongPtrW,
        GWLP_WNDPROC, WM_LBUTTONDBLCLK, WM_LBUTTONDOWN, WM_MOUSEMOVE, WM_RBUTTONDBLCLK,
        WM_RBUTTONDOWN, WNDPROC,
    },
};

static mut PREV_WNDPROC: WNDPROC = None;

#[napi]
pub fn start_hook(hwnd: u32) {
    unsafe {
        let target = HWND(hwnd as usize as _);
        attach_hook(target);
    }
}

unsafe fn attach_hook(top_hwnd: HWND) {
    let mut target = HWND::default();
    let _ = EnumChildWindows(
        Some(top_hwnd),
        Some(find_render_widget),
        LPARAM(&mut target as *mut _ as _),
    );
    if target.0.is_null() {
        return;
    }

    let orig = GetWindowLongPtrW(target, GWLP_WNDPROC);
    PREV_WNDPROC = std::mem::transmute(orig);

    #[cfg(target_pointer_width = "64")]
    {
        SetWindowLongPtrW(target, GWLP_WNDPROC, wnd_proc as isize);
    }
    #[cfg(target_pointer_width = "32")]
    {
        SetWindowLongPtrW(
            target,
            GWLP_WNDPROC,
            (wnd_proc as isize).try_into().unwrap(),
        );
    }
}

unsafe extern "system" fn find_render_widget(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let buf = &mut [0u16; 256];
    let len = GetClassNameW(hwnd, buf) as usize;
    if String::from_utf16_lossy(&buf[..len]) == "Chrome_RenderWidgetHostHWND" {
        *(lparam.0 as *mut HWND) = hwnd;
        return BOOL(0);
    }
    BOOL(1)
}

#[no_mangle]
extern "system" fn wnd_proc(hwnd: HWND, msg: u32, mut wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        if matches!(
            msg,
            WM_MOUSEMOVE | WM_LBUTTONDOWN | WM_LBUTTONDBLCLK | WM_RBUTTONDOWN | WM_RBUTTONDBLCLK
        ) {
            wparam = WPARAM(wparam.0 & !MK_LBUTTON.0 as usize);
        }
        CallWindowProcW(PREV_WNDPROC, hwnd, msg, wparam, lparam)
    }
}
