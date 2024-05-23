use super::Fraction;
use crate::helpers::Helpers;
use crate::NanosDurationU64;
use crate::Rate;
use crate::SecsDurationU64;
use core::cmp::Ordering;
use core::convert;
use core::ops;

/// Represents a duration of time.
///
/// The generic `T` can either be `u32` or `u64`, and the const generics represent the ratio of the
/// ticks contained within the duration: `duration in seconds = NOM / DENOM * ticks`
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "postcard_max_size",
    derive(postcard::experimental::max_size::MaxSize)
)]
#[derive(Clone, Copy, Debug)]
pub struct Duration<T, const F: Fraction> {
    pub(crate) ticks: T,
}

macro_rules! shorthand {
    ($i:ty, $nom:literal, $denum:literal, $from_unit:ident, $as_unit:ident, $from_unital:ident, $unitstr:literal) => {
        #[doc = concat!("Convert the Duration to an integer number of ", $unitstr, ".")]
        #[doc = ""]
        #[doc = concat!("**Warning**: This function will cause a compilation error if the conversion constants don't fit in `", stringify!($i), "`.")]
        #[inline]
        pub const fn $as_unit(&self) -> $i {
            // Compile-time/runtime check - will fail if constants don't fit in target type
            assert!(
                Helpers::<{ Fraction::new($nom, $denum) }, F>::RD_TIMES_LN <= <$i>::MAX as u64,
                concat!("Conversion constant RD_TIMES_LN doesn't fit in ", stringify!($i), " for this Duration type")
            );
            assert!(
                Helpers::<{ Fraction::new($nom, $denum) }, F>::LD_TIMES_RN <= <$i>::MAX as u64,
                concat!("Conversion constant LD_TIMES_RN doesn't fit in ", stringify!($i), " for this Duration type")
            );

            (Helpers::<{ Fraction::new($nom, $denum) }, F>::LD_TIMES_RN as $i * self.ticks)
                / Helpers::<{ Fraction::new($nom, $denum) }, F>::RD_TIMES_LN as $i
        }

        #[doc = concat!("Create a duration from a number of ", $unitstr, ".")]
        #[doc = ""]
        #[doc = concat!("**Warning**: This function will cause a compilation error if the conversion constants don't fit in `", stringify!($i), "`.")]
        #[inline]
        pub const fn $from_unit(val: $i) -> Self {
            // Compile-time/runtime check - will fail if constants don't fit in target type
            assert!(
                Helpers::<{ Fraction::new($nom, $denum) }, F>::RD_TIMES_LN <= <$i>::MAX as u64,
                concat!("Conversion constant RD_TIMES_LN doesn't fit in ", stringify!($i), " for this Duration type")
            );
            assert!(
                Helpers::<{ Fraction::new($nom, $denum) }, F>::LD_TIMES_RN <= <$i>::MAX as u64,
                concat!("Conversion constant LD_TIMES_RN doesn't fit in ", stringify!($i), " for this Duration type")
            );

            Self::from_ticks(
                (Helpers::<{ Fraction::new($nom, $denum) }, F>::RD_TIMES_LN as $i * val)
                    / Helpers::<{ Fraction::new($nom, $denum) }, F>::LD_TIMES_RN as $i
            )
        }

        #[doc = concat!("Create a duration from a number of ", $unitstr, " (ceil rounded).")]
        #[doc = ""]
        #[doc = concat!("**Warning**: This function will cause a compilation error if the conversion constants don't fit in `", stringify!($i), "`.")]
        #[inline]
        pub const fn $from_unital(val: $i) -> Self {
            // Compile-time/runtime check - will fail if constants don't fit in target type
            assert!(
                Helpers::<{ Fraction::new($nom, $denum) }, F>::RD_TIMES_LN <= <$i>::MAX as u64,
                concat!("Conversion constant RD_TIMES_LN doesn't fit in ", stringify!($i), " for this Duration type")
            );
            assert!(
                Helpers::<{ Fraction::new($nom, $denum) }, F>::LD_TIMES_RN <= <$i>::MAX as u64,
                concat!("Conversion constant LD_TIMES_RN doesn't fit in ", stringify!($i), " for this Duration type")
            );

            let mul = Helpers::<{ Fraction::new($nom, $denum) }, F>::RD_TIMES_LN as $i * val;
            let ld_times_rn = Helpers::<{ Fraction::new($nom, $denum) }, F>::LD_TIMES_RN as $i;
            Self::from_ticks(mul.div_ceil(ld_times_rn))
        }
    };
}

