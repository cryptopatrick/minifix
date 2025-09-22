/// Configuration constants for field validation.
/// This module consolidates validation ranges and limits to reduce code duplication
/// and provide centralized configuration for field validation rules.

/// Time field validation configuration.
#[derive(Debug)]
pub struct TimeValidation;

impl TimeValidation {
    pub const HOUR_MIN: u32 = 0;
    pub const HOUR_MAX: u32 = 23;
    pub const MINUTE_MIN: u32 = 0;
    pub const MINUTE_MAX: u32 = 59;
    pub const SECOND_MIN: u32 = 0;
    pub const SECOND_MAX: u32 = 60; // Leap seconds
    pub const MILLISECOND_MIN: u32 = 0;
    pub const MILLISECOND_MAX: u32 = 999;
}

/// Date field validation configuration.
#[derive(Debug)]
pub struct DateValidation;

impl DateValidation {
    pub const YEAR_MIN: u32 = 0;
    pub const YEAR_MAX: u32 = 9999;
    pub const MONTH_MIN: u32 = 1;
    pub const MONTH_MAX: u32 = 12;
    pub const DAY_MIN: u32 = 1;
    pub const DAY_MAX: u32 = 31;
}

/// Field length configuration for various FIX data types.
#[derive(Debug)]
pub struct FieldLengths;

impl FieldLengths {
    pub const DATE_BYTES: usize = 8;
    pub const TIME_BYTES_NO_MILLI: usize = 8;
    pub const TIME_BYTES_WITH_MILLI: usize = 12;
    pub const MONTHYEAR_BYTES: usize = 8;
    pub const CHECKSUM_BYTES: usize = 3;
}
