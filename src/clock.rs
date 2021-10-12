use crate::instant::Instant;

mod sealed {
    pub trait TimeInt {}
}

impl sealed::TimeInt for u32 {}
// impl sealed::TimeInt for u64 {}

pub trait Clock<const NOM: u32, const DENOM: u32>: Sized {
    /// The type to hold the tick count
    type T: sealed::TimeInt;

    /// Get the current Instant
    fn now(&self) -> Instant<NOM, DENOM>;
}
