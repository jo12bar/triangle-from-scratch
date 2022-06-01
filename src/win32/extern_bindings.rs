//! Bindings to Windows functions and variables contained in various DLLs.

use super::{structs::*, typedefs::*};

#[link(name = "Gdi32")]
extern "system" {
    /// See [`ChoosePixelFormat` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-choosepixelformat).
    pub fn ChoosePixelFormat(hdc: HDC, ppfd: *const PIXELFORMATDESCRIPTOR) -> c_int;

    // See [`DescribePixelFormat` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-describepixelformat).
    pub fn DescribePixelFormat(
        hdc: HDC,
        iPixelFormat: c_int,
        nBytes: c_uint,
        ppfd: LPPIXELFORMATDESCRIPTOR,
    ) -> c_int;

    /// See [`SetPixelFormat` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setpixelformat).
    pub fn SetPixelFormat(hdc: HDC, format: c_int, ppfd: *const PIXELFORMATDESCRIPTOR) -> BOOL;
}

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

    /// See [`FreeLibrary` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-freelibrary).
    pub fn FreeLibrary(hLibModule: HMODULE) -> BOOL;

    /// See [`GetLastError` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror).
    pub fn GetLastError() -> DWORD;

    /// See [`GetModuleHandleW` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew).
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;

    /// See [`GetProcAddress` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress).
    pub fn GetProcAddress(hModule: HMODULE, lpProcName: LPCSTR) -> FARPROC;

    /// See [`LoadLibraryW` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryw).
    pub fn LoadLibraryW(lpLibFileName: LPCWSTR) -> HMODULE;

    /// See [`LocalFree` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree).
    pub fn LocalFree(hMem: HLOCAL) -> HLOCAL;

    /// See [`SetLastError` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror).
    pub fn SetLastError(dwErrCode: DWORD);
}

#[link(name = "Opengl32")]
extern "system" {
    /// See [`wglCreateContext` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-wglcreatecontext).
    pub fn wglCreateContext(Arg1: HDC) -> HGLRC;

    /// See [`wglDeleteContext` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-wgldeletecontext).
    pub fn wglDeleteContext(Arg1: HGLRC) -> BOOL;

    /// See [`wglGetProcAddress` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-wglgetprocaddress).
    pub fn wglGetProcAddress(Arg1: LPCSTR) -> PROC;

    /// See [`wglMakeCurrent` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-wglmakecurrent).
    pub fn wglMakeCurrent(hdc: HDC, hglrc: HGLRC) -> BOOL;
}

/// Type for [wglChoosePixelFormatARB](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt).
pub type wglChoosePixelFormatARB_t = Option<
    unsafe extern "system" fn(
        hdc: HDC,
        piAttribIList: *const c_int,
        pfAttribFList: *const FLOAT,
        nMaxFormats: UINT,
        piFormats: *mut c_int,
        nNumFormats: *mut UINT,
    ) -> BOOL,
>;

/// Type for [wglCreateContextAttribsARB](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_create_context.txt).
pub type wglCreateContextAttribsARB_t = Option<
    unsafe extern "system" fn(hDC: HDC, hShareContext: HGLRC, attribList: *const c_int) -> HGLRC,
>;

/// Type for [`wglGetExtensionsStringARB`](https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_extensions_string.txt).
pub type wglGetExtensionsStringARB_t = Option<unsafe extern "system" fn(HDC) -> *const c_char>;

/// Type for [wglSwapIntervalEXT](https://www.khronos.org/registry/OpenGL/extensions/EXT/WGL_EXT_swap_control.txt)
pub type wglSwapIntervalEXT_t = Option<unsafe extern "system" fn(interval: c_int) -> BOOL>;

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

    /// See [`GetDC` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdc).
    pub fn GetDC(hWnd: HWND) -> HDC;

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

    /// See [`ReleaseDC` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasedc).
    pub fn ReleaseDC(hWnd: HWND, hDC: HDC) -> c_int;

    /// See [`SetWindowLongPtrW` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw).
    pub fn SetWindowLongPtrW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> LONG_PTR;

    /// See [`ShowWindow` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow).
    pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;

    /// See [`TranslateMessage` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage).
    pub fn TranslateMessage(lpMsg: *const MSG) -> BOOL;

    /// See [`UnregisterClassW` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterclassw).
    pub fn UnregisterClassW(lpClassName: LPCWSTR, hInstance: HINSTANCE) -> BOOL;
}
