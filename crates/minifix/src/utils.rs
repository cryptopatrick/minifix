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
    pub const fn validate_time_components(hour: u32, minute: u32, second: u32, milli: u32) -> bool {
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
}
