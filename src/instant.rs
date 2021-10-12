use crate::duration::Duration;
use crate::helpers::{self, Helpers};
use core::cmp::Ordering;
use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Instant<const NOM: u32, const DENOM: u32> {
    pub ticks: u32,
}

impl<const NOM: u32, const DENOM: u32> Instant<NOM, DENOM> {
    pub const fn new(ticks: u32) -> Self {
        helpers::greater_than_0::<NOM>();
        helpers::greater_than_0::<DENOM>();

        Instant { ticks }
    }

    pub const fn ticks(&self) -> u32 {
        self.ticks
    }

    pub const fn checked_duration_since(self, other: Self) -> Option<Duration<NOM, DENOM>> {
        if let Some(v) = self.ticks.checked_sub(other.ticks) {
            Some(Duration::new(v))
        } else {
            None
        }
    }

    pub const fn checked_add_duration<const O_NOM: u32, const O_DENOM: u32>(
        self,
        other: Duration<O_NOM, O_DENOM>,
    ) -> Option<Self> {
        if Helpers::<NOM, DENOM, O_NOM, O_DENOM>::SAME_BASE {
            if let Some(ticks) = self.ticks.checked_add(other.ticks()) {
                Some(Instant::new(ticks))
            } else {
                None
            }
        } else {
            if let Some(lh) = other
                .ticks()
                .checked_mul(Helpers::<NOM, DENOM, O_NOM, O_DENOM>::LD_TIMES_RN)
            {
                let ticks = lh / Helpers::<NOM, DENOM, O_NOM, O_DENOM>::RD_TIMES_LN;

                if let Some(ticks) = self.ticks.checked_add(ticks) {
                    Some(Instant::new(ticks))
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}

impl<const NOM: u32, const DENOM: u32> PartialOrd for Instant<NOM, DENOM> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.ticks.partial_cmp(&other.ticks)
    }
}

impl<const NOM: u32, const DENOM: u32> Ord for Instant<NOM, DENOM> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ticks.cmp(&other.ticks)
    }
}

impl<const NOM: u32, const DENOM: u32> PartialEq for Instant<NOM, DENOM> {
    fn eq(&self, other: &Self) -> bool {
        self.ticks.eq(&other.ticks)
    }
}

impl<const NOM: u32, const DENOM: u32> Eq for Instant<NOM, DENOM> {}

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
    ops::Add<Duration<R_NOM, R_DENOM>> for Instant<L_NOM, L_DENOM>
{
    type Output = Instant<L_NOM, L_DENOM>;

    fn add(self, other: Duration<R_NOM, R_DENOM>) -> Self::Output {
        if let Some(v) = self.checked_add_duration(other) {
            v
        } else {
            panic!("Add failed! Overflow");
        }
    }
}
