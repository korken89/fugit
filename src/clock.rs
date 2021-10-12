use crate::instant::Instant;

mod sealed {
    pub trait TimeInt {}
}

impl sealed::TimeInt for u32 {}
// impl sealed::TimeInt for u64 {}

/// The `Clock` trait provides an abstraction for hardware-specific timer peripherals.
///
/// The `Clock` is characterized by an inner unsigned integer storage type (either [`u32`] or
/// [`u64`]) and two const generics which define the ratio of the clock as `NOM / DENOM`.
pub trait Clock<const NOM: u32, const DENOM: u32>: Sized {
    /// The type to hold the tick count
    type T: sealed::TimeInt;

    // TODO: Should instant take a marker to the `Clock`? So instants are marked with the clock
    // from which they come.

    /// Get the current Instant
    fn now(&self) -> Instant<NOM, DENOM>;
}
