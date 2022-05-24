// If we're running with debug assertions, we probably want a console auto-allocated for us too.
// In that case, only compile for the windows subsystem if debug assertions are disabled (e.g. in
// the release profile).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::ptr;

/// Implements zero-initialization for C-style structs.
///
/// # Safety
/// This macro uses [`core::mem::MaybeUninit::zeroed()`] and `core::mem::MaybeUninit::assume_init()`
/// to zero-out all of the struct's memory. This might not be a defined value for the struct, so
/// please ensure that the struct can *actually* be zero-initialized before using this macro.
macro_rules! unsafe_impl_default_zeroed {
    ($t:ty) => {
        impl Default for $t {
            #[inline]
            #[must_use]
            fn default() -> Self {
                unsafe { core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
            }
        }
    };
}

/// An atom, representing a string in the system-defined atom table.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinDef.h as follows:
///
/// ```c
/// typedef WORD ATOM;
/// ```
#[allow(clippy::upper_case_acronyms)]
type ATOM = WORD;

/// In Windows, booleans are actually 16-bit ints.
#[allow(clippy::upper_case_acronyms)]
type BOOL = c_int;

/// A byte (8 bits).
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#byte), this
/// is defined in WinDef.h as follows:
///
/// ```c
/// typedef unsigned char BYTE;
/// ```
#[allow(clippy::upper_case_acronyms)]
type BYTE = u8;

/// The C representation of an `int` on x86.
#[allow(non_camel_case_types)]
type c_int = i32;

/// The C representation of a `long` on x86.
#[allow(non_camel_case_types)]
type c_long = i32;

/// The C representation of an `unsigned int` on x86.
#[allow(non_camel_case_types)]
type c_uint = u32;

/// The C representation of an `unsigned long` on x86.
#[allow(non_camel_case_types)]
type c_ulong = u32;

/// The C representation of an `unsigned short` on x86.
#[allow(non_camel_case_types)]
type c_ushort = u16;

/// A 32-bit unsigned integer. The range is 0 through 4294967295 decimal.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in IntSafe.h as follows:
///
/// ```c
/// typedef unsigned long DWORD;
/// ```
#[allow(clippy::upper_case_acronyms)]
type DWORD = c_ulong;

/// A handle to a win32 object.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinNT.h as follows:
///
/// ```c
/// typedef PVOID HANDLE;
/// ```
#[allow(clippy::upper_case_acronyms)]
type HANDLE = PVOID;

/// A handle to a [brush](https://docs.microsoft.com/en-us/windows/win32/gdi/brushes).
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinDef.h as follows:
///
/// ```c
/// typedef HANDLE HBRUSH;
/// ```
#[allow(clippy::upper_case_acronyms)]
type HBRUSH = HANDLE;

/// A handle to a [cursor](https://docs.microsoft.com/en-us/windows/win32/menurc/cursors).
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinDef.h as follows:
///
/// ```c
/// typedef HICON HCURSOR;
/// ```
#[allow(clippy::upper_case_acronyms)]
type HCURSOR = HICON;

/// A handle to a device context (DC).
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdc), this
/// is defined in WinDef.h as follows:
///
/// ```c
/// typedef HANDLE HDC;
/// ```
#[allow(clippy::upper_case_acronyms)]
type HDC = HANDLE;

/// A handle to an [icon](https://docs.microsoft.com/en-us/windows/win32/menurc/icons).
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinDef.h as follows:
///
/// ```c
/// typedef HANDLE HICON;
/// ```
#[allow(clippy::upper_case_acronyms)]
type HICON = HANDLE;

/// A handle to a instance. This is the base address of the module in memory.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinDef.h as follows:
///
/// ```c
/// typedef HANDLE HINSTANCE;
/// ```
#[allow(clippy::upper_case_acronyms)]
type HINSTANCE = HANDLE;

