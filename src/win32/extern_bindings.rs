//! Bindings to Windows functions and variables contained in various DLLs.

use super::{structs::*, typedefs::*};

#[link(name = "Kernel32")]
extern "system" {
    /// See [`FormatMessageW` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew).
    pub fn FormatMessageW(
        dwFlags: DWORD,
        lpSource: LPCVOID,
        dwMessageId: DWORD,
        dwLanguageId: DWORD,
        lpBuffer: LPWSTR,
        nSize: DWORD,
        Arguments: va_list,
    ) -> DWORD;

    /// See [`GetLastError` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror).
    pub fn GetLastError() -> DWORD;

    /// See [`GetModuleHandleW` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew).
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;

    /// See [`LocalFree` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree).
    pub fn LocalFree(hMem: HLOCAL) -> HLOCAL;

    /// See [`SetLastError` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror).
    pub fn SetLastError(dwErrCode: DWORD);
}

#[link(name = "User32")]
extern "system" {
    /// See [`BeginPaint` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-beginpaint).
    pub fn BeginPaint(hWnd: HWND, lpPaint: LPPAINTSTRUCT) -> HDC;

    /// See [`CreateWindowExW` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
    pub fn CreateWindowExW(
        dwExStyle: DWORD,
        lpClassName: LPCWSTR,
        lpWindowName: LPCWSTR,
        dwStyle: DWORD,
        X: c_int,
        Y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID,
    ) -> HWND;

    /// See [`DefWindowProcW` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
    pub fn DefWindowProcW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;

    /// See [`DestroyWindow` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow).
    pub fn DestroyWindow(hWnd: HWND) -> BOOL;

    /// See [`DispatchMessageW` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew).
    pub fn DispatchMessageW(lpMsg: *const MSG) -> LRESULT;

    /// See [`EndPaint` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint).
    pub fn EndPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;

    /// See [`FillRect` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-fillrect).
    pub fn FillRect(hDC: HDC, lprc: *const RECT, hbr: HBRUSH) -> c_int;

    /// See [`GetMessageW` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew).
    pub fn GetMessageW(lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL;

    /// See [`GetWindowLongPtrW` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw).
    pub fn GetWindowLongPtrW(hWnd: HWND, nIndex: c_int) -> LONG_PTR;

    /// See [`LoadCursorW` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw).
    pub fn LoadCursorW(hInstance: HINSTANCE, lpCursorName: LPCWSTR) -> HCURSOR;

    /// See [`MessageBoxW` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw).
    pub fn MessageBoxW(hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT) -> c_int;

    /// See [`PostQuitMessage` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage).
    pub fn PostQuitMessage(nExitCode: c_int);

    /// See [`RegisterClassW` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassw).
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;

    /// See [`SetWindowLongPtrW` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw).
    pub fn SetWindowLongPtrW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> LONG_PTR;

    /// See [`ShowWindow` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow).
    pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;

    /// See [`TranslateMessage` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage).
    pub fn TranslateMessage(lpMsg: *const MSG) -> BOOL;
}
