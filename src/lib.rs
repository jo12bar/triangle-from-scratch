pub mod str_util;
#[cfg(windows)]
pub mod win32;

pub use str_util::wide_null;

/// Gathers up the bytes from a buffer into a vector, copying them.
///
/// ## Safety
///
/// The byte sequence must be null-terminated. Otherwise, the vector will continue accumulating
/// bytes until either a null byte is reached or the program segfaults.
///
/// The output excludes the terminating null byte.
pub unsafe fn gather_null_terminated_bytes(mut p: *const u8) -> Vec<u8> {
    let mut v = vec![];
    while *p != 0 {
        v.push(*p);
        p = p.add(1);
    }
    v
}
