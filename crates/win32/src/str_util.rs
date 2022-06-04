/// Turns a Rust string slice into a null-terminated utf-16 vector.
pub fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

/// Converts a `Vec<u8>` into a `String` using the minimum amount of re-allocation.
///
/// Specifically, the `Vec<u8>`'s data is moved into the `String` if (and only if) it already is
/// valid UTF-8. Otherwise a new `String` is allocated and data is copied from the `Vec<u8>`.
pub fn min_alloc_lossy_into_string(bytes: Vec<u8>) -> String {
    match String::from_utf8(bytes) {
        Ok(s) => s,
        Err(e) => String::from_utf8_lossy(e.as_bytes()).into_owned(),
    }
}

/// Given a buffer of UTF-8 encoded bytes, break off one code point worth of bytes and return it
/// along with the remaining bytes.
///
/// **See:** [UTF-8 encoding reference on Wikipedia](https://en.wikipedia.org/wiki/UTF-8#Encoding)
///
/// ```
/// # use triangle_from_scratch::str_util::break_off_code_point;
/// // code point gets lopped off of a string
/// let test_str = "𐍈rigin".to_string();
/// assert_eq!(
///     break_off_code_point(test_str.as_bytes()),
///     Some(('𐍈' as u32, &['r' as u8, 'i' as u8, 'g' as u8, 'i' as u8, 'n' as u8][..]))
/// );
/// ```
pub const fn break_off_code_point(utf8: &[u8]) -> Option<(u32, &[u8])> {
    match utf8 {
        // Per the UTF-8 spec, for a multi-byte sequence the number of initial 1 bits is the number
        // of *total* bytes in the sequence.

        // One-byte sequences:
        [a @ 0b0000_0000..=0b0111_1111, rest @ ..] => Some((*a as u32, rest)),

        // Two-byte sequences:
        [a @ 0b1100_0000..=0b1101_1111, b, rest @ ..] => {
            let lead = (*a & 0b0001_1111) as u32;
            let trail = (*b & 0b0011_1111) as u32;
            Some((lead << 6 | trail, rest))
        }

        // Three-byte sequences:
        [a @ 0b1110_0000..=0b1110_1111, b, c, rest @ ..] => {
            let lead = (*a & 0b0000_1111) as u32;
            let trail1 = (*b & 0b0011_1111) as u32;
            let trail2 = (*c & 0b0011_1111) as u32;
            let out = lead << 12 | trail1 << 6 | trail2;
            Some((out, rest))
        }

        // Four-byte sequences:
        [a @ 0b1111_0000..=0b1111_0111, b, c, d, rest @ ..] => {
            let lead = (*a & 0b0000_0111) as u32;
            let trail1 = (*b & 0b0011_1111) as u32;
            let trail2 = (*c & 0b0011_1111) as u32;
            let trail3 = (*d & 0b0011_1111) as u32;
            let out = lead << 18 | trail1 << 12 | trail2 << 6 | trail3;
            Some((out, rest))
        }

        // If we can't find a code point in the input, return None.
        [] => None,

        // If we can't match anything above, pull off one byte and give the unicode replacement
        // code point (U+FFFD, �)
        [_unknown, rest @ ..] => Some(('�' as u32, rest)),
    }
}

/// Counts the number of UTF-16 code units present in a UTF-8-encoded string.
pub const fn count_utf16_code_units(utf8: &str) -> usize {
    let mut bytes = utf8.as_bytes();
    let mut len = 0;
    while let Some((u, rest)) = break_off_code_point(bytes) {
        len += if u <= 0xFFFF { 1 } else { 2 };
        bytes = rest;
    }
    len
}

/// Converts a normal Rust UTF-8-encoded string literal to an array of UTF-16-encoded bytes, at
/// compile time.
///
/// **Note:** This macro does _not_ add a null terminator to the end of the string. See [`utf16_null`]
/// for that functionality.
#[macro_export]
macro_rules! utf16 {
    ($text:expr) => {{
        // Save $text in a local const with a name unlikely to exist in the scope $text came from,
        // which prevents a potential const eval cycle error.
        const __TRIANGLE_FROM_SCRATCH_STR_UTIL_UTF16_MACRO_A1B2C3D4_CONST_EVAL_LOOP_BREAK: &str =
            $text;
        const UTF8: &str =
            __TRIANGLE_FROM_SCRATCH_STR_UTIL_UTF16_MACRO_A1B2C3D4_CONST_EVAL_LOOP_BREAK;

        const OUT_BUFFER_LEN: usize = $crate::str_util::count_utf16_code_units(UTF8);

        const UTF16: [u16; OUT_BUFFER_LEN] = {
            let mut buffer = [0u16; OUT_BUFFER_LEN];
            let mut bytes = UTF8.as_bytes();
            let mut i = 0;

            while let Some((u, rest)) = $crate::str_util::break_off_code_point(bytes) {
                if u <= 0xFFFF {
                    buffer[i] = u as u16;
                    i += 1;
                } else {
                    let code = u - 0x1_0000;
                    buffer[i] = 0xD800 | ((code >> 10) as u16);
                    buffer[i + 1] = 0xDC00 | ((code & 0x03FF) as u16);
                    i += 2;
                }
                bytes = rest;
            }
            buffer
        };

        UTF16
    }};
}

