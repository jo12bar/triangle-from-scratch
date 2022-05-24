//! Basic Win32 type definitions. More complicated structs are found in [`super::structs`].

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