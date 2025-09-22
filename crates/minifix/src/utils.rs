#![macro_use]

/// A handly macro for quick and dirty debugging. It reports the caller location
/// in the form of file plus line, and it also supports `format!` -like arguments.
#[allow(unused_macros)]
macro_rules! dbglog {
    ($($arg:tt)*) => {{
        if std::cfg!(debug_assertions) {
            std::eprintln!("[{}:{}] {}", std::file!(), std::line!(), std::format!($($arg)*));
        }
    }}
}

/// Utility functions for field validation.
pub mod validation {
    /// Validates that all values are within their respective ranges.
    pub const fn validate_time_components(
        hour: u32,
        minute: u32,
        second: u32,
        milli: u32,
    ) -> bool {
        hour <= 23 && minute <= 59 && second <= 60 && milli <= 999
    }
}

/// Utility functions for FIX field encoding.
pub mod encoding {
    /// Converts a number to ASCII digits with zero-padding.
    ///
    /// # Examples
    ///
    /// ```
    /// # use minifix::utils::encoding::digits_to_ascii;
    /// assert_eq!(digits_to_ascii::<4>(42), [b'0', b'0', b'4', b'2']);
    /// assert_eq!(digits_to_ascii::<3>(123), [b'1', b'2', b'3']);
    /// ```
    pub const fn digits_to_ascii<const N: usize>(mut num: u32) -> [u8; N] {
        let mut result = [b'0'; N];
        let mut i = N;

        if num == 0 {
            return result;
        }

        while num > 0 && i > 0 {
            i -= 1;
            result[i] = (num % 10) as u8 + b'0';
            num /= 10;
        }

        result
    }

    /// Converts a two-digit number to ASCII.
    pub const fn two_digits_to_ascii(num: u32) -> [u8; 2] {
        digits_to_ascii::<2>(num)
    }

    /// Converts a four-digit number to ASCII.
    pub const fn four_digits_to_ascii(num: u32) -> [u8; 4] {
        digits_to_ascii::<4>(num)
    }

    /// Checks if a byte represents an ASCII digit (0-9).
    pub const fn is_ascii_digit(byte: u8) -> bool {
        byte >= b'0' && byte <= b'9'
    }

    /// Converts an ASCII digit to u32 with a multiplier.
    /// This function assumes the input byte is a valid ASCII digit.
    pub const fn ascii_digit_to_u32(digit: u8, multiplier: u32) -> u32 {
        (digit as u32).wrapping_sub(b'0' as u32) * multiplier
    }
}

/// Utility functions for parsing byte sequences.
pub mod parsing {
    use memchr::memchr;

    /// Finds the position of a byte within a slice, starting from a given offset.
    pub fn find_byte_from(
        data: &[u8],
        byte: u8,
        offset: usize,
    ) -> Option<usize> {
        if offset >= data.len() {
            return None;
        }
        memchr(byte, &data[offset..]).map(|pos| pos + offset)
    }

    /// Finds the next equal sign '=' in the data starting from an offset.
    pub fn find_equal_sign_from(data: &[u8], offset: usize) -> Option<usize> {
        find_byte_from(data, b'=', offset)
    }

    /// Finds the next separator byte in the data starting from an offset.
    pub fn find_separator_from(
        data: &[u8],
        separator: u8,
        offset: usize,
    ) -> Option<usize> {
        find_byte_from(data, separator, offset)
    }

    /// Parses a decimal number from ASCII digits in a byte slice.
    pub fn parse_decimal_ascii(data: &[u8]) -> Option<u32> {
        let mut result = 0u32;
        for &byte in data {
            if byte < b'0' || byte > b'9' {
                return None;
            }
            result =
                result.wrapping_mul(10).wrapping_add((byte - b'0') as u32);
        }
        Some(result)
    }
}
