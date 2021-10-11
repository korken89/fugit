use crate::duration::Duration;
use std::ops;

#[derive(Clone, Copy)]
pub struct Instant<const NOM: u32, const DENOM: u32> {
    pub ticks: u32,
}

impl<const NOM: u32, const DENOM: u32> Instant<NOM, DENOM> {
    pub const fn checked_duration_since(self, other: Self) -> Option<Duration<NOM, DENOM>> {
        if let Some(v) = self.ticks.checked_sub(other.ticks) {
            Some(Duration::new(v))
        } else {
            None
        }
    }
}

// Instant - Instant = Instant
impl<const NOM: u32, const DENOM: u32> ops::Sub<Instant<NOM, DENOM>> for Instant<NOM, DENOM> {
    type Output = Duration<NOM, DENOM>;

    fn sub(self, other: Self) -> Self::Output {
        if let Some(v) = self.checked_duration_since(other) {
            v
        } else {
            panic!("Sub failed! Other > self");
        }
    }
}

// Instant + Duration = Instant
impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
    ops::Add<Duration<R_NOM, R_DENOM>> for Instant<L_NOM, L_DENOM> {
    type Output = Instant<L_NOM, L_DENOM>;

    fn add(self, other: Duration<R_NOM, R_DENOM>) -> Self::Output {
        todo!()
    }
}
