//! Bindings to Win32 structs, types, and functions.

// Win32 names are very incompatible with Rust and Clippy's default lints, so
// we have to disable some of them.
#![allow(clippy::upper_case_acronyms, non_snake_case, non_camel_case_types)]

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
pub type ATOM = WORD;

/// In Windows, booleans are actually 16-bit ints.
pub type BOOL = c_int;

/// A byte (8 bits).
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#byte), this
/// is defined in WinDef.h as follows:
///
/// ```c
/// typedef unsigned char BYTE;
/// ```
pub type BYTE = u8;

/// The C representation of an `int` on x86.
pub type c_int = i32;

/// The C representation of a `long` on x86.
pub type c_long = i32;

/// The C representation of an `unsigned int` on x86.
pub type c_uint = u32;

/// The C representation of an `unsigned long` on x86.
pub type c_ulong = u32;

/// The C representation of an `unsigned short` on x86.
pub type c_ushort = u16;

/// A 32-bit unsigned integer. The range is 0 through 4294967295 decimal.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in IntSafe.h as follows:
///
/// ```c
/// typedef unsigned long DWORD;
/// ```
pub type DWORD = c_ulong;

/// A handle to a win32 object.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinNT.h as follows:
///
/// ```c
/// typedef PVOID HANDLE;
/// ```
pub type HANDLE = PVOID;

/// A handle to a [brush](https://docs.microsoft.com/en-us/windows/win32/gdi/brushes).
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinDef.h as follows:
///
/// ```c
/// typedef HANDLE HBRUSH;
/// ```
pub type HBRUSH = HANDLE;

/// A handle to a [cursor](https://docs.microsoft.com/en-us/windows/win32/menurc/cursors).
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinDef.h as follows:
///
/// ```c
/// typedef HICON HCURSOR;
/// ```
pub type HCURSOR = HICON;

/// A handle to a device context (DC).
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdc), this
/// is defined in WinDef.h as follows:
///
/// ```c
/// typedef HANDLE HDC;
/// ```
pub type HDC = HANDLE;

/// A handle to an [icon](https://docs.microsoft.com/en-us/windows/win32/menurc/icons).
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinDef.h as follows:
///
/// ```c
/// typedef HANDLE HICON;
/// ```
pub type HICON = HANDLE;

/// A handle to a instance. This is the base address of the module in memory.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinDef.h as follows:
///
/// ```c
/// typedef HANDLE HINSTANCE;
/// ```
pub type HINSTANCE = HANDLE;

/// A handle to a [menu](https://docs.microsoft.com/en-us/windows/desktop/menurc/menus).
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinDef.h as follows:
///
/// ```c
/// typedef HANDLE HMENU;
/// ```
pub type HMENU = HANDLE;

/// On 16-bit Windows, HMODULE and HINSTANCE were different types. Now they're the same thing.
pub type HMODULE = HINSTANCE;

/// A handle to a [window](https://docs.microsoft.com/en-us/windows/win32/winmsg/windows).
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinDef.h as follows:
///
/// ```c
/// typedef HANDLE HWND;
/// ```
pub type HWND = HANDLE;

/// A 32-bit signed integer. See [MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#long).
pub type LONG = c_long;

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
pub type LONG_PTR = isize;

/// A message parameter.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinDef.h as follows:
///
/// ```c
/// typedef LONG_PTR LPARAM;
/// ```
pub type LPARAM = LONG_PTR;

/// A pointer to a constant null-terminated string of 16-bit Unicode characters.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinDef.h as follows:
///
/// ```c
/// typedef CONST WCHAR *LPCWSTR;
/// ```
pub type LPCWSTR = *const WCHAR;

/// A pointer to any type. Basically a c-style void pointer.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinDef.h as follows:
///
/// ```c
/// typedef void *LPVOID;
/// ```
pub type LPVOID = *mut core::ffi::c_void;

/// A pointer to a null-terminated string of 16-bit UTF16 Unicode chars.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinNT.h as follows:
///
/// ```c
/// typedef WCHAR *LPWSTR;
/// ```
pub type LPWSTR = *mut WCHAR;

/// The signed result of message processing.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinDef.h as follows:
///
/// ```c
/// typedef LONG_PTR LRESULT;
/// ```
pub type LRESULT = LONG_PTR;

