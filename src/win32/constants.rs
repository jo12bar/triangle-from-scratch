//! Predefined Win32 constants. These are either C `static` variables, `const` variables, or (more
//! often) `#define` preprocessor macros. This module does not contain any Rusty macro replacements
//! for C preprocessor macros that act like functions. For that, see [`super::c_macros`].

use super::typedefs::*;

pub const CW_USEDEFAULT: c_int = 0x80000000_u32 as c_int;

/// For use with [FormatMessageW][msdn-format-message-w].
///
/// Allocates a buffer large enough to hold the formatted message.
///
/// [msdn-format-message-w]: https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew
pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: DWORD = 0x0000_0100;

/// For use with [FormatMessageW][msdn-format-message-w].
///
/// Indicates the *Arguments* parameter is not a [`va_list`] structure, but is a pointer to an array
/// of values that represent the arguments.
///
/// [msdn-format-message-w]: https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew
pub const FORMAT_MESSAGE_ARGUMENT_ARRAY: DWORD = 0x0000_2000;

/// For use with [FormatMessageW][msdn-format-message-w].
///
/// The _lpSource_ parameter is a module handle containing the message-table resource(s) to search.
/// If this _lpSource_ handle is null, the current process's application image file will be searched.
/// This flag cannot be used with [`FORMAT_MESSAGE_FROM_STRING`].
///
/// [msdn-format-message-w]: https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew
pub const FORMAT_MESSAGE_FROM_HMODULE: DWORD = 0x0000_0800;

/// For use with [FormatMessageW][msdn-format-message-w].
///
/// The _lpSource_ parameter is a pointer to a null-terminated string that contains a message
/// definition. The message definition may contain insert sequences, just as the message text in a
/// message table resource may. This flag cannot be used with [`FORMAT_MESSAGE_FROM_HMODULE`] or
/// [`FORMAT_MESSAGE_FROM_SYSTEM`].
///
/// [msdn-format-message-w]: https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew
pub const FORMAT_MESSAGE_FROM_STRING: DWORD = 0x0000_0400;

/// For use with [FormatMessageW][msdn-format-message-w].
///
/// The function should search the system message-table resource(s) for the requested message. If
/// this flag is specified with [`FORMAT_MESSAGE_FROM_HMODULE`], the function searches the system
/// message table if the message is not found in the module specified by lpSource. This flag cannot
/// be used with [`FORMAT_MESSAGE_FROM_STRING`].
///
/// If this flag is specified, an application can pass the result of the [`GetLastError`][super::GetLastError]
/// function to retrieve the message text for a system-defined error.
///
/// [msdn-format-message-w]: https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew
pub const FORMAT_MESSAGE_FROM_SYSTEM: DWORD = 0x0000_1000;

/// For use with [FormatMessageW][msdn-format-message-w].
///
/// Insert sequences in the message definition such as `%1` are to be ignored and passed through to
/// the output buffer unchanged. This flag is useful for fetching a message for later formatting. If
/// this flag is set, the _Arguments_ parameter is ignored.
///
/// [msdn-format-message-w]: https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew
pub const FORMAT_MESSAGE_IGNORE_INSERTS: DWORD = 0x0000_0200;

/// For use with [`SetWindowLongPtrW()`].
///
/// [From MSDN](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw#parameters):
///
/// > Sets the user data associated with the window. This data is intended for use by the
/// > application that created the window. Its value is initially zero.
pub const GWLP_USERDATA: c_int = -21;

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
/// Indicates a request to termiante the application.
pub const WM_QUIT: u32 = 0x0012;

/// The predefined cursor styles.
pub enum IDCursor {
    /// Standard arrow and small hourglass
    AppStarting = 32650,
    /// Standard arrow
    Arrow = 32512,
    /// Crosshair
    Cross = 32515,
    /// Hand
    Hand = 32649,
    /// Arrow and question mark
    Help = 32651,
    /// I-beam
    IBeam = 32513,
    /// Slashed circle
    No = 32648,
    /// Four-pointed arrow pointing north, south, east, and west
    SizeAll = 32646,
    /// Double-pointed arrow pointing northeast and southwest
    SizeNeSw = 32643,
    /// Double-pointed arrow pointing north and south
    SizeNS = 32645,
    /// Double-pointed arrow pointing northwest and southeast
    SizeNwSe = 32642,
    /// Double-pointed arrow pointing west and east
    SizeWE = 32644,
    /// Vertical arrow
    UpArrow = 32516,
    /// Hourglass
    Wait = 32514,
}

/// Predefined system colours.
///
/// **See:** [`GetSysColor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor)
pub enum SysColor {
    ActiveBorder = 10,
    ActiveCaption = 2,
    AppWorkspace = 12,
    /// Button face, also "3D face" color.
    ButtonFace = 15,
    /// Button highlight, also "3D highlight" color.
    ButtonHighlight = 20,
    /// Button shadow, also "3D shadow" color.
    ButtonShadow = 16,
    ButtonText = 18,
    CaptionText = 9,
    /// AKA "3D dark shadow"
    D3DarkShadow = 21,
    /// AKA "3D light"
    D3Light = 22,
    /// Desktop background color
    Desktop = 1,
    GradientActiveCaption = 27,
    GradientInactiveCaption = 28,
    GrayText = 17,
    Highlight = 13,
    HighlightText = 14,
    HotLight = 26,
    InactiveBorder = 11,
    InactiveCaption = 3,
    InactiveCaptionText = 19,
    InfoBackground = 24,
    InfoText = 23,
    Menu = 4,
    MenuHighlight = 29,
    MenuBar = 30,
    MenuText = 7,
    ScrollBar = 0,
    Window = 5,
    WindowFrame = 6,
    WindowText = 8,
}
