//! Bindings to Win32 structs, types, and functions.

// Win32 names are very incompatible with Rust and Clippy's default lints, so
// we have to disable some of them.
#![allow(clippy::upper_case_acronyms, non_snake_case, non_camel_case_types)]

use core::ptr;

pub mod c_macros;
pub mod constants;
pub mod extern_bindings;
pub mod structs;
pub mod typedefs;

pub use c_macros::*;
pub use constants::*;
pub use extern_bindings::*;
pub use structs::*;
pub use typedefs::*;

/// Returns a handle to the file used to create the calling process (.exe file).
///
/// See [`GetModuleHandleW` on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew).
pub fn get_process_handle() -> HMODULE {
    // Safety: as per the MSDN docs, passing a nullptr to this function returns the ifle used to
    // create the calling process.
    unsafe { GetModuleHandleW(ptr::null()) }
}