/// A pointer to any type - basically a c-style void pointer.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinNT.h as follows:
///
/// ```c
/// typedef void *PVOID;
/// ```
pub type PVOID = *mut core::ffi::c_void;

/// Corresponds to [the following typedef in windows.h](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/52ddd4c3-55b9-4e03-8287-5392aac0627f):
///
/// ```c
/// typedef unsigned int UINT;
/// ```
pub type UINT = c_uint;

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
pub type UINT_PTR = usize;

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
pub type ULONG_PTR = usize;

/// A 16-bit Unicode character.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinNT.h as follows:
///
/// ```c
/// typedef wchar_t WCHAR;
/// ```
pub type WCHAR = wchar_t;

/// A "wide character", equivalent to a 16-bit number.
pub type wchar_t = u16;

/// A message parameter.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in WinDef.h as follows:
///
/// ```c
/// typedef UINT_PTR WPARAM;
/// ```
pub type WPARAM = UINT_PTR;

/// A nullable pointer to a callback function that processes messages sent to a window.
///
/// See [MSDN's explanation](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-wndproc).
pub type WNDPROC = Option<
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
pub type WORD = c_ushort;

/// The [win32 `WNDCLASSW` structure][wndclassw_msdn], used by the [RegisterClass][registerclass_msdn]
/// function to register a new window.
///
/// [wndclassw_msdn]: https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassw
/// [registerclass_msdn]: https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassa
#[repr(C)]
pub struct WNDCLASSW {
    /// The [Class Styles](https://docs.microsoft.com/en-us/windows/desktop/winmsg/about-window-classes)
    /// to apply to this window.
    pub style: UINT,
    /// A pointer to the window procedure.
    pub lpfnWndProc: WNDPROC,
    /// The number of extra bytes to allocate following the window-class structure. The system
    /// initializes the bytes to zero.
    pub cbClsExtra: c_int,
    /// The number of extra bytes to allocate following the window instance. The system initializes
    /// the bytes to zero.
    pub cbWndExtra: c_int,
    /// A handle to the instance that contains the window procedure for the class.
    pub hInstance: HINSTANCE,
    /// A handle to the class icon. This must be a handle to the icon _resource_. If this member
    /// is null, the system provides a default icon.
    pub hIcon: HICON,
    /// A handle to the class cursor. This must be a handle to the cursor _resource_. If this member
    /// is null, an application must explicitly set the cursor shape whenever the mouse moves into
    /// the application's window.
    pub hCursor: HCURSOR,
    /// A handle to the class background brush.
    pub hbrBackground: HBRUSH,
    /// The resource name of the class menu, as the name appears in the resource file.
    pub lpszMenuName: LPCWSTR,
    /// The window class name, with a maximum length of 256.
    pub lpszClassName: LPCWSTR,
}

pub type PWNDCLASSW = *mut WNDCLASSW;
pub type NPWNDCLASSW = *mut WNDCLASSW;
pub type LPWNDCLASSW = *mut WNDCLASSW;

unsafe_impl_default_zeroed! { WNDCLASSW }

/// Defines the initialization parameters passed to the window procedure of an application.
/// The members of this struct are identical to the parameters of the
/// [CreateWindowEx][msdn-create-window-ex] function.
///
/// [msdn-create-window-ex]: https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-createwindowexa
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
    pub lpCreateParams: LPVOID,
    /// A handle to the module that owns the new window.
    pub hInstance: HINSTANCE,
    /// A handle to the menu to be used by the new window.
    pub hMenu: HMENU,
    /// A handle to the parent/owner window, if the window is a child window or owned. If the window
    /// is neither a child window nor owned, this will be a `NULL` handle.
    pub hwndParent: HWND,
    /// The height of the new window, in pixels.
    pub cy: c_int,
    /// The width of the new window, in pixels.
    pub cx: c_int,
    /// The y-coordinate of the upper left corner of the new window. If the window is a child window,
    /// coordinates are relative to the parent window. Otherwise, coordinates are relative to the
    /// screen origin.
    pub y: c_int,
    /// The x-coordinate of the upper left corner of the new window. If the window is a child window,
    /// coordinates are relative to the parent window. Otherwise, coordinates are relative to the
    /// screen origin.
    pub x: c_int,
    /// The [window style](https://docs.microsoft.com/en-us/windows/desktop/winmsg/window-styles)
    /// for the new window.
    pub style: LONG,
    /// The name of the new window.
    pub lpszName: LPCWSTR,
    /// A pointer to a null-terminated string or atom that specifies the class name of the new
    /// window.
    ///
    /// [MSDN remarks](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-createstructw#remarks)
    /// that, since this member can contain a pointer to a local (and thus inacessible) atom, **the
    /// class name shold not be obtained via this member**. The
    /// [`GetClassName`](https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-getclassname)
    /// function should be used instead.
    pub lpszClass: LPCWSTR,
    /// The [extended window style](https://docs.microsoft.com/en-us/windows/desktop/winmsg/extended-window-styles)
    /// for this new window.
    pub dwExStyle: DWORD,
}