macro_rules! impl_duration_for_integer {
    ($i:ty) => {
        impl<const F: Fraction> Duration<$i, F> {
            /// Create a `Duration` from a ticks value.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let _d = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(1);")]
            /// ```
            #[inline]
            pub const fn from_ticks(ticks: $i) -> Self {
                assert!(F.num > 0);
                assert!(F.denom > 0);

                Duration { ticks }
            }

            /// Extract the ticks from a `Duration`.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let d = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(234);")]
            ///
            /// assert_eq!(d.as_ticks(), 234);
            /// ```
            #[inline]
            pub const fn as_ticks(&self) -> $i {
                self.ticks
            }

            /// A duration of zero time.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let d = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::ZERO;")]
            ///
            /// assert_eq!(d.as_ticks(), 0);
            /// assert!(d.is_zero());
            /// ```
            pub const ZERO: Self = Self::from_ticks(0);

            /// The maximum duration.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let d = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::MAX;")]
            ///
            #[doc = concat!("assert_eq!(d.as_ticks(), ", stringify!($i), "::MAX);")]
            /// ```
            pub const MAX: Self = Self::from_ticks(<$i>::MAX);

            /// Returns true if this `Duration` spans no time
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let zero = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(0);")]
            #[doc = concat!("let one = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(1);")]
            ///
            /// assert_eq!(zero.is_zero(), true);
            /// assert_eq!(one.is_zero(), false);
            /// ```
            #[inline]
            pub const fn is_zero(&self) -> bool {
                self.ticks == 0
            }

            /// Add two durations while checking for overflow.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let d1 = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(1);")]
            #[doc = concat!("let d2 = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(2);")]
            #[doc = concat!("let d3 = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(", stringify!($i), "::MAX);")]
            ///
            /// assert_eq!(d1.checked_add(d2).unwrap().as_ticks(), 3);
            /// assert_eq!(d1.checked_add(d3), None);
            /// ```
            pub const fn checked_add<const O: Fraction>(
                self,
                other: Duration<$i, O>,
            ) -> Option<Self> {
                if Helpers::<F, O>::SAME_BASE {
                    if let Some(ticks) = self.ticks.checked_add(other.ticks) {
                        Some(Self::from_ticks(ticks))
                    } else {
                        None
                    }
                } else {
                    if let Some(lh) = other
                        .ticks
                        .checked_mul(Helpers::<F, O>::LD_TIMES_RN as $i)
                    {
                        let ticks = lh / Helpers::<F, O>::RD_TIMES_LN as $i;

                        if let Some(ticks) = self.ticks.checked_add(ticks) {
                            Some(Self::from_ticks(ticks))
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
            /// # use fugit::*;
            #[doc = concat!("let d1 = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(1);")]
            #[doc = concat!("let d2 = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(2);")]
            #[doc = concat!("let d3 = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(", stringify!($i), "::MAX);")]
            ///
            /// assert_eq!(d2.checked_sub(d1).unwrap().as_ticks(), 1);
            /// assert_eq!(d1.checked_sub(d3), None);
            /// ```
            pub const fn checked_sub<const O: Fraction>(
                self,
                other: Duration<$i, O>,
            ) -> Option<Self> {
                if Helpers::<F, O>::SAME_BASE {
                    if let Some(ticks) = self.ticks.checked_sub(other.ticks) {
                        Some(Self::from_ticks(ticks))
                    } else {
                        None
                    }
                } else {
                    if let Some(lh) = other
                        .ticks
                        .checked_mul(Helpers::<F, O>::LD_TIMES_RN as $i)
                    {
                        let ticks = lh / Helpers::<F, O>::RD_TIMES_LN as $i;

                        if let Some(ticks) = self.ticks.checked_sub(ticks) {
                            Some(Self::from_ticks(ticks))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            }

            /// Multiply this duration by an integer while checking for overflow.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let d = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(10);")]
            ///
            /// assert_eq!(d.checked_mul(3).unwrap().as_ticks(), 30);
            #[doc = concat!("assert_eq!(Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::MAX.checked_mul(2), None);")]
            /// ```
            #[inline]
            pub const fn checked_mul(self, rhs: $i) -> Option<Self> {
                if let Some(ticks) = self.ticks.checked_mul(rhs) {
                    Some(Self::from_ticks(ticks))
                } else {
                    None
                }
            }

            /// Divide this duration by an integer while checking for division by zero.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let d = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(30);")]
            ///
            /// assert_eq!(d.checked_div(3).unwrap().as_ticks(), 10);
            #[doc = concat!("assert_eq!(Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(10).checked_div(0), None);")]
            /// ```
            #[inline]
            pub const fn checked_div(self, rhs: $i) -> Option<Self> {
                if rhs == 0 {
                    None
                } else {
                    Some(Self::from_ticks(self.ticks / rhs))
                }
            }

            /// Get the remainder of dividing two durations while checking for divide-by-zero.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let d1 = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(10);")]
            #[doc = concat!("let d2 = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(3);")]
            ///
            /// assert_eq!(d1.checked_rem(d2).unwrap().as_ticks(), 1);
            /// ```
            pub const fn checked_rem<const O: Fraction>(
                self,
                other: Duration<$i, O>,
            ) -> Option<Self> {
                if other.ticks == 0 {
                    None
                } else if Helpers::<F, O>::SAME_BASE {
                    Some(Self::from_ticks(self.ticks % other.ticks))
                } else {
                    if let Some(lh) = other
                        .ticks
                        .checked_mul(Helpers::<F, O>::LD_TIMES_RN as $i)
                    {
                        let ticks = lh / Helpers::<F, O>::RD_TIMES_LN as $i;

                        if ticks > 0 {
                            Some(Self::from_ticks(self.ticks % ticks))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            }

            /// Divide this duration by an integer, rounding up (ceiling division).
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let d = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(31);")]
            ///
            /// assert_eq!(d.div_ceil(3).as_ticks(), 11);
            #[doc = concat!("let d = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(30);")]
            /// assert_eq!(d.div_ceil(3).as_ticks(), 10);
            /// ```
            ///
            /// # Panics
            ///
            /// This function will panic if `rhs` is zero.
            #[inline]
            #[track_caller]
            pub const fn div_ceil(self, rhs: $i) -> Self {
                Self::from_ticks(self.ticks.div_ceil(rhs))
            }

            /// Saturating duration addition. Computes `self + other`, saturating at the maximum value.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let d1 = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(1);")]
            #[doc = concat!("let d2 = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(2);")]
            ///
            /// assert_eq!(d1.saturating_add(d2).as_ticks(), 3);
            #[doc = concat!("assert_eq!(Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::MAX.saturating_add(d1).as_ticks(), ", stringify!($i), "::MAX);")]
            /// ```
            pub const fn saturating_add<const O: Fraction>(
                self,
                other: Duration<$i, O>,
            ) -> Self {
                if let Some(result) = self.checked_add(other) {
                    result
                } else {
                    Self::MAX
                }
            }

            /// Saturating duration subtraction. Computes `self - other`, saturating at zero.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let d1 = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(10);")]
            #[doc = concat!("let d2 = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(2);")]
            ///
            /// assert_eq!(d1.saturating_sub(d2).as_ticks(), 8);
            /// assert_eq!(d2.saturating_sub(d1).as_ticks(), 0);
            /// ```
            pub const fn saturating_sub<const O: Fraction>(
                self,
                other: Duration<$i, O>,
            ) -> Self {
                if let Some(result) = self.checked_sub(other) {
                    result
                } else {
                    Self::ZERO
                }
            }

            /// Saturating duration multiplication. Computes `self * rhs`, saturating at the maximum value.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let d = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(10);")]
            ///
            /// assert_eq!(d.saturating_mul(3).as_ticks(), 30);
            #[doc = concat!("assert_eq!(Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::MAX.saturating_mul(2).as_ticks(), ", stringify!($i), "::MAX);")]
            /// ```
            #[inline]
            pub const fn saturating_mul(self, rhs: $i) -> Self {
                if let Some(result) = self.checked_mul(rhs) {
                    result
                } else {
                    Self::MAX
                }
            }

            #[doc = concat!("Const `cmp` for ", stringify!($i))]
            #[inline(always)]
            const fn _const_cmp(a: $i, b: $i) -> Ordering {
                if a < b {
                    Ordering::Less
                } else if a > b {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }

            /// Const partial comparison.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let d1 = Duration::<", stringify!($i), ", { Fraction::new(1, 1_00) }>::from_ticks(1);")]
            #[doc = concat!("let d2 = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(1);")]
            ///
            /// assert_eq!(d1.const_partial_cmp(d2), Some(core::cmp::Ordering::Greater));
            /// ```
            #[inline]
            pub const fn const_partial_cmp<const R: Fraction>(
                self,
                other: Duration<$i, R>
            ) -> Option<Ordering> {
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

                if Helpers::<F, R>::SAME_BASE {
                    // If we are in the same base, comparison in trivial
                    Some(Self::_const_cmp(self.ticks, other.ticks))
                } else {
                    let lh = self
                        .ticks
                        .checked_mul(Helpers::<F, R>::RD_TIMES_LN as $i);
                    let rh = other
                        .ticks
                        .checked_mul(Helpers::<F, R>::LD_TIMES_RN as $i);

                    if let (Some(lh), Some(rh)) = (lh, rh) {
                        Some(Self::_const_cmp(lh, rh))
                    } else {
                        None
                    }
                }
            }

            /// Const equality check.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let d1 = Duration::<", stringify!($i), ", { Fraction::new(1, 1_00) }>::from_ticks(1);")]
            #[doc = concat!("let d2 = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(10);")]
            ///
            /// assert!(d1.const_eq(d2));
            /// ```
            #[inline]
            pub const fn const_eq<const R: Fraction>(
                self,
                other: Duration<$i, R>
            ) -> bool {
                if Helpers::<F, R>::SAME_BASE {
                    // If we are in the same base, comparison in trivial
                    self.ticks == other.ticks
                } else {
                    let lh = self
                        .ticks
                        .checked_mul(Helpers::<F, R>::RD_TIMES_LN as $i);
                    let rh = other
                        .ticks
                        .checked_mul(Helpers::<F, R>::LD_TIMES_RN as $i);

                    if let (Some(lh), Some(rh)) = (lh, rh) {
                        lh == rh
                    } else {
                        false
                    }
                }
            }

            /// Const try from, checking for overflow.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let d1 = Duration::<", stringify!($i), ", { Fraction::new(1, 1_00) }>::from_ticks(1);")]
            #[doc = concat!("let d2 = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::const_try_from(d1);")]
            ///
            /// assert_eq!(d2.unwrap().as_ticks(), 10);
            /// ```
            pub const fn const_try_from<const I: Fraction>(
                duration: Duration<$i, I>,
            ) -> Option<Self> {
                if Helpers::<I, F>::SAME_BASE {
                    Some(Self::from_ticks(duration.ticks))
                } else {
                    if let Some(lh) = (duration.ticks as u64)
                        .checked_mul(Helpers::<I, F>::RD_TIMES_LN)
                    {
                        let ticks = (lh + Helpers::<I, F>::LD_TIMES_RN / 2)
                            / Helpers::<I, F>::LD_TIMES_RN;

                        if ticks <= <$i>::MAX as u64 {
                            Some(Self::from_ticks(ticks as $i))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            }

            /// Const try into, checking for overflow.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let d1 = Duration::<", stringify!($i), ", { Fraction::new(1, 1_00) }>::from_ticks(1);")]
            #[doc = concat!("let d2: Option<Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>> = d1.const_try_into();")]
            ///
            /// assert_eq!(d2.unwrap().as_ticks(), 10);
            /// ```
            #[inline]
            pub const fn const_try_into<const O: Fraction>(
                self,
            ) -> Option<Duration<$i, O>> {
                Duration::<$i, O>::const_try_from(self)
            }

            /// Const try into rate, checking for divide-by-zero.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let d1 = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(2);")]
            #[doc = concat!("let r1: Option<Rate::<", stringify!($i), ", { Fraction::new(1, 1) }>> = d1.try_to_rate();")]
            ///
            /// assert_eq!(r1.unwrap().to_raw(), 500);
            /// ```
            #[inline]
            pub const fn try_to_rate<const O: Fraction>(
                self,
            ) -> Option<Rate<$i, O>> {
                Rate::<$i, O>::try_from_duration(self)
            }

            /// Convert from duration to rate.
            #[inline]
            #[track_caller]
            pub const fn to_rate<const O: Fraction>(
                self,
            ) -> Rate<$i, O> {
                if let Some(v) = self.try_to_rate() {
                    v
                } else {
                    panic!("Into rate failed, divide-by-zero!");
                }
            }

            /// Const try from rate, checking for divide-by-zero.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let r1 = Rate::<", stringify!($i), ", { Fraction::new(1, 1) }>::from_raw(1);")]
            #[doc = concat!("let d1 = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::try_from_rate(r1);")]
            ///
            /// assert_eq!(d1.unwrap().as_ticks(), 1_000);
            /// ```
            #[inline]
            pub const fn try_from_rate<const I: Fraction>(
                rate: Rate<$i, I>,
            ) -> Option<Self> {
                if rate.raw > 0 {
                    Some(Self::from_ticks(
                        Helpers::<I, F>::RATE_TO_DURATION_NUMERATOR as $i
                        / rate.raw
                    ))
                } else {
                    None
                }
            }

            /// Convert from rate to duration.
            #[inline]
            #[track_caller]
            pub const fn from_rate<const I: Fraction>(
                rate: Rate<$i, I>,
            ) -> Self {
                if let Some(v) = Self::try_from_rate(rate) {
                    v
                } else {
                    panic!("From rate failed, divide-by-zero!");
                }
            }

            /// Convert between bases for a duration, rounds to nearest.
            ///
            /// Unfortunately not a `From` impl due to collision with the std lib.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let d1 = Duration::<", stringify!($i), ", { Fraction::new(1, 100) }>::from_ticks(1);")]
            #[doc = concat!("let d2: Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }> = d1.convert();")]
            ///
            /// assert_eq!(d2.as_ticks(), 10);
            /// ```
            /// Can be used in const contexts. Compilation will fail if the conversion causes overflow
            /// ```compile_fail
            /// # use fugit::*;
            #[doc = concat!("const TICKS: ", stringify!($i), "= ", stringify!($i), "::MAX - 10;")]
            #[doc = concat!("const D1: Duration::<", stringify!($i), ", { Fraction::new(1, 100) }> = Duration::<", stringify!($i), ", { Fraction::new(1, 100) }>::from_ticks(TICKS);")]
            /// // Fails conversion due to tick overflow
            #[doc = concat!("const D2: Duration::<", stringify!($i), ", { Fraction::new(1, 200) }> = D1.convert();")]
            #[inline]
            #[track_caller]
            pub const fn convert<const O: Fraction>(
                self,
            ) -> Duration<$i, O> {
                if let Some(v) = self.const_try_into() {
                    v
                } else {
                    panic!("Convert failed!");
                }
            }

            shorthand!($i, 1, 1_000_000_000_000, from_picos, as_picos, from_picos_at_least, "picoseconds");
            shorthand!($i, 1, 1_000_000_000, from_nanos, as_nanos, from_nanos_at_least, "nanoseconds");
            shorthand!($i, 1, 1_000_000, from_micros, as_micros, from_micros_at_least, "microseconds");
            shorthand!($i, 1, 1_000, from_millis, as_millis, from_millis_at_least, "milliseconds");
            shorthand!($i, 1, 1, from_secs, as_secs, from_secs_at_least, "seconds");
            shorthand!($i, 60, 1, from_minutes, as_minutes, from_minutes_at_least, "minutes");
            shorthand!($i, 3600, 1, from_hours, as_hours, from_hours_at_least, "hours");

            /// Convert the Duration to a floating point number of seconds.
            #[inline]
            pub const fn as_secs_f32(&self) -> f32 {
                let factor = const { F.factor_f32() };
                factor * self.ticks as f32
            }

            /// Convert the Duration to a floating point number of seconds.
            #[inline]
            pub const fn as_secs_f64(&self) -> f64 {
                let factor = const { F.factor_f64() };
                factor * self.ticks as f64
            }

            /// Create a duration from a floating point number of seconds.
            ///
            /// The value is rounded to the nearest tick.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let d = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_secs_f32(1.5);")]
            ///
            /// assert_eq!(d.as_ticks(), 1_500);
            ///
            #[doc = concat!("let d = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_secs_f32(1.5005);")]
            /// assert_eq!(d.as_ticks(), 1_501);
            /// ```
            #[inline]
            pub const fn from_secs_f32(secs: f32) -> Self {
                let factor = const { F.inv_factor_f32() };
                Self::from_ticks((secs * factor + 0.5) as $i)
            }

            /// Create a duration from a floating point number of seconds.
            ///
            /// The value is rounded to the nearest tick.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let d = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_secs_f64(1.5);")]
            ///
            /// assert_eq!(d.as_ticks(), 1_500);
            ///
            #[doc = concat!("let d = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_secs_f64(1.5005);")]
            /// assert_eq!(d.as_ticks(), 1_501);
            /// ```
            #[inline]
            pub const fn from_secs_f64(secs: f64) -> Self {
                let factor = const { F.inv_factor_f64() };
                Self::from_ticks((secs * factor + 0.5) as $i)
            }

            /// Shorthand for creating a duration which represents hertz.
            #[inline]
            #[allow(non_snake_case)]
            pub const fn Hz(val: $i) -> Self {
                Self::from_rate(crate::Hertz::<$i>::from_raw(val))
            }

            /// Shorthand for creating a duration which represents kilohertz.
            #[inline]
            #[allow(non_snake_case)]
            pub const fn kHz(val: $i) -> Self {
                Self::from_rate(crate::Kilohertz::<$i>::from_raw(val))
            }

            /// Shorthand for creating a duration which represents megahertz.
            #[inline]
            #[allow(non_snake_case)]
            pub const fn MHz(val: $i) -> Self {
                Self::from_rate(crate::Megahertz::<$i>::from_raw(val))
            }
        }

        impl<const L: Fraction, const R: Fraction>
            PartialOrd<Duration<$i, R>> for Duration<$i, L>
        {
            #[inline]
            fn partial_cmp(&self, other: &Duration<$i, R>) -> Option<Ordering> {
                self.const_partial_cmp(*other)
            }
        }

        impl<const F: Fraction> Ord for Duration<$i, F> {
            #[inline]
            fn cmp(&self, other: &Self) -> Ordering {
                Self::_const_cmp(self.ticks, other.ticks)
            }
        }

        impl<const L: Fraction, const R: Fraction>
            PartialEq<Duration<$i, R>> for Duration<$i, L>
        {
            #[inline]
            fn eq(&self, other: &Duration<$i, R>) -> bool {
                self.const_eq(*other)
            }
        }

        impl<const F: Fraction> Eq for Duration<$i, F> {}

        // Duration - Duration = Duration (only same base until const_generics_defaults is
        // stabilized)
        impl<const F: Fraction> ops::Sub for Duration<$i, F>
        {
            type Output = Self;

            #[inline]
            #[track_caller]
            fn sub(self, other: Self) -> Self::Output {
                if let Some(v) = self.checked_sub(other) {
                    v
                } else {
                    panic!("Sub failed!");
                }
            }
        }

        // Duration -= Duration
        impl<const F: Fraction> ops::SubAssign for Duration<$i, F>
        {
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                *self = *self - other;
            }
        }

        // Duration + Duration = Duration (only same base until const_generics_defaults is
        // stabilized)
        impl<const F: Fraction> ops::Add for Duration<$i, F>
        {
            type Output = Self;

            #[inline]
            #[track_caller]
            fn add(self, other: Self) -> Self::Output {
                if let Some(v) = self.checked_add(other) {
                    v
                } else {
                    panic!("Add failed!");
                }
            }
        }

        // Duration += Duration
        impl<const F: Fraction> ops::AddAssign for Duration<$i, F>
        {
            #[inline]
            fn add_assign(&mut self, other: Self) {
                *self = *self + other;
            }
        }

        // integer * Duration = Duration
        impl<const F: Fraction> ops::Mul<Duration<$i, F>> for u32 {
            type Output = Duration<$i, F>;

            #[inline]
            fn mul(self, mut other: Duration<$i, F>) -> Self::Output {
                other.ticks *= self as $i;
                other
            }
        }

        // Duration * integer = Duration
        impl<const F: Fraction> ops::Mul<u32> for Duration<$i, F> {
            type Output = Self;

            #[inline]
            fn mul(mut self, other: u32) -> Self::Output {
                self.ticks *= other as $i;
                self
            }
        }

        // Duration *= integer
        impl<const F: Fraction> ops::MulAssign<u32>
            for Duration<$i, F>
        {
            #[inline]
            fn mul_assign(&mut self, other: u32) {
                *self = *self * other;
            }
        }

        // Duration / integer = Duration
        impl<const F: Fraction> ops::Div<u32> for Duration<$i, F> {
            type Output = Self;

            #[inline]
            fn div(mut self, other: u32) -> Self::Output {
                self.ticks /= other as $i;
                self
            }
        }

        // Duration /= integer
        impl<const F: Fraction> ops::DivAssign<u32>
            for Duration<$i, F>
        {
            #[inline]
            fn div_assign(&mut self, other: u32) {
                *self = *self / other;
            }
        }

        // Duration % Duration = Duration (only same base until const_generics_defaults is
        // stabilized)
        impl<const F: Fraction> ops::Rem<Duration<$i, F>>
            for Duration<$i, F>
        {
            type Output = Self;

            #[inline]
            #[track_caller]
            fn rem(self, other: Self) -> Self::Output {
                if let Some(v) = self.checked_rem(other) {
                    v
                } else {
                    panic!("Rem failed!");
                }
            }
        }

        // Duration %= Duration
        impl<const F: Fraction> ops::RemAssign<Duration<$i, F>>
            for Duration<$i, F>
        {
            #[inline]
            fn rem_assign(&mut self, other: Self) {
                *self = *self % other;
            }
        }

        // Duration / Duration = integer
        impl<const L: Fraction, const R: Fraction> ops::Div<Duration<$i, R>>
            for Duration<$i, L>
        {
            type Output = $i;

            #[inline]
            #[track_caller]
            fn div(self, other: Duration<$i, R>) -> Self::Output {
                let conv: Duration<$i, R> = self.convert();
                conv.ticks / other.ticks
            }
        }

        #[cfg(feature = "defmt")]
        impl<const F: Fraction> defmt::Format for Duration<$i, F>
        {
            fn format(&self, f: defmt::Formatter) {
                if F.const_eq(Fraction::new(3600, 1)) {
                    defmt::write!(f, "{} h", self.ticks)
                } else if F.const_eq(Fraction::new(60, 1)) {
                    defmt::write!(f, "{} min", self.ticks)
                } else if F.const_eq(Fraction::ONE) {
                    defmt::write!(f, "{} s", self.ticks)
                } else if F.const_eq(Fraction::MILLI) {
                    defmt::write!(f, "{} ms", self.ticks)
                } else if F.const_eq(Fraction::MICRO) {
                    defmt::write!(f, "{} us", self.ticks)
                } else if F.const_eq(Fraction::NANO) {
                    defmt::write!(f, "{} ns", self.ticks)
                } else {
                    defmt::write!(f, "{} ticks @ ({}/{})", self.ticks, F.num, F.denom)
                }
            }
        }

        impl<const F: Fraction> core::fmt::Display for Duration<$i, F> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                if F.num == 3_600 && F.denom == 1 {
                    write!(f, "{} h", self.ticks)
                } else if F.num == 60 && F.denom == 1 {
                    write!(f, "{} min", self.ticks)
                } else if F.num == 1 && F.denom == 1 {
                    write!(f, "{} s", self.ticks)
                } else if F.num == 1 && F.denom == 1_000 {
                    write!(f, "{} ms", self.ticks)
                } else if F.num == 1 && F.denom == 1_000_000 {
                    write!(f, "{} us", self.ticks)
                } else if F.num == 1 && F.denom == 1_000_000_000 {
                    write!(f, "{} ns", self.ticks)
                } else {
                    write!(f, "{} ticks @ ({}/{})", self.ticks, F.num, F.denom)
                }
            }
        }
    };
}

impl_duration_for_integer!(u32);
impl_duration_for_integer!(u64);

//
// Conversion from core::time::Duration
//

impl<const F: Fraction> convert::TryFrom<core::time::Duration> for Duration<u32, F> {
    type Error = ();

    #[inline]
    fn try_from(val: core::time::Duration) -> Result<Self, Self::Error> {
        Duration::<u64, F>::try_from(val)?.try_into()
    }
}

impl<const F: Fraction> convert::TryFrom<core::time::Duration> for Duration<u64, F> {
    type Error = ();

    #[inline]
    fn try_from(val: core::time::Duration) -> Result<Self, Self::Error> {
        let secs_duration = SecsDurationU64::from_ticks(val.as_secs());
        let nanos_duration = NanosDurationU64::from_ticks(val.subsec_nanos() as u64);

        let secs_converted: Self = secs_duration.const_try_into().ok_or(())?;
        let nanos_converted: Self = nanos_duration.const_try_into().ok_or(())?;

        secs_converted.checked_add(nanos_converted).ok_or(())
    }
}

//
// Conversion to core::time::Duration
//

impl<const F: Fraction> From<Duration<u32, F>> for core::time::Duration {
    #[inline]
    fn from(val: Duration<u32, F>) -> Self {
        let val_u64: Duration<u64, F> = val.into();
        core::time::Duration::from(val_u64)
    }
}

impl<const F: Fraction> From<Duration<u64, F>> for core::time::Duration {
    #[inline]
    fn from(val: Duration<u64, F>) -> Self {
        let secs = val.as_secs();
        let secs_duration = Duration::<u64, F>::from_secs(secs);
        let remainder = val.saturating_sub(secs_duration);
        let nanos = remainder.as_nanos() as u32;

        core::time::Duration::new(secs, nanos)
    }
}

//
// Operations between u32 and u64 Durations
//

impl<const F: Fraction> From<Duration<u32, F>> for Duration<u64, F> {
    #[inline]
    fn from(val: Duration<u32, F>) -> Self {
        Duration::<u64, F>::from_ticks(val.as_ticks() as u64)
    }
}

impl<const F: Fraction> convert::TryFrom<Duration<u64, F>> for Duration<u32, F> {
    type Error = ();

    #[inline]
    fn try_from(val: Duration<u64, F>) -> Result<Self, ()> {
        Ok(Self::from_ticks(val.as_ticks().try_into().map_err(|_| ())?))
    }
}

// Duration - Duration = Duration (to make shorthands work, until const_generics_defaults is
// stabilized)
impl<const F: Fraction> ops::Sub<Duration<u32, F>> for Duration<u64, F> {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn sub(self, other: Duration<u32, F>) -> Self::Output {
        if let Some(v) = self.checked_sub(Self::from_ticks(other.as_ticks() as u64)) {
            v
        } else {
            panic!("Sub failed!");
        }
    }
}

// Duration -= Duration (to make shorthands work, until const_generics_defaults is stabilized)
impl<const F: Fraction> ops::SubAssign<Duration<u32, F>> for Duration<u64, F> {
    #[inline]
    fn sub_assign(&mut self, other: Duration<u32, F>) {
        *self = *self - other;
    }
}

// Duration + Duration = Duration (to make shorthands work, until const_generics_defaults is
// stabilized)
impl<const F: Fraction> ops::Add<Duration<u32, F>> for Duration<u64, F> {
    type Output = Self;

    #[inline]
    #[track_caller]
    fn add(self, other: Duration<u32, F>) -> Self::Output {
        if let Some(v) = self.checked_add(Self::from_ticks(other.as_ticks() as u64)) {
            v
        } else {
            panic!("Add failed!");
        }
    }
}

// Duration += Duration (to make shorthands work, until const_generics_defaults is stabilized)
impl<const F: Fraction> ops::AddAssign<Duration<u32, F>> for Duration<u64, F> {
    #[inline]
    fn add_assign(&mut self, other: Duration<u32, F>) {
        *self = *self + other;
    }
}

impl<const L: Fraction, const R: Fraction> PartialOrd<Duration<u32, R>> for Duration<u64, L> {
    #[inline]
    fn partial_cmp(&self, other: &Duration<u32, R>) -> Option<Ordering> {
        self.partial_cmp(&Duration::<u64, R>::from_ticks(other.as_ticks() as u64))
    }
}

impl<const L: Fraction, const R: Fraction> PartialEq<Duration<u32, R>> for Duration<u64, L> {
    #[inline]
    fn eq(&self, other: &Duration<u32, R>) -> bool {
        self.eq(&Duration::<u64, R>::from_ticks(other.as_ticks() as u64))
    }
}

impl<const L: Fraction, const R: Fraction> PartialOrd<Duration<u64, R>> for Duration<u32, L> {
    #[inline]
    fn partial_cmp(&self, other: &Duration<u64, R>) -> Option<Ordering> {
        Duration::<u64, L>::from_ticks(self.ticks as u64).partial_cmp(other)
    }
}

impl<const L: Fraction, const R: Fraction> PartialEq<Duration<u64, R>> for Duration<u32, L> {
    #[inline]
    fn eq(&self, other: &Duration<u64, R>) -> bool {
        Duration::<u64, L>::from_ticks(self.ticks as u64).eq(other)
    }
}

/// Extension trait for simple short-hands for u32 Durations
pub trait ExtU32 {
    /// Shorthand for creating a duration which represents picoseconds.
    fn picos<const F: Fraction>(self) -> Duration<u32, F>;

    /// Shorthand for creating a duration which represents nanoseconds.
    fn nanos<const F: Fraction>(self) -> Duration<u32, F>;

    /// Shorthand for creating a duration which represents microseconds.
    fn micros<const F: Fraction>(self) -> Duration<u32, F>;

    /// Shorthand for creating a duration which represents milliseconds.
    fn millis<const F: Fraction>(self) -> Duration<u32, F>;

    /// Shorthand for creating a duration which represents seconds.
    fn secs<const F: Fraction>(self) -> Duration<u32, F>;

    /// Shorthand for creating a duration which represents minutes.
    fn minutes<const F: Fraction>(self) -> Duration<u32, F>;

    /// Shorthand for creating a duration which represents hours.
    fn hours<const F: Fraction>(self) -> Duration<u32, F>;
}

impl ExtU32 for u32 {
    #[inline]
    fn picos<const F: Fraction>(self) -> Duration<u32, F> {
        Duration::<u32, F>::from_picos(self)
    }

    #[inline]
    fn nanos<const F: Fraction>(self) -> Duration<u32, F> {
        Duration::<u32, F>::from_nanos(self)
    }

    #[inline]
    fn micros<const F: Fraction>(self) -> Duration<u32, F> {
        Duration::<u32, F>::from_micros(self)
    }

    #[inline]
    fn millis<const F: Fraction>(self) -> Duration<u32, F> {
        Duration::<u32, F>::from_millis(self)
    }

    #[inline]
    fn secs<const F: Fraction>(self) -> Duration<u32, F> {
        Duration::<u32, F>::from_secs(self)
    }

    #[inline]
    fn minutes<const F: Fraction>(self) -> Duration<u32, F> {
        Duration::<u32, F>::from_minutes(self)
    }

    #[inline]
    fn hours<const F: Fraction>(self) -> Duration<u32, F> {
        Duration::<u32, F>::from_hours(self)
    }
}

/// Extension trait for simple short-hands for u32 Durations (ceil rounded)
pub trait ExtU32Ceil {
    /// Shorthand for creating a duration which represents picoseconds.
    fn picos_at_least<const F: Fraction>(self) -> Duration<u32, F>;

    /// Shorthand for creating a duration which represents nanoseconds.
    fn nanos_at_least<const F: Fraction>(self) -> Duration<u32, F>;

    /// Shorthand for creating a duration which represents microseconds.
    fn micros_at_least<const F: Fraction>(self) -> Duration<u32, F>;

    /// Shorthand for creating a duration which represents milliseconds.
    fn millis_at_least<const F: Fraction>(self) -> Duration<u32, F>;

    /// Shorthand for creating a duration which represents seconds.
    fn secs_at_least<const F: Fraction>(self) -> Duration<u32, F>;

    /// Shorthand for creating a duration which represents minutes.
    fn minutes_at_least<const F: Fraction>(self) -> Duration<u32, F>;

    /// Shorthand for creating a duration which represents hours.
    fn hours_at_least<const F: Fraction>(self) -> Duration<u32, F>;
}

impl ExtU32Ceil for u32 {
    #[inline]
    fn picos_at_least<const F: Fraction>(self) -> Duration<u32, F> {
        Duration::<u32, F>::from_picos_at_least(self)
    }

    #[inline]
    fn nanos_at_least<const F: Fraction>(self) -> Duration<u32, F> {
        Duration::<u32, F>::from_nanos_at_least(self)
    }

    #[inline]
    fn micros_at_least<const F: Fraction>(self) -> Duration<u32, F> {
        Duration::<u32, F>::from_micros_at_least(self)
    }

    #[inline]
    fn millis_at_least<const F: Fraction>(self) -> Duration<u32, F> {
        Duration::<u32, F>::from_millis_at_least(self)
    }

    #[inline]
    fn secs_at_least<const F: Fraction>(self) -> Duration<u32, F> {
        Duration::<u32, F>::from_secs_at_least(self)
    }

    #[inline]
    fn minutes_at_least<const F: Fraction>(self) -> Duration<u32, F> {
        Duration::<u32, F>::from_minutes_at_least(self)
    }

    #[inline]
    fn hours_at_least<const F: Fraction>(self) -> Duration<u32, F> {
        Duration::<u32, F>::from_hours_at_least(self)
    }
}

/// Extension trait for simple short-hands for u64 Durations
pub trait ExtU64 {
    /// Shorthand for creating a duration which represents picoseconds.
    fn picos<const F: Fraction>(self) -> Duration<u64, F>;

    /// Shorthand for creating a duration which represents nanoseconds.
    fn nanos<const F: Fraction>(self) -> Duration<u64, F>;

    /// Shorthand for creating a duration which represents microseconds.
    fn micros<const F: Fraction>(self) -> Duration<u64, F>;

    /// Shorthand for creating a duration which represents milliseconds.
    fn millis<const F: Fraction>(self) -> Duration<u64, F>;

    /// Shorthand for creating a duration which represents seconds.
    fn secs<const F: Fraction>(self) -> Duration<u64, F>;

    /// Shorthand for creating a duration which represents minutes.
    fn minutes<const F: Fraction>(self) -> Duration<u64, F>;

    /// Shorthand for creating a duration which represents hours.
    fn hours<const F: Fraction>(self) -> Duration<u64, F>;
}

impl ExtU64 for u64 {
    #[inline]
    fn picos<const F: Fraction>(self) -> Duration<u64, F> {
        Duration::<u64, F>::from_picos(self)
    }

    #[inline]
    fn nanos<const F: Fraction>(self) -> Duration<u64, F> {
        Duration::<u64, F>::from_nanos(self)
    }

    #[inline]
    fn micros<const F: Fraction>(self) -> Duration<u64, F> {
        Duration::<u64, F>::from_micros(self)
    }

    #[inline]
    fn millis<const F: Fraction>(self) -> Duration<u64, F> {
        Duration::<u64, F>::from_millis(self)
    }

    #[inline]
    fn secs<const F: Fraction>(self) -> Duration<u64, F> {
        Duration::<u64, F>::from_secs(self)
    }

    #[inline]
    fn minutes<const F: Fraction>(self) -> Duration<u64, F> {
        Duration::<u64, F>::from_minutes(self)
    }

    #[inline]
    fn hours<const F: Fraction>(self) -> Duration<u64, F> {
        Duration::<u64, F>::from_hours(self)
    }
}

/// Extension trait for simple short-hands for u64 Durations (ceil rounded)
pub trait ExtU64Ceil {
    /// Shorthand for creating a duration which represents picoseconds.
    fn picos_at_least<const F: Fraction>(self) -> Duration<u64, F>;

    /// Shorthand for creating a duration which represents nanoseconds.
    fn nanos_at_least<const F: Fraction>(self) -> Duration<u64, F>;

    /// Shorthand for creating a duration which represents microseconds.
    fn micros_at_least<const F: Fraction>(self) -> Duration<u64, F>;

    /// Shorthand for creating a duration which represents milliseconds.
    fn millis_at_least<const F: Fraction>(self) -> Duration<u64, F>;

    /// Shorthand for creating a duration which represents seconds.
    fn secs_at_least<const F: Fraction>(self) -> Duration<u64, F>;

    /// Shorthand for creating a duration which represents minutes.
    fn minutes_at_least<const F: Fraction>(self) -> Duration<u64, F>;

    /// Shorthand for creating a duration which represents hours.
    fn hours_at_least<const F: Fraction>(self) -> Duration<u64, F>;
}

impl ExtU64Ceil for u64 {
    #[inline]
    fn picos_at_least<const F: Fraction>(self) -> Duration<u64, F> {
        Duration::<u64, F>::from_picos_at_least(self)
    }

    #[inline]
    fn nanos_at_least<const F: Fraction>(self) -> Duration<u64, F> {
        Duration::<u64, F>::from_nanos_at_least(self)
    }

    #[inline]
    fn micros_at_least<const F: Fraction>(self) -> Duration<u64, F> {
        Duration::<u64, F>::from_micros_at_least(self)
    }

    #[inline]
    fn millis_at_least<const F: Fraction>(self) -> Duration<u64, F> {
        Duration::<u64, F>::from_millis_at_least(self)
    }

    #[inline]
    fn secs_at_least<const F: Fraction>(self) -> Duration<u64, F> {
        Duration::<u64, F>::from_secs_at_least(self)
    }

    #[inline]
    fn minutes_at_least<const F: Fraction>(self) -> Duration<u64, F> {
        Duration::<u64, F>::from_minutes_at_least(self)
    }

    #[inline]
    fn hours_at_least<const F: Fraction>(self) -> Duration<u64, F> {
        Duration::<u64, F>::from_hours_at_least(self)
    }
}
