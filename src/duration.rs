use super::helpers::{self, Helpers};
use core::cmp::Ordering;
use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Duration<const NOM: u32, const DENOM: u32> {
    ticks: u32,
}

impl<const NOM: u32, const DENOM: u32> Duration<NOM, DENOM> {
    pub const fn new(ticks: u32) -> Self {
        helpers::greater_than_0::<NOM>();
        helpers::greater_than_0::<DENOM>();

        Duration { ticks }
    }

    pub const fn ticks(&self) -> u32 {
        self.ticks
    }

    pub const fn checked_add<const O_NOM: u32, const O_DENOM: u32>(
        self,
        other: Duration<O_NOM, O_DENOM>,
    ) -> Option<Self> {
        if Helpers::<NOM, DENOM, O_NOM, O_DENOM>::SAME_BASE {
            if let Some(ticks) = self.ticks.checked_add(other.ticks) {
                Some(Duration::new(ticks))
            } else {
                None
            }
        } else {
            if let Some(lh) = other
                .ticks
                .checked_mul(Helpers::<NOM, DENOM, O_NOM, O_DENOM>::LD_TIMES_RN)
            {
                let ticks = lh / Helpers::<NOM, DENOM, O_NOM, O_DENOM>::RD_TIMES_LN;

                if let Some(ticks) = self.ticks.checked_add(ticks) {
                    Some(Duration::new(ticks))
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    pub const fn checked_sub<const O_NOM: u32, const O_DENOM: u32>(
        self,
        other: Duration<O_NOM, O_DENOM>,
    ) -> Option<Self> {
        if Helpers::<NOM, DENOM, O_NOM, O_DENOM>::SAME_BASE {
            if let Some(ticks) = self.ticks.checked_sub(other.ticks) {
                Some(Duration::new(ticks))
            } else {
                None
            }
        } else {
            if let Some(lh) = other
                .ticks
                .checked_mul(Helpers::<NOM, DENOM, O_NOM, O_DENOM>::LD_TIMES_RN)
            {
                let ticks = lh / Helpers::<NOM, DENOM, O_NOM, O_DENOM>::RD_TIMES_LN;

                if let Some(ticks) = self.ticks.checked_sub(ticks) {
                    Some(Duration::new(ticks))
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    pub const fn micros(val: u32) -> Duration<NOM, DENOM> {
        Duration::new(
            (Helpers::<1, 1_000_000, NOM, DENOM>::RD_TIMES_LN * val)
                / Helpers::<1, 1_000_000, NOM, DENOM>::LD_TIMES_RN,
        )
    }

    pub const fn millis(val: u32) -> Duration<NOM, DENOM> {
        Duration::new(
            (Helpers::<1, 1_000, NOM, DENOM>::RD_TIMES_LN * val)
                / Helpers::<1, 1_000, NOM, DENOM>::LD_TIMES_RN,
        )
    }

    pub const fn secs(val: u32) -> Duration<NOM, DENOM> {
        Duration::new(
            (Helpers::<1, 1, NOM, DENOM>::RD_TIMES_LN * val)
                / Helpers::<1, 1, NOM, DENOM>::LD_TIMES_RN,
        )
    }

    pub const fn minutes(val: u32) -> Duration<NOM, DENOM> {
        Duration::new(
            (Helpers::<60, 1, NOM, DENOM>::RD_TIMES_LN * val)
                / Helpers::<60, 1, NOM, DENOM>::LD_TIMES_RN,
        )
    }

    pub const fn hours(val: u32) -> Duration<NOM, DENOM> {
        Duration::new(
            (Helpers::<3_600, 1, NOM, DENOM>::RD_TIMES_LN * val)
                / Helpers::<3_600, 1, NOM, DENOM>::LD_TIMES_RN,
        )
    }
}

impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
    PartialOrd<Duration<R_NOM, R_DENOM>> for Duration<L_NOM, L_DENOM>
{
    fn partial_cmp(&self, other: &Duration<R_NOM, R_DENOM>) -> Option<Ordering> {
        //
        // We want to check:
        //
        // n_lh / d_lh * lh_ticks {cmp} n_rh / d_rh * rh_ticks
        //
        // simplify to
        //
        // n_lh * d_rh * lh_ticks {cmp} n_rh * d_lh * rh_ticks
        //
        // find gdc(n_lh * d_rh, n_rh * d_lh) and use that to make the constants minimal (done
        // with the `helpers::Helpers` struct)
        //
        // then perform the comparison in a comparable basis
        //

        if Helpers::<L_NOM, L_DENOM, R_NOM, R_DENOM>::SAME_BASE {
            // If we are in the same base, comparison in trivial
            Some(self.ticks.cmp(&other.ticks))
        } else {
            Some(
                self.ticks
                    .checked_mul(Helpers::<L_NOM, L_DENOM, R_NOM, R_DENOM>::RD_TIMES_LN)?
                    .cmp(
                        &other
                            .ticks
                            .checked_mul(Helpers::<L_NOM, L_DENOM, R_NOM, R_DENOM>::LD_TIMES_RN)?,
                    ),
            )
        }
    }
}

impl<const NOM: u32, const DENOM: u32> Ord for Duration<NOM, DENOM> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ticks.cmp(&other.ticks)
    }
}

impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
    PartialEq<Duration<R_NOM, R_DENOM>> for Duration<L_NOM, L_DENOM>
{
    fn eq(&self, other: &Duration<R_NOM, R_DENOM>) -> bool {
        if Helpers::<L_NOM, L_DENOM, R_NOM, R_DENOM>::SAME_BASE {
            // If we are in the same base, comparison in trivial
            self.ticks.eq(&other.ticks)
        } else {
            let lh = self
                .ticks
                .checked_mul(Helpers::<L_NOM, L_DENOM, R_NOM, R_DENOM>::RD_TIMES_LN);
            let rh = other
                .ticks
                .checked_mul(Helpers::<L_NOM, L_DENOM, R_NOM, R_DENOM>::LD_TIMES_RN);

            if let (Some(lh), Some(rh)) = (lh, rh) {
                lh == rh
            } else {
                false
            }
        }
    }
}

impl<const NOM: u32, const DENOM: u32> Eq for Duration<NOM, DENOM> {}

// Duration - Duration = Duration
impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
    ops::Sub<Duration<R_NOM, R_DENOM>> for Duration<L_NOM, L_DENOM>
{
    type Output = Duration<L_NOM, L_DENOM>;

    fn sub(self, other: Duration<R_NOM, R_DENOM>) -> Self::Output {
        if let Some(v) = self.checked_sub(other) {
            v
        } else {
            panic!("Add failed!");
        }
    }
}

// Duration + Duration = Duration
impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
    ops::Add<Duration<R_NOM, R_DENOM>> for Duration<L_NOM, L_DENOM>
{
    type Output = Duration<L_NOM, L_DENOM>;

    fn add(self, other: Duration<R_NOM, R_DENOM>) -> Self::Output {
        if let Some(v) = self.checked_add(other) {
            v
        } else {
            panic!("Add failed!");
        }
    }
}

// integer * Duration = Duration
impl<const NOM: u32, const DENOM: u32> ops::Mul<Duration<NOM, DENOM>> for u32 {
    type Output = Duration<NOM, DENOM>;

    fn mul(self, mut other: Duration<NOM, DENOM>) -> Self::Output {
        other.ticks *= self;
        other
    }
}

// Duration * integer = Duration
impl<const NOM: u32, const DENOM: u32> ops::Mul<u32> for Duration<NOM, DENOM> {
    type Output = Duration<NOM, DENOM>;

    fn mul(mut self, other: u32) -> Self::Output {
        self.ticks *= other;
        self
    }
}

// Duration / integer = Duration
impl<const NOM: u32, const DENOM: u32> ops::Div<u32> for Duration<NOM, DENOM> {
    type Output = Duration<NOM, DENOM>;

    fn div(mut self, other: u32) -> Self::Output {
        self.ticks /= other;
        self
    }
}

/// Extension trait for simple short-hands
pub trait ExtU32 {
    fn micros<const NOM: u32, const DENOM: u32>(self) -> Duration<NOM, DENOM>;
    fn millis<const NOM: u32, const DENOM: u32>(self) -> Duration<NOM, DENOM>;
    fn secs<const NOM: u32, const DENOM: u32>(self) -> Duration<NOM, DENOM>;
    fn minutes<const NOM: u32, const DENOM: u32>(self) -> Duration<NOM, DENOM>;
    fn hours<const NOM: u32, const DENOM: u32>(self) -> Duration<NOM, DENOM>;
}

impl ExtU32 for u32 {
    fn micros<const NOM: u32, const DENOM: u32>(self) -> Duration<NOM, DENOM> {
        Duration::micros(self)
    }

    fn millis<const NOM: u32, const DENOM: u32>(self) -> Duration<NOM, DENOM> {
        Duration::millis(self)
    }

    fn secs<const NOM: u32, const DENOM: u32>(self) -> Duration<NOM, DENOM> {
        Duration::secs(self)
    }

    fn minutes<const NOM: u32, const DENOM: u32>(self) -> Duration<NOM, DENOM> {
        Duration::minutes(self)
    }

    fn hours<const NOM: u32, const DENOM: u32>(self) -> Duration<NOM, DENOM> {
        Duration::hours(self)
    }
}