pub type LPCREATESTRUCTW = *mut CREATESTRUCTW;

unsafe_impl_default_zeroed! { CREATESTRUCTW }

/// Contains information an application can use to paint the client area of a window it owns.
///
/// [See MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-paintstruct).
#[repr(C)]
pub struct PAINTSTRUCT {
    /// A handle to the display device context to be used for painting.
    pub hdc: HDC,
    /// Indicates whether the background should be erased. The value is nonzero if the application
    /// should erase the background - for example, if the window class is created without a
    /// background brush. See the
    /// [description of the `hbrBackground` member of the `WNDCLASS` structure on MSDN][msdn-wndclass]
    /// for details.
    ///
    /// [msdn-wndclass]: https://docs.microsoft.com/en-us/windows/desktop/api/winuser/ns-winuser-wndclassa
    pub fErase: BOOL,
    /// A rectangle that specifies the upper left and lower right corners of the rectangle in which
    /// the painting is requested, in device units relative to the upper-left corner of the client
    /// area.
    pub rcPaint: RECT,
    /// Reserved by Windows (used internally by the system).
    pub fRestore: BOOL,
    /// Reserved by Windows (used internally by the system).
    pub fIncUpdate: BOOL,
    /// Reserved by Windows (used internally by the system).
    pub rgbReserved: [BYTE; 32],
}

pub type PPAINTSTRUCT = *mut PAINTSTRUCT;
pub type NPPAINTSTRUCT = *mut PAINTSTRUCT;
pub type LPPAINTSTRUCT = *mut PAINTSTRUCT;

unsafe_impl_default_zeroed! { PAINTSTRUCT }

/// Defines the xy-coordinates of a point.
///
/// [See MSDN](https://docs.microsoft.com/en-us/windows/win32/api/windef/ns-windef-point).
#[repr(C)]
pub struct POINT {
    pub x: LONG,
    pub y: LONG,
}

pub type PPOINT = *mut POINT;
pub type NPPOINT = *mut POINT;
pub type LPPOINT = *mut POINT;

unsafe_impl_default_zeroed! { POINT }

/// Defines a rectangle by the coordinates of its upper-left and lower-right corners.
///
/// [See MSDN](https://docs.microsoft.com/en-us/windows/win32/api/windef/ns-windef-rect).
#[repr(C)]
pub struct RECT {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}

pub type PRECT = *mut RECT;
pub type NPRECT = *mut RECT;
pub type LPRECT = *mut RECT;

unsafe_impl_default_zeroed! { RECT }

/// Contains message information from a thread's message queue.
///
/// See [MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-msg).
#[repr(C)]
pub struct MSG {
    /// A handle to the window whose window procedure receives the message. NULL if this message is
    /// a thread message.
    pub hwnd: HWND,
    /// The message identifier. Applications can only use the low work; the high word is reserved
    /// by the system.
    pub message: UINT,
    /// Additional information about the message.
    pub wParam: WPARAM,
    /// Additional information about the message.
    pub lParam: LPARAM,
    /// The time at which the message was posted.
    pub time: DWORD,
    /// The cursor position, in screen coordinates, when the message was posted.
    pub pt: POINT,
    /// Private memory, not to be touched.
    pub lPrivate: DWORD,
}

pub type PMSG = *mut MSG;
pub type NPMSG = *mut MSG;
pub type LPMSG = *mut MSG;

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
