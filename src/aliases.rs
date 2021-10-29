//! Type aliases for common uses

use crate::duration::Duration;

/// Alias for microsecond duration
pub type MicrosDuration<T> = Duration<T, 1, 1_000_000>;

/// Alias for microsecond duration (`u32` backing storage)
pub type MicrosDurationU32 = Duration<u32, 1, 1_000_000>;

/// Alias for microsecond duration (`u64` backing storage)
pub type MicrosDurationU64 = Duration<u64, 1, 1_000_000>;

/// Alias for millisecond duration
pub type MillisDuration<T> = Duration<T, 1, 1_000>;

/// Alias for millisecond duration (`u32` backing storage)
pub type MillisDurationU32 = Duration<u32, 1, 1_000>;

/// Alias for millisecond duration (`u64` backing storage)
pub type MillisDurationU64 = Duration<u64, 1, 1_000>;

/// Alias for second duration
pub type SecsDuration<T> = Duration<T, 1, 1>;

/// Alias for second duration (`u32` backing storage)
pub type SecsDurationU32 = Duration<u32, 1, 1>;

/// Alias for second duration (`u64` backing storage)
pub type SecsDurationU64 = Duration<u64, 1, 1>;

/// Alias for minutes duration
pub type MinsDuration<T> = Duration<T, 60, 1>;

/// Alias for minutes duration (`u32` backing storage)
pub type MinsDurationU32 = Duration<u32, 60, 1>;

/// Alias for minutes duration (`u64` backing storage)
pub type MinsDurationU64 = Duration<u64, 60, 1>;

/// Alias for hours duration
pub type HoursDuration<T> = Duration<T, 3_600, 1>;

/// Alias for hours duration (`u32` backing storage)
pub type HoursDurationU32 = Duration<u32, 3_600, 1>;

/// Alias for hours duration (`u64` backing storage)
pub type HoursDurationU64 = Duration<u64, 3_600, 1>;