/// A handle to a [menu](https://docs.microsoft.com/en-us/windows/desktop/menurc/menus).
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinDef.h as follows:
///
/// ```c
/// typedef HANDLE HMENU;
/// ```
#[allow(clippy::upper_case_acronyms)]
type HMENU = HANDLE;

/// On 16-bit Windows, HMODULE and HINSTANCE were different types. Now they're the same thing.
#[allow(clippy::upper_case_acronyms)]
type HMODULE = HINSTANCE;

/// A handle to a [window](https://docs.microsoft.com/en-us/windows/win32/winmsg/windows).
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinDef.h as follows:
///
/// ```c
/// typedef HANDLE HWND;
/// ```
#[allow(clippy::upper_case_acronyms)]
type HWND = HANDLE;

/// A 32-bit signed integer. See [MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#long).
#[allow(clippy::upper_case_acronyms)]
type LONG = c_long;

/// A signed long type for pointer precision. [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types),
/// this should be used when casting a pointer to a `long` to perform pointer arithmetic.
///
/// The type is declared in BaseTsd.h as follows:
///
/// ```c
/// #if defined(_WIN64)
///  typedef __int64 LONG_PTR;
/// #else
///  typedef long LONG_PTR;
/// #endif
/// ```
///
/// This allows `LONG_PTR` to be 64-bits long on 64-bit platforms, and some other size (usually 32-bits)
/// on other platforms. Rust has this functionality built in in the form of the [`isize`] type.
#[allow(non_camel_case_types)]
type LONG_PTR = isize;

/// A message parameter.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinDef.h as follows:
///
/// ```c
/// typedef LONG_PTR LPARAM;
/// ```
#[allow(clippy::upper_case_acronyms)]
type LPARAM = LONG_PTR;

/// A pointer to a constant null-terminated string of 16-bit Unicode characters.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinDef.h as follows:
///
/// ```c
/// typedef CONST WCHAR *LPCWSTR;
/// ```
#[allow(clippy::upper_case_acronyms)]
type LPCWSTR = *const WCHAR;

/// A pointer to any type. Basically a c-style void pointer.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinDef.h as follows:
///
/// ```c
/// typedef void *LPVOID;
/// ```
#[allow(clippy::upper_case_acronyms)]
type LPVOID = *mut core::ffi::c_void;

/// A pointer to a null-terminated string of 16-bit UTF16 Unicode chars.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinNT.h as follows:
///
/// ```c
/// typedef WCHAR *LPWSTR;
/// ```
#[allow(clippy::upper_case_acronyms)]
type LPWSTR = *mut WCHAR;

/// The signed result of message processing.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinDef.h as follows:
///
/// ```c
/// typedef LONG_PTR LRESULT;
/// ```
#[allow(clippy::upper_case_acronyms)]
type LRESULT = LONG_PTR;

/// A pointer to any type - basically a c-style void pointer.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinNT.h as follows:
///
/// ```c
/// typedef void *PVOID;
/// ```
#[allow(clippy::upper_case_acronyms)]
type PVOID = *mut core::ffi::c_void;

/// Corresponds to [the following typedef in windows.h](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/52ddd4c3-55b9-4e03-8287-5392aac0627f):
///
/// ```c
/// typedef unsigned int UINT;
/// ```
#[allow(clippy::upper_case_acronyms)]
type UINT = c_uint;

/// An unsigned int type for pointer precision. [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types),
/// this should be used when casting a pointer to a `unsigned int` to perform pointer arithmetic.
///
/// The type is declared in BaseTsd.h as follows:
///
/// ```c
/// #if defined(_WIN64)
///  typedef unsigned __int64 UINT_PTR;
/// #else
///  typedef unsigned int UINT_PTR;
/// #endif
/// ```
///
/// This allows `UINT_PTR` to be 64-bits long on 64-bit platforms, and some other size (usually 16-bits)
/// on other platforms. Rust has this functionality built in in the form of the [`usize`] type.
#[allow(non_camel_case_types)]
type UINT_PTR = usize;

