use super::helpers::{self, Helpers};
use core::cmp::Ordering;
use core::convert;
use core::ops;

// Used in a pattern.

#[derive(Clone, Copy, Debug)]
pub struct Duration<T, const NOM: u32, const DENOM: u32> {
    ticks: T,
}

macro_rules! impl_duration_for_integer {
    ($i:ty) => {
        impl<const NOM: u32, const DENOM: u32> Duration<$i, NOM, DENOM> {
            pub const fn from_ticks(ticks: $i) -> Self {
                helpers::greater_than_0::<NOM>();
                helpers::greater_than_0::<DENOM>();

                Duration { ticks }
            }

            pub const fn ticks(&self) -> $i {
                self.ticks
            }

            pub const fn checked_add<const O_NOM: u32, const O_DENOM: u32>(
                self,
                other: Duration<$i, O_NOM, O_DENOM>,
            ) -> Option<Self> {
                if Helpers::<NOM, DENOM, O_NOM, O_DENOM>::SAME_BASE {
                    if let Some(ticks) = self.ticks.checked_add(other.ticks) {
                        Some(Duration::<$i, NOM, DENOM>::from_ticks(ticks))
                    } else {
                        None
                    }
                } else {
                    if let Some(lh) = other
                        .ticks
                        .checked_mul(Helpers::<NOM, DENOM, O_NOM, O_DENOM>::LD_TIMES_RN as $i)
                    {
                        let ticks = lh / Helpers::<NOM, DENOM, O_NOM, O_DENOM>::RD_TIMES_LN as $i;

                        if let Some(ticks) = self.ticks.checked_add(ticks) {
                            Some(Duration::<$i, NOM, DENOM>::from_ticks(ticks))
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
                other: Duration<$i, O_NOM, O_DENOM>,
            ) -> Option<Self> {
                if Helpers::<NOM, DENOM, O_NOM, O_DENOM>::SAME_BASE {
                    if let Some(ticks) = self.ticks.checked_sub(other.ticks) {
                        Some(Duration::<$i, NOM, DENOM>::from_ticks(ticks))
                    } else {
                        None
                    }
                } else {
                    if let Some(lh) = other
                        .ticks
                        .checked_mul(Helpers::<NOM, DENOM, O_NOM, O_DENOM>::LD_TIMES_RN as $i)
                    {
                        let ticks = lh / Helpers::<NOM, DENOM, O_NOM, O_DENOM>::RD_TIMES_LN as $i;

                        if let Some(ticks) = self.ticks.checked_sub(ticks) {
                            Some(Duration::<$i, NOM, DENOM>::from_ticks(ticks))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            }

            pub const fn micros(val: $i) -> Duration<$i, NOM, DENOM> {
                Duration::<$i, NOM, DENOM>::from_ticks(
                    (Helpers::<1, 1_000_000, NOM, DENOM>::RD_TIMES_LN as $i * val)
                        / Helpers::<1, 1_000_000, NOM, DENOM>::LD_TIMES_RN as $i,
                )
            }

            pub const fn millis(val: $i) -> Duration<$i, NOM, DENOM> {
                Duration::<$i, NOM, DENOM>::from_ticks(
                    (Helpers::<1, 1_000, NOM, DENOM>::RD_TIMES_LN as $i * val)
                        / Helpers::<1, 1_000, NOM, DENOM>::LD_TIMES_RN as $i,
                )
            }

            pub const fn secs(val: $i) -> Duration<$i, NOM, DENOM> {
                Duration::<$i, NOM, DENOM>::from_ticks(
                    (Helpers::<1, 1, NOM, DENOM>::RD_TIMES_LN as $i * val)
                        / Helpers::<1, 1, NOM, DENOM>::LD_TIMES_RN as $i,
                )
            }

            pub const fn minutes(val: $i) -> Duration<$i, NOM, DENOM> {
                Duration::<$i, NOM, DENOM>::from_ticks(
                    (Helpers::<60, 1, NOM, DENOM>::RD_TIMES_LN as $i * val)
                        / Helpers::<60, 1, NOM, DENOM>::LD_TIMES_RN as $i,
                )
            }

            pub const fn hours(val: $i) -> Duration<$i, NOM, DENOM> {
                Duration::<$i, NOM, DENOM>::from_ticks(
                    (Helpers::<3_600, 1, NOM, DENOM>::RD_TIMES_LN as $i * val)
                        / Helpers::<3_600, 1, NOM, DENOM>::LD_TIMES_RN as $i,
                )
            }
        }

        impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
            PartialOrd<Duration<$i, R_NOM, R_DENOM>> for Duration<$i, L_NOM, L_DENOM>
        {
            fn partial_cmp(&self, other: &Duration<$i, R_NOM, R_DENOM>) -> Option<Ordering> {
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
                            .checked_mul(
                                Helpers::<L_NOM, L_DENOM, R_NOM, R_DENOM>::RD_TIMES_LN as $i,
                            )?
                            .cmp(&other.ticks.checked_mul(
                                Helpers::<L_NOM, L_DENOM, R_NOM, R_DENOM>::LD_TIMES_RN as $i,
                            )?),
                    )
                }
            }
        }

        impl<const NOM: u32, const DENOM: u32> Ord for Duration<$i, NOM, DENOM> {
            fn cmp(&self, other: &Self) -> Ordering {
                self.ticks.cmp(&other.ticks)
            }
        }

        impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
            PartialEq<Duration<$i, R_NOM, R_DENOM>> for Duration<$i, L_NOM, L_DENOM>
        {
            fn eq(&self, other: &Duration<$i, R_NOM, R_DENOM>) -> bool {
                if Helpers::<L_NOM, L_DENOM, R_NOM, R_DENOM>::SAME_BASE {
                    // If we are in the same base, comparison in trivial
                    self.ticks.eq(&other.ticks)
                } else {
                    let lh = self
                        .ticks
                        .checked_mul(Helpers::<L_NOM, L_DENOM, R_NOM, R_DENOM>::RD_TIMES_LN as $i);
                    let rh = other
                        .ticks
                        .checked_mul(Helpers::<L_NOM, L_DENOM, R_NOM, R_DENOM>::LD_TIMES_RN as $i);

                    if let (Some(lh), Some(rh)) = (lh, rh) {
                        lh == rh
                    } else {
                        false
                    }
                }
            }
        }

        impl<const NOM: u32, const DENOM: u32> Eq for Duration<$i, NOM, DENOM> {}

        // Duration - Duration = Duration
        impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
            ops::Sub<Duration<$i, R_NOM, R_DENOM>> for Duration<$i, L_NOM, L_DENOM>
        {
            type Output = Duration<$i, L_NOM, L_DENOM>;

            fn sub(self, other: Duration<$i, R_NOM, R_DENOM>) -> Self::Output {
                if let Some(v) = self.checked_sub(other) {
                    v
                } else {
                    panic!("Add failed!");
                }
            }
        }

        // Duration + Duration = Duration
        impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
            ops::Add<Duration<$i, R_NOM, R_DENOM>> for Duration<$i, L_NOM, L_DENOM>
        {
            type Output = Duration<$i, L_NOM, L_DENOM>;

            fn add(self, other: Duration<$i, R_NOM, R_DENOM>) -> Self::Output {
                if let Some(v) = self.checked_add(other) {
                    v
                } else {
                    panic!("Add failed!");
                }
            }
        }

        // integer * Duration = Duration
        impl<const NOM: u32, const DENOM: u32> ops::Mul<Duration<$i, NOM, DENOM>> for u32 {
            type Output = Duration<$i, NOM, DENOM>;

            fn mul(self, mut other: Duration<$i, NOM, DENOM>) -> Self::Output {
                other.ticks *= self as $i;
                other
            }
        }

        // Duration * integer = Duration
        impl<const NOM: u32, const DENOM: u32> ops::Mul<u32> for Duration<$i, NOM, DENOM> {
            type Output = Duration<$i, NOM, DENOM>;

            fn mul(mut self, other: u32) -> Self::Output {
                self.ticks *= other as $i;
                self
            }
        }

        // Duration / integer = Duration
        impl<const NOM: u32, const DENOM: u32> ops::Div<u32> for Duration<$i, NOM, DENOM> {
            type Output = Duration<$i, NOM, DENOM>;

            fn div(mut self, other: u32) -> Self::Output {
                self.ticks /= other as $i;
                self
            }
        }
    };
}

impl_duration_for_integer!(u32);
impl_duration_for_integer!(u64);

//
// Operations between u32 and u64 Durations
//

impl<const NOM: u32, const DENOM: u32> From<Duration<u32, NOM, DENOM>>
    for Duration<u64, NOM, DENOM>
{
    fn from(val: Duration<u32, NOM, DENOM>) -> Duration<u64, NOM, DENOM> {
        Duration::<u64, NOM, DENOM>::from_ticks(val.ticks() as u64)
    }
}

impl<const NOM: u32, const DENOM: u32> convert::TryFrom<Duration<u64, NOM, DENOM>>
    for Duration<u32, NOM, DENOM>
{
    type Error = ();

    fn try_from(val: Duration<u64, NOM, DENOM>) -> Result<Duration<u32, NOM, DENOM>, ()> {
        use convert::TryInto;

        Ok(Duration::<u32, NOM, DENOM>::from_ticks(
            val.ticks().try_into().map_err(|_| ())?,
        ))
    }
}

// Duration - Duration = Duration (to make shorthands work)
impl<const NOM: u32, const DENOM: u32> ops::Sub<Duration<u32, NOM, DENOM>>
    for Duration<u64, NOM, DENOM>
{
    type Output = Duration<u64, NOM, DENOM>;

    fn sub(self, other: Duration<u32, NOM, DENOM>) -> Self::Output {
        self.sub(Duration::<u64, NOM, DENOM>::from_ticks(other.ticks() as u64))
    }
}

// Duration + Duration = Duration (to make shorthands work)
impl<const NOM: u32, const DENOM: u32> ops::Add<Duration<u32, NOM, DENOM>>
    for Duration<u64, NOM, DENOM>
{
    type Output = Duration<u64, NOM, DENOM>;

    fn add(self, other: Duration<u32, NOM, DENOM>) -> Self::Output {
        self.add(Duration::<u64, NOM, DENOM>::from_ticks(other.ticks() as u64))
    }
}

impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
    PartialOrd<Duration<u32, R_NOM, R_DENOM>> for Duration<u64, L_NOM, L_DENOM>
{
    fn partial_cmp(&self, other: &Duration<u32, R_NOM, R_DENOM>) -> Option<Ordering> {
        self.partial_cmp(&Duration::<u64, R_NOM, R_DENOM>::from_ticks(
            other.ticks() as u64
        ))
    }
}

impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
    PartialEq<Duration<u32, R_NOM, R_DENOM>> for Duration<u64, L_NOM, L_DENOM>
{
    fn eq(&self, other: &Duration<u32, R_NOM, R_DENOM>) -> bool {
        self.eq(&Duration::<u64, R_NOM, R_DENOM>::from_ticks(
            other.ticks() as u64
        ))
    }
}

impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
    PartialOrd<Duration<u64, R_NOM, R_DENOM>> for Duration<u32, L_NOM, L_DENOM>
{
    fn partial_cmp(&self, other: &Duration<u64, R_NOM, R_DENOM>) -> Option<Ordering> {
        Duration::<u64, L_NOM, L_DENOM>::from_ticks(self.ticks as u64).partial_cmp(other)
    }
}

impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
    PartialEq<Duration<u64, R_NOM, R_DENOM>> for Duration<u32, L_NOM, L_DENOM>
{
    fn eq(&self, other: &Duration<u64, R_NOM, R_DENOM>) -> bool {
        Duration::<u64, L_NOM, L_DENOM>::from_ticks(self.ticks as u64).eq(other)
    }
}

/// Extension trait for simple short-hands
pub trait ExtU32 {
    fn micros<const NOM: u32, const DENOM: u32>(self) -> Duration<u32, NOM, DENOM>;
    fn millis<const NOM: u32, const DENOM: u32>(self) -> Duration<u32, NOM, DENOM>;
    fn secs<const NOM: u32, const DENOM: u32>(self) -> Duration<u32, NOM, DENOM>;
    fn minutes<const NOM: u32, const DENOM: u32>(self) -> Duration<u32, NOM, DENOM>;
    fn hours<const NOM: u32, const DENOM: u32>(self) -> Duration<u32, NOM, DENOM>;
}

impl ExtU32 for u32 {
    fn micros<const NOM: u32, const DENOM: u32>(self) -> Duration<u32, NOM, DENOM> {
        Duration::<u32, NOM, DENOM>::micros(self)
    }

    fn millis<const NOM: u32, const DENOM: u32>(self) -> Duration<u32, NOM, DENOM> {
        Duration::<u32, NOM, DENOM>::millis(self)
    }

    fn secs<const NOM: u32, const DENOM: u32>(self) -> Duration<u32, NOM, DENOM> {
        Duration::<u32, NOM, DENOM>::secs(self)
    }

    fn minutes<const NOM: u32, const DENOM: u32>(self) -> Duration<u32, NOM, DENOM> {
        Duration::<u32, NOM, DENOM>::minutes(self)
    }

    fn hours<const NOM: u32, const DENOM: u32>(self) -> Duration<u32, NOM, DENOM> {
        Duration::<u32, NOM, DENOM>::hours(self)
    }
}
