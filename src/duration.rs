use crate::helpers::{self, Helpers};
use core::cmp::Ordering;
use core::convert;
use core::ops;

/// Represents a duration of time in seconds.
///
/// The generic `T` can either be `u32` or `u64`, and the const generics represent the ratio of the
/// ticks contained within the duration: `duration in seconds = NOM / DENOM * ticks`
#[derive(Clone, Copy, Debug)]
pub struct Duration<T, const NOM: u32, const DENOM: u32> {
    ticks: T,
}

macro_rules! impl_duration_for_integer {
    ($i:ty) => {
        impl<const NOM: u32, const DENOM: u32> Duration<$i, NOM, DENOM> {
            /// Create a `Duration` from a ticks value.
            ///
            /// ```
            /// # use const_embedded_time::*;
            #[doc = concat!("let _d = Duration::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            /// ```
            #[inline]
            pub const fn from_ticks(ticks: $i) -> Self {
                helpers::greater_than_0::<NOM>();
                helpers::greater_than_0::<DENOM>();

                Duration { ticks }
            }

            /// Extract the ticks from a `Duration`.
            ///
            /// ```
            /// # use const_embedded_time::*;
            #[doc = concat!("let d = Duration::<", stringify!($i), ", 1, 1_000>::from_ticks(234);")]
            ///
            /// assert_eq!(d.ticks(), 234);
            /// ```
            #[inline]
            pub const fn ticks(&self) -> $i {
                self.ticks
            }

            /// Add two durations while checking for overflow.
            ///
            /// ```
            /// # use const_embedded_time::*;
            #[doc = concat!("let d1 = Duration::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            #[doc = concat!("let d2 = Duration::<", stringify!($i), ", 1, 1_000>::from_ticks(2);")]
            #[doc = concat!("let d3 = Duration::<", stringify!($i), ", 1, 1_000>::from_ticks(", stringify!($i), "::MAX);")]
            ///
            /// assert_eq!(d1.checked_add(d2).unwrap().ticks(), 3);
            /// assert_eq!(d1.checked_add(d3), None);
            /// ```
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

            /// Subtract two durations while checking for overflow.
            ///
            /// ```
            /// # use const_embedded_time::*;
            #[doc = concat!("let d1 = Duration::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            #[doc = concat!("let d2 = Duration::<", stringify!($i), ", 1, 1_000>::from_ticks(2);")]
            #[doc = concat!("let d3 = Duration::<", stringify!($i), ", 1, 1_000>::from_ticks(", stringify!($i), "::MAX);")]
            ///
            /// assert_eq!(d2.checked_sub(d1).unwrap().ticks(), 1);
            /// assert_eq!(d1.checked_sub(d3), None);
            /// ```
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

            /// Const into, checking for overflow.
            ///
            /// ```
            /// # use const_embedded_time::*;
            #[doc = concat!("let d1 = Duration::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            #[doc = concat!("let d2 = Duration::<", stringify!($i), ", 1, 1_000>::from_ticks(2);")]
            #[doc = concat!("let d3 = Duration::<", stringify!($i), ", 1, 1_000>::from_ticks(", stringify!($i), "::MAX);")]
            ///
            /// assert_eq!(d2.checked_sub(d1).unwrap().ticks(), 1);
            /// assert_eq!(d1.checked_sub(d3), None);
            /// ```
            pub fn checked_into<const O_NOM: u32, const O_DENOM: u32>(
                self,
            ) -> Option<Duration<$i, O_NOM, O_DENOM>> {
                if Helpers::<NOM, DENOM, O_NOM, O_DENOM>::SAME_BASE {
                    Some(Duration::<$i, O_NOM, O_DENOM>::from_ticks(self.ticks))
                } else {
                    if let Some(lh) = self
                        .ticks
                        .checked_mul(Helpers::<NOM, DENOM, O_NOM, O_DENOM>::RD_TIMES_LN as $i)
                    {
                        let ticks = lh / Helpers::<NOM, DENOM, O_NOM, O_DENOM>::LD_TIMES_RN as $i;

                        Some(Duration::<$i, O_NOM, O_DENOM>::from_ticks(ticks))
                    } else {
                        None
                    }
                }
            }

            /// Convert between bases for a duration.
            ///
            /// Unfortunately not a `From` impl due to collision with the std lib.
            ///
            /// ```
            /// # use const_embedded_time::*;
            #[doc = concat!("let d1 = Duration::<", stringify!($i), ", 1, 1_00>::from_ticks(1);")]
            #[doc = concat!("let d2: Duration::<", stringify!($i), ", 1, 1_000> = d1.convert();")]
            ///
            /// assert_eq!(d2.ticks(), 10);
            /// ```
            // Sooooon const with const panic
            pub fn convert<const O_NOM: u32, const O_DENOM: u32>(
                self,
            ) -> Duration<$i, O_NOM, O_DENOM> {
                if let Some(v) = self.checked_into() {
                    v
                } else {
                    panic!("Into failed!");
                }
            }

            /// Shorthand for creating a duration which represents microseconds.
            #[inline]
            pub const fn micros(val: $i) -> Duration<$i, NOM, DENOM> {
                Duration::<$i, NOM, DENOM>::from_ticks(
                    (Helpers::<1, 1_000_000, NOM, DENOM>::RD_TIMES_LN as $i * val)
                        / Helpers::<1, 1_000_000, NOM, DENOM>::LD_TIMES_RN as $i,
                )
            }

            /// Shorthand for creating a duration which represents milliseconds.
            #[inline]
            pub const fn millis(val: $i) -> Duration<$i, NOM, DENOM> {
                Duration::<$i, NOM, DENOM>::from_ticks(
                    (Helpers::<1, 1_000, NOM, DENOM>::RD_TIMES_LN as $i * val)
                        / Helpers::<1, 1_000, NOM, DENOM>::LD_TIMES_RN as $i,
                )
            }

            /// Shorthand for creating a duration which represents seconds.
            #[inline]
            pub const fn secs(val: $i) -> Duration<$i, NOM, DENOM> {
                Duration::<$i, NOM, DENOM>::from_ticks(
                    (Helpers::<1, 1, NOM, DENOM>::RD_TIMES_LN as $i * val)
                        / Helpers::<1, 1, NOM, DENOM>::LD_TIMES_RN as $i,
                )
            }

            /// Shorthand for creating a duration which represents minutes.
            #[inline]
            pub const fn minutes(val: $i) -> Duration<$i, NOM, DENOM> {
                Duration::<$i, NOM, DENOM>::from_ticks(
                    (Helpers::<60, 1, NOM, DENOM>::RD_TIMES_LN as $i * val)
                        / Helpers::<60, 1, NOM, DENOM>::LD_TIMES_RN as $i,
                )
            }

            /// Shorthand for creating a duration which represents hours.
            #[inline]
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
            #[inline]
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
            #[inline]
            fn cmp(&self, other: &Self) -> Ordering {
                self.ticks.cmp(&other.ticks)
            }
        }

        impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
            PartialEq<Duration<$i, R_NOM, R_DENOM>> for Duration<$i, L_NOM, L_DENOM>
        {
            #[inline]
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

            #[inline]
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

            #[inline]
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

            #[inline]
            fn mul(self, mut other: Duration<$i, NOM, DENOM>) -> Self::Output {
                other.ticks *= self as $i;
                other
            }
        }

        // Duration * integer = Duration
        impl<const NOM: u32, const DENOM: u32> ops::Mul<u32> for Duration<$i, NOM, DENOM> {
            type Output = Duration<$i, NOM, DENOM>;

            #[inline]
            fn mul(mut self, other: u32) -> Self::Output {
                self.ticks *= other as $i;
                self
            }
        }

        // Duration / integer = Duration
        impl<const NOM: u32, const DENOM: u32> ops::Div<u32> for Duration<$i, NOM, DENOM> {
            type Output = Duration<$i, NOM, DENOM>;

            #[inline]
            fn div(mut self, other: u32) -> Self::Output {
                self.ticks /= other as $i;
                self
            }
        }

        #[cfg(feature = "defmt")]
        impl<const NOM: u32, const DENOM: u32> defmt::Format for Duration<$i, NOM, DENOM>
        {
            fn format(&self, f: defmt::Formatter) {
                if NOM == 3_600 && DENOM == 1 {
                    defmt::write!(f, "{} h", self.ticks)
                } else if NOM == 60 && DENOM == 1 {
                    defmt::write!(f, "{} min", self.ticks)
                } else if NOM == 1 && DENOM == 1 {
                    defmt::write!(f, "{} s", self.ticks)
                } else if NOM == 1 && DENOM == 1_000 {
                    defmt::write!(f, "{} ms", self.ticks)
                } else if NOM == 1 && DENOM == 1_000_000 {
                    defmt::write!(f, "{} us", self.ticks)
                } else if NOM == 1 && DENOM == 1_000_000_000 {
                    defmt::write!(f, "{} ns", self.ticks)
                } else {
                    defmt::write!(f, "{} ticks @ ({}/{})", self.ticks, NOM, DENOM)
                }
            }
        }

        impl<const NOM: u32, const DENOM: u32> core::fmt::Display for Duration<$i, NOM, DENOM> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                if NOM == 3_600 && DENOM == 1 {
                    write!(f, "{} h", self.ticks)
                } else if NOM == 60 && DENOM == 1 {
                    write!(f, "{} min", self.ticks)
                } else if NOM == 1 && DENOM == 1 {
                    write!(f, "{} s", self.ticks)
                } else if NOM == 1 && DENOM == 1_000 {
                    write!(f, "{} ms", self.ticks)
                } else if NOM == 1 && DENOM == 1_000_000 {
                    write!(f, "{} us", self.ticks)
                } else if NOM == 1 && DENOM == 1_000_000_000 {
                    write!(f, "{} ns", self.ticks)
                } else {
                    write!(f, "{} ticks @ ({}/{})", self.ticks, NOM, DENOM)
                }
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
    #[inline]
    fn from(val: Duration<u32, NOM, DENOM>) -> Duration<u64, NOM, DENOM> {
        Duration::<u64, NOM, DENOM>::from_ticks(val.ticks() as u64)
    }
}

impl<const NOM: u32, const DENOM: u32> convert::TryFrom<Duration<u64, NOM, DENOM>>
    for Duration<u32, NOM, DENOM>
{
    type Error = ();

    #[inline]
    fn try_from(val: Duration<u64, NOM, DENOM>) -> Result<Duration<u32, NOM, DENOM>, ()> {
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

    #[inline]
    fn sub(self, other: Duration<u32, NOM, DENOM>) -> Self::Output {
        self.sub(Duration::<u64, NOM, DENOM>::from_ticks(other.ticks() as u64))
    }
}

// Duration + Duration = Duration (to make shorthands work)
impl<const NOM: u32, const DENOM: u32> ops::Add<Duration<u32, NOM, DENOM>>
    for Duration<u64, NOM, DENOM>
{
    type Output = Duration<u64, NOM, DENOM>;

    #[inline]
    fn add(self, other: Duration<u32, NOM, DENOM>) -> Self::Output {
        self.add(Duration::<u64, NOM, DENOM>::from_ticks(other.ticks() as u64))
    }
}

impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
    PartialOrd<Duration<u32, R_NOM, R_DENOM>> for Duration<u64, L_NOM, L_DENOM>
{
    #[inline]
    fn partial_cmp(&self, other: &Duration<u32, R_NOM, R_DENOM>) -> Option<Ordering> {
        self.partial_cmp(&Duration::<u64, R_NOM, R_DENOM>::from_ticks(
            other.ticks() as u64
        ))
    }
}

impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
    PartialEq<Duration<u32, R_NOM, R_DENOM>> for Duration<u64, L_NOM, L_DENOM>
{
    #[inline]
    fn eq(&self, other: &Duration<u32, R_NOM, R_DENOM>) -> bool {
        self.eq(&Duration::<u64, R_NOM, R_DENOM>::from_ticks(
            other.ticks() as u64
        ))
    }
}

impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
    PartialOrd<Duration<u64, R_NOM, R_DENOM>> for Duration<u32, L_NOM, L_DENOM>
{
    #[inline]
    fn partial_cmp(&self, other: &Duration<u64, R_NOM, R_DENOM>) -> Option<Ordering> {
        Duration::<u64, L_NOM, L_DENOM>::from_ticks(self.ticks as u64).partial_cmp(other)
    }
}

impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
    PartialEq<Duration<u64, R_NOM, R_DENOM>> for Duration<u32, L_NOM, L_DENOM>
{
    #[inline]
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
    #[inline]
    fn micros<const NOM: u32, const DENOM: u32>(self) -> Duration<u32, NOM, DENOM> {
        Duration::<u32, NOM, DENOM>::micros(self)
    }

    #[inline]
    fn millis<const NOM: u32, const DENOM: u32>(self) -> Duration<u32, NOM, DENOM> {
        Duration::<u32, NOM, DENOM>::millis(self)
    }

    #[inline]
    fn secs<const NOM: u32, const DENOM: u32>(self) -> Duration<u32, NOM, DENOM> {
        Duration::<u32, NOM, DENOM>::secs(self)
    }

    #[inline]
    fn minutes<const NOM: u32, const DENOM: u32>(self) -> Duration<u32, NOM, DENOM> {
        Duration::<u32, NOM, DENOM>::minutes(self)
    }

    #[inline]
    fn hours<const NOM: u32, const DENOM: u32>(self) -> Duration<u32, NOM, DENOM> {
        Duration::<u32, NOM, DENOM>::hours(self)
    }
}
