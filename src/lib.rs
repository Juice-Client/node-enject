use napi_derive::napi;
use windows::Win32::{
    Foundation::{GetLastError, SetLastError, BOOL, HWND, LPARAM, LRESULT, WIN32_ERROR, WPARAM},
    System::SystemServices::MK_LBUTTON,
    UI::WindowsAndMessaging::{
        CallWindowProcW, EnumChildWindows, GetClassNameW, SetWindowLongPtrW, GWLP_WNDPROC,
        WM_LBUTTONDBLCLK, WM_LBUTTONDOWN, WM_MOUSEMOVE, WM_RBUTTONDBLCLK, WM_RBUTTONDOWN, WNDPROC,
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

    SetLastError(WIN32_ERROR(0));
    let new_proc = wnd_proc as usize as isize;
    let prev = SetWindowLongPtrW(target, GWLP_WNDPROC, new_proc);
    let err = GetLastError(); // <-- здесь объявляется `err`

    if prev == 0 && err.0 != 0 {
        return;
    }

    PREV_WNDPROC = std::mem::transmute(prev);
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
