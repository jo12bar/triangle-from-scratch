//! Win32 structures, and their associated typedefs (where defined).

use super::typedefs::*;

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