/// Converts UTF-8 string literals to arrays of UTF-16 code units, as per [`utf16`], but places
/// a null-terminator at the end.
///
/// **Note**: This macro can only be passed string _literals_ (not variables or constants!) due to
/// its internal use of [`concat`] to add a null byte to the end of the string. If you want to
/// convert a variable or constant to UTF-16, use the [`utf16`] macro instead and add the null-terminating
/// byte yourself.
#[macro_export]
macro_rules! utf16_null {
    ($text:expr) => {{
        const __TRIANGLE_FROM_SCRATCH_STR_UTIL_UTF16_NULL_MACRO_A1B2C3D4_CONST_EVAL_LOOP_BREAK:
            &str = concat!($text, '\0');
        $crate::utf16!(
            __TRIANGLE_FROM_SCRATCH_STR_UTIL_UTF16_NULL_MACRO_A1B2C3D4_CONST_EVAL_LOOP_BREAK
        )
    }};
}

/// Convert a UTF-8 rust string literal into a null-terminated `&[u8]`.
///
/// **Note**: This macro can only be passed string _literals_ (not variables or constants!) due to
/// its internal use of [`concat`] to add a null byte to the end of the string.
#[macro_export]
macro_rules! c_str {
    ($text:expr) => {{
        concat!($text, '\0').as_bytes()
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    mod break_off_code_point {
        use super::*;

        #[test]
        fn works_with_multiple_sizes() {
            // code points of 1, 2, 3, and 4 byte size
            for ch in &['$', '¢', 'ह', '€', '한', '𐍈'] {
                let s = format!("{}", ch);
                assert_eq!(
                    break_off_code_point(s.as_bytes()),
                    Some((*ch as u32, &[][..]))
                );
            }
        }

        #[test]
        fn handles_empty_strings() {
            // empty string works properly
            assert!(break_off_code_point("".as_bytes()).is_none());
        }
    }

    mod count_utf16_code_units {
        use super::*;

        #[test]
        fn works_with_ascii() {
            let s = "hello from the unit test";
            let normal_style: usize = s.chars().map(|ch| ch.len_utf16()).sum();
            assert_eq!(normal_style, count_utf16_code_units(s));
        }

        #[test]
        fn works_with_international_symbols() {
            let s = "$¢ह€한𐍈, 漢字, ひらがな / 平仮名, カタカナ / 片仮名";
            let normal_style: usize = s.chars().map(|ch| ch.len_utf16()).sum();
            assert_eq!(normal_style, count_utf16_code_units(s));
        }
    }

    mod utf16 {
        #[test]
        fn basic_usage() {
            const HELLO16: [u16; 5] = utf16!("hello");
            assert_eq!(&HELLO16[..], &"hello".encode_utf16().collect::<Vec<u16>>());
        }

        #[test]
        fn works_with_international_symbols() {
            const WORDS8: &str = "$¢ह€한𐍈, 漢字, ひらがな / 平仮名, カタカナ / 片仮名";
            const WORDS16: &[u16] = &utf16!(WORDS8);
            assert_eq!(WORDS16, &WORDS8.encode_utf16().collect::<Vec<u16>>());
        }
    }

    mod utf16_null {
        #[test]
        fn basic_usage() {
            const HELLO16: [u16; 6] = utf16_null!("hello");
            assert_eq!(
                &HELLO16[..],
                &"hello\0".encode_utf16().collect::<Vec<u16>>()
            );
        }

        #[test]
        fn works_with_international_symbols() {
            const WORDS8_WITH_NULL: &str = "$¢ह€한𐍈, 漢字, ひらがな / 平仮名, カタカナ / 片仮名\0";
            const WORDS16: &[u16] =
                &utf16_null!("$¢ह€한𐍈, 漢字, ひらがな / 平仮名, カタカナ / 片仮名");
            assert_eq!(
                WORDS16,
                &WORDS8_WITH_NULL.encode_utf16().collect::<Vec<u16>>()
            );
        }
    }
}
