//! Rusty replacements for Win32 C preprocessor macros.

use super::typedefs::*;

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
