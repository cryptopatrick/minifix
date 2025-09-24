/// Configuration constants for field validation.
/// This module consolidates validation ranges and limits to reduce code duplication
/// and provide centralized configuration for field validation rules.

/// Time field validation configuration.
#[derive(Debug)]
pub struct TimeValidation;

impl TimeValidation {
    /// Minimum valid hour value (0).
    pub const HOUR_MIN: u32 = 0;
    /// Maximum valid hour value (23).
    pub const HOUR_MAX: u32 = 23;
    /// Minimum valid minute value (0).
    pub const MINUTE_MIN: u32 = 0;
    /// Maximum valid minute value (59).
    pub const MINUTE_MAX: u32 = 59;
    /// Minimum valid second value (0).
    pub const SECOND_MIN: u32 = 0;
    /// Maximum valid second value (60, including leap seconds).
    pub const SECOND_MAX: u32 = 60; // Leap seconds
    /// Minimum valid millisecond value (0).
    pub const MILLISECOND_MIN: u32 = 0;
    /// Maximum valid millisecond value (999).
    pub const MILLISECOND_MAX: u32 = 999;
}

/// Date field validation configuration.
#[derive(Debug)]
pub struct DateValidation;

impl DateValidation {
    /// Minimum valid year value (0).
    pub const YEAR_MIN: u32 = 0;
    /// Maximum valid year value (9999).
    pub const YEAR_MAX: u32 = 9999;
    /// Minimum valid month value (1).
    pub const MONTH_MIN: u32 = 1;
    /// Maximum valid month value (12).
    pub const MONTH_MAX: u32 = 12;
    /// Minimum valid day value (1).
    pub const DAY_MIN: u32 = 1;
    /// Maximum valid day value (31).
    pub const DAY_MAX: u32 = 31;
}

/// Field length configuration for various FIX data types.
#[derive(Debug)]
pub struct FieldLengths;

impl FieldLengths {
    /// Expected byte length for DATE fields (YYYYMMDD = 8 bytes).
    pub const DATE_BYTES: usize = 8;
    /// Expected byte length for TIME fields without milliseconds (HH:MM:SS = 8 bytes).
    pub const TIME_BYTES_NO_MILLI: usize = 8;
    /// Expected byte length for TIME fields with milliseconds (HH:MM:SS.mmm = 12 bytes).
    pub const TIME_BYTES_WITH_MILLI: usize = 12;
    /// Expected byte length for MONTHYEAR fields (YYYYMM + optional week/day = 8 bytes).
    pub const MONTHYEAR_BYTES: usize = 8;
    /// Expected byte length for CHECKSUM fields (3 bytes).
    pub const CHECKSUM_BYTES: usize = 3;
}
