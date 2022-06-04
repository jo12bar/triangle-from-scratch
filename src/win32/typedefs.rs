//! Basic Win32 type definitions. More complicated structs are found in [`super::structs`].

use crate::c_types::*;

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
pub type BOOL = CInt;

/// A byte (8 bits).
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#byte), this
/// is defined in WinDef.h as follows:
///
/// ```c
/// typedef unsigned char BYTE;
/// ```
pub type BYTE = u8;

/// A 32-bit unsigned integer. The range is 0 through 4294967295 decimal.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types), this is
/// defined in IntSafe.h as follows:
///
/// ```c
/// typedef unsigned long DWORD;
/// ```
pub type DWORD = CULong;

/// Pointer to a procedure of unknown type.
pub type FARPROC = *mut core::ffi::c_void;

/// Win32 float definition
pub type FLOAT = CFloat;

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

/// A handle to a GL rendering context.
pub type HGLRC = HANDLE;

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

/// A handle to a local memory block.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hlocal),
/// this is defined in WinDef.h as follows:
///
/// ```c
/// typedef HANDLE HLOCAL;
/// ```
pub type HLOCAL = HANDLE;

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
pub type LONG = CLong;

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

/// A pointer to an ANSI string.
///
/// For UTF-16 strings, use [`LPCWSTR`] instead.
pub type LPCSTR = *const CChar;

/// A pointer to a constant value of any type.
///
/// [Per MSDN](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#lpcvoid),
/// this is defined in WinDef.h as follows:
///
/// ```c
/// typedef const void *LPCVOID;
/// ```
pub type LPCVOID = *const core::ffi::c_void;

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

/// Pointer to a procedure of unknown type.
pub type PROC = *mut core::ffi::c_void;

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
pub type UINT = CUInt;

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

/// A language and implementation-specific structure for passing a variadic number
/// of parameters to a function - for instance, to [`FormatMessageW`][msdn-format-message-w].
/// For now, we just represent it as a mutable buffer of `char`'s.
///
/// [msdn-format-message-w]: https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew
pub type va_list = *mut CChar;

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
pub type WORD = CUShort;