/// An unsigned long type for pointer precision. [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#ulong_ptr),
/// this should be used when casting a pointer to an `unsigned long` to perform pointer arithmetic.
///
/// This type is declared in BaseTsd.h as follows:
///
/// ```c
/// #if defined(_WIN64)
///  typedef unsigned __int64 ULONG_PTR;
/// #else
///  typedef unsigned long ULONG_PTR;
/// #endif
/// ```
#[allow(non_camel_case_types)]
type ULONG_PTR = usize;

/// A 16-bit Unicode character.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinNT.h as follows:
///
/// ```c
/// typedef wchar_t WCHAR;
/// ```
#[allow(clippy::upper_case_acronyms)]
type WCHAR = wchar_t;

/// A "wide character", equivalent to a 16-bit number.
#[allow(non_camel_case_types)]
type wchar_t = u16;

/// A message parameter.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinDef.h as follows:
///
/// ```c
/// typedef UINT_PTR WPARAM;
/// ```
#[allow(clippy::upper_case_acronyms)]
type WPARAM = UINT_PTR;

/// A nullable pointer to a callback function that processes messages sent to a window.
///
/// See [MSDN's explanation](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-wndproc).
#[allow(clippy::upper_case_acronyms)]
type WNDPROC = Option<
    unsafe extern "system" fn(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT,
>;

/// A 16-bit unsigned integer. The range is 0 through 65535 decimal.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinDef.h as follows:
///
/// ```c
/// typedef unsigned short WORD;
/// ```
#[allow(clippy::upper_case_acronyms)]
type WORD = c_ushort;

/// The [win32 `WNDCLASSW` structure][wndclassw_msdn], used by the [RegisterClass][registerclass_msdn]
/// function to register a new window.
///
/// [wndclassw_msdn]: https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassw
/// [registerclass_msdn]: https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassa
#[allow(non_snake_case, clippy::upper_case_acronyms)]
#[repr(C)]
pub struct WNDCLASSW {
    /// The [Class Styles](https://docs.microsoft.com/en-us/windows/desktop/winmsg/about-window-classes)
    /// to apply to this window.
    style: UINT,
    /// A pointer to the window procedure.
    lpfnWndProc: WNDPROC,
    /// The number of extra bytes to allocate following the window-class structure. The system
    /// initializes the bytes to zero.
    cbClsExtra: c_int,
    /// The number of extra bytes to allocate following the window instance. The system initializes
    /// the bytes to zero.
    cbWndExtra: c_int,
    /// A handle to the instance that contains the window procedure for the class.
    hInstance: HINSTANCE,
    /// A handle to the class icon. This must be a handle to the icon _resource_. If this member
    /// is null, the system provides a default icon.
    hIcon: HICON,
    /// A handle to the class cursor. This must be a handle to the cursor _resource_. If this member
    /// is null, an application must explicitly set the cursor shape whenever the mouse moves into
    /// the application's window.
    hCursor: HCURSOR,
    /// A handle to the class background brush.
    hbrBackground: HBRUSH,
    /// The resource name of the class menu, as the name appears in the resource file.
    lpszMenuName: LPCWSTR,
    /// The window class name, with a maximum length of 256.
    lpszClassName: LPCWSTR,
}

unsafe_impl_default_zeroed! { WNDCLASSW }

/// Defines the initialization parameters passed to the window procedure of an application.
/// The members of this struct are identical to the parameters of the
/// [CreateWindowEx][msdn-create-window-ex] function.
///
/// [msdn-create-window-ex]: https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-createwindowexa
#[allow(non_snake_case)]
#[repr(C)]
pub struct CREATESTRUCTW {
    /// Additional data that may be used to create the window. In particular, this member may contain
    /// data passed to the `lpParam` parameter of the [CreateWindow][msdn-create-window] or
    /// [CreateWindowEx][msdn-create-window-ex] functions.
    ///
    /// [See MSDN for details](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-createstructw#members).
    ///
    /// [msdn-create-window]: https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-createwindowa
    /// [msdn-create-window-ex]: https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-createwindowexa
    lpCreateParams: LPVOID,
    /// A handle to the module that owns the new window.
    hInstance: HINSTANCE,
    /// A handle to the menu to be used by the new window.
    hMenu: HMENU,
    /// A handle to the parent/owner window, if the window is a child window or owned. If the window
    /// is neither a child window nor owned, this will be a `NULL` handle.
    hwndParent: HWND,
    /// The height of the new window, in pixels.
    cy: c_int,
    /// The width of the new window, in pixels.
    cx: c_int,
    /// The y-coordinate of the upper left corner of the new window. If the window is a child window,
    /// coordinates are relative to the parent window. Otherwise, coordinates are relative to the
    /// screen origin.
    y: c_int,
    /// The x-coordinate of the upper left corner of the new window. If the window is a child window,
    /// coordinates are relative to the parent window. Otherwise, coordinates are relative to the
    /// screen origin.
    x: c_int,
    /// The [window style](https://docs.microsoft.com/en-us/windows/desktop/winmsg/window-styles)
    /// for the new window.
    style: LONG,
    /// The name of the new window.
    lpszName: LPCWSTR,
    /// A pointer to a null-terminated string or atom that specifies the class name of the new
    /// window.
    ///
    /// [MSDN remarks](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-createstructw#remarks)
    /// that, since this member can contain a pointer to a local (and thus inacessible) atom, **the
    /// class name shold not be obtained via this member**. The
    /// [`GetClassName`](https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-getclassname)
    /// function should be used instead.
    lpszClass: LPCWSTR,
    /// The [extended window style](https://docs.microsoft.com/en-us/windows/desktop/winmsg/extended-window-styles)
    /// for this new window.
    dwExStyle: DWORD,
}

unsafe_impl_default_zeroed! { CREATESTRUCTW }

#[allow(clippy::upper_case_acronyms)]
type LPCREATESTRUCTW = *mut CREATESTRUCTW;

/// Contains information an application can use to paint the client area of a window it owns.
///
/// [See MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-paintstruct).
#[allow(non_snake_case)]
#[repr(C)]
pub struct PAINTSTRUCT {
    /// A handle to the display device context to be used for painting.
    hdc: HDC,
    /// Indicates whether the background should be erased. The value is nonzero if the application
    /// should erase the background - for example, if the window class is created without a
    /// background brush. See the
    /// [description of the `hbrBackground` member of the `WNDCLASS` structure on MSDN][msdn-wndclass]
    /// for details.
    ///
    /// [msdn-wndclass]: https://docs.microsoft.com/en-us/windows/desktop/api/winuser/ns-winuser-wndclassa
    fErase: BOOL,
    /// A rectangle that specifies the upper left and lower right corners of the rectangle in which
    /// the painting is requested, in device units relative to the upper-left corner of the client
    /// area.
    rcPaint: RECT,
    /// Reserved by Windows (used internally by the system).
    fRestore: BOOL,
    /// Reserved by Windows (used internally by the system).
    fIncUpdate: BOOL,
    /// Reserved by Windows (used internally by the system).
    rgbReserved: [BYTE; 32],
}

unsafe_impl_default_zeroed! { PAINTSTRUCT }

#[allow(clippy::upper_case_acronyms)]
type LPPAINTSTRUCT = *mut PAINTSTRUCT;

/// Defines the xy-coordinates of a point.
///
/// [See MSDN](https://docs.microsoft.com/en-us/windows/win32/api/windef/ns-windef-point).
#[allow(clippy::upper_case_acronyms)]
#[repr(C)]
pub struct POINT {
    x: LONG,
    y: LONG,
}

unsafe_impl_default_zeroed! { POINT }

/// Defines a rectangle by the coordinates of its upper-left and lower-right corners.
///
/// [See MSDN](https://docs.microsoft.com/en-us/windows/win32/api/windef/ns-windef-rect).
#[repr(C)]
pub struct RECT {
    left: LONG,
    top: LONG,
    right: LONG,
    bottom: LONG,
}

unsafe_impl_default_zeroed! { RECT }

/// Contains message information from a thread's message queue.
///
/// See [MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-msg).
#[allow(non_snake_case, clippy::upper_case_acronyms)]
#[repr(C)]
pub struct MSG {
    /// A handle to the window whose window procedure receives the message. NULL if this message is
    /// a thread message.
    hwnd: HWND,
    /// The message identifier. Applications can only use the low work; the high word is reserved
    /// by the system.
    message: UINT,
    /// Additional information about the message.
    wParam: WPARAM,
    /// Additional information about the message.
    lParam: LPARAM,
    /// The time at which the message was posted.
    time: DWORD,
    /// The cursor position, in screen coordinates, when the message was posted.
    pt: POINT,
    /// Private memory, not to be touched.
    lPrivate: DWORD,
}

#[allow(clippy::upper_case_acronyms)]
type LPMSG = *mut MSG;

unsafe_impl_default_zeroed! { MSG }

#[link(name = "Kernel32")]
extern "system" {
    /// See [`GetLastError` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror).
    pub fn GetLastError() -> DWORD;

    /// See [`GetModuleHandleW` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew).
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
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

/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makeintresourcew):
///
/// > Converts an integer value to a resource type compatible with the resource-management functions.
/// > This macro is used in place of a string containing the name of the resource.
///
/// As MSDN notes, this is defined as a C preprocessor macro, so we can't link to it from Rust code.
/// Instead, the macro has been re-created as a Rust `const fn`, enabling additional typechecking.
/// The original C macro is defined in WinUser.h, for reference.
#[allow(non_snake_case)]
pub const fn MAKEINITRESOURCEW(i: WORD) -> LPWSTR {
    i as ULONG_PTR as LPWSTR
}

/// The default background color of a window.
pub const COLOR_WINDOW: u32 = 5;

pub const CW_USEDEFAULT: c_int = 0x80000000_u32 as c_int;

/// For use with [`SetWindowLongPtrW()`].
///
/// [From MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw#parameters):
///
/// > Sets the user data associated with the window. This data is intended for use by the
/// > application that created the window. Its value is initially zero.
pub const GWLP_USERDATA: c_int = -21;

/// The regular "arrow" cursor resource index.
pub const IDC_ARROW: LPCWSTR = MAKEINITRESOURCEW(32512);

/// The id of the "Ok" button on a message box.
pub const IDOK: c_int = 1;

/// Display "Ok" and "Cancel" buttons on a message box.
pub const MB_OKCANCEL: u32 = 1;

pub const SW_SHOW: c_int = 5;

pub const WS_OVERLAPPED: u32 = 0x00000000;
pub const WS_CAPTION: u32 = 0x00C00000;
pub const WS_SYSMENU: u32 = 0x00080000;
pub const WS_THICKFRAME: u32 = 0x00040000;
pub const WS_MINIMIZEBOX: u32 = 0x00020000;
pub const WS_MAXIMIZEBOX: u32 = 0x00010000;
pub const WS_OVERLAPPEDWINDOW: u32 =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;

/// Sent when the window is closed.
pub const WM_CLOSE: u32 = 0x0010;
/// Sent when an application requests a window be created. The window procedure will receive this
/// message after the window is created, but before the window becomes visible.
pub const WM_CREATE: u32 = 0x0001;
/// Sent when the window is being destroyed.
pub const WM_DESTROY: u32 = 0x0002;
/// Sent prior to the [`WM_CREATE`] message when a window is first created.
pub const WM_NCCREATE: u32 = 0x0081;
/// Sent when the window should be painted.
pub const WM_PAINT: u32 = 0x000F;

fn main() {
    let h_instance = unsafe { GetModuleHandleW(ptr::null()) };
    let sample_window_class_wn = wide_null("Sample Window Class");

    let wc = WNDCLASSW {
        lpfnWndProc: Some(window_procedure),
        hInstance: h_instance,
        lpszClassName: sample_window_class_wn.as_ptr(),
        hCursor: unsafe { LoadCursorW(ptr::null_mut(), IDC_ARROW) },
        ..Default::default()
    };

    let atom = unsafe { RegisterClassW(&wc) };

    if atom == 0 {
        let last_error = unsafe { GetLastError() };
        panic!("Could not register the window class, error code {last_error:#x}",);
    }

    let sample_window_name_wn = wide_null("Sample Window Name");

    // This is data to pass to the window, which the window procedure can handle in its WM_CREATE
    // or WM_NCCREATE message handlers.
    // Note that we intentionally Box::leak the data - it should be cleaned up by the window procedure
    // in is WM_DESTROY message handler.
    let lparam: *mut i32 = Box::leak(Box::new(5_i32));

    let hwnd = unsafe {
        CreateWindowExW(
            0,
            sample_window_class_wn.as_ptr(),
            sample_window_name_wn.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            ptr::null_mut(),
            ptr::null_mut(),
            h_instance,
            lparam.cast(),
        )
    };
    if hwnd.is_null() {
        panic!("Failed to create a window.");
    }

    let _previously_visible = unsafe { ShowWindow(hwnd, SW_SHOW) };

    let mut msg = MSG::default();
    loop {
        let message_return = unsafe { GetMessageW(&mut msg, ptr::null_mut(), 0, 0) };

        if message_return == 0 {
            break;
        } else if message_return == -1 {
            let last_error = unsafe { GetLastError() };
            panic!("Error with `GetMessageW`, error code: {last_error:#x}");
        }

        unsafe {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

/// The main window procedure.
///
/// # Safety
///
/// - Runs in a different thread (potentially)
/// - Uses weird calling conventions
/// - Hates you
pub unsafe extern "system" fn window_procedure(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        // This message is sent *just* before the WM_CREATE message, when the window is first created.
        // If window creation should procede, return TRUE (1). Otherwise, return FALSE (0).
        WM_NCCREATE => {
            println!("NC Create");

            // We expect to receive window creation data in l_param when processing this message. If
            // we don't receive it, disallow window creation and exit.
            let createstruct: LPCREATESTRUCTW = lparam as _;
            if createstruct.is_null() {
                return 0;
            }

            let boxed_i32_ptr: *mut i32 = (*createstruct).lpCreateParams.cast();
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, boxed_i32_ptr as LONG_PTR);

            return 1;
        }

        // The window is being created. Application state should be setup here.
        //
        // Per MSDN:
        //
        // > If an application processes this message, it should return zero to continue creation of
        // > the window. If the application returns –1, the window is destroyed and the
        // > CreateWindowEx or CreateWindow function returns a NULL handle.
        WM_CREATE => {
            println!("Create");
        }

        // Paint the window's client area.
        WM_PAINT => {
            let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut i32;
            println!("Current ptr: {}", *ptr);
            *ptr += 1;

            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);

            // Just fill the background with the default window color
            let _success = FillRect(hdc, &ps.rcPaint, (COLOR_WINDOW + 1) as HBRUSH);

            EndPaint(hwnd, &ps);
        }

        // Destroy the window class when told to close.
        WM_CLOSE => {
            DestroyWindow(hwnd);
        }
        // Tell the system the application quit upon window class destruction.
        WM_DESTROY => {
            // Remember to clean up application state upon destruction!
            let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut i32;

            Box::from_raw(ptr); // by not saving the box, it immediately gets dropped and the
                                // application state deallocated.
            println!("Deallocated application state!");

            PostQuitMessage(0);
        }

        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    }

    0
}

/// Turns a Rust string slice into a null-terminated utf-16 vector.
pub fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}
