use super::Fraction;
use crate::duration::Duration;
use crate::helpers::Helpers;
use core::cmp::Ordering;
use core::ops;

/// Represents an instant in time.
///
/// The generic `T` can either be `u32` or `u64`, and the const generics represent the ratio of the
/// ticks contained within the instant: `instant in seconds = NOM / DENOM * ticks`
#[derive(Clone, Copy, Debug)]
pub struct Instant<T, const F: Fraction> {
    ticks: T,
}

macro_rules! impl_instant_for_integer {
    ($i:ty) => {
        impl<const F: Fraction> Instant<$i, F> {
            /// Create an `Instant` from a ticks value.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let _i = Instant::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(1);")]
            /// ```
            #[inline]
            pub const fn from_ticks(ticks: $i) -> Self {
                assert!(F.num > 0);
                assert!(F.denom > 0);

                Instant { ticks }
            }

            /// Extract the ticks from an `Instant`.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i = Instant::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(234);")]
            ///
            /// assert_eq!(i.ticks(), 234);
            /// ```
            #[inline]
            pub const fn ticks(&self) -> $i {
                self.ticks
            }

            /// Const comparison of `Instant`s.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i1 = Instant::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(1);")]
            #[doc = concat!("let i2 = Instant::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(2);")]
            ///
            /// assert_eq!(i1.const_cmp(i2), core::cmp::Ordering::Less);
            /// ```
            ///
            /// This function takes into account that ticks might wrap around. If the absolute
            /// values of `self` and `other` differ by more than half the possible range, it is
            /// assumed that an overflow occured and the result is reversed:
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i1 = Instant::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(", stringify!($i),"::MAX);")]
            #[doc = concat!("let i2 = Instant::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(1);")]
            ///
            /// assert_eq!(i1.const_cmp(i2), core::cmp::Ordering::Less);
            /// ```
            #[inline]
            pub const fn const_cmp(self, other: Self) -> Ordering {
                if self.ticks == other.ticks {
                    Ordering::Equal
                } else {
                    let v = self.ticks.wrapping_sub(other.ticks);

                    // not using `v.cmp(<$i>::MAX / 2).reverse()` due to `cmp` being non-const
                    if v > <$i>::MAX / 2 {
                        Ordering::Less
                    } else if v < <$i>::MAX / 2 {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                }
            }

            /// Duration between since the start of the `Instant`. This assumes an instant which
            /// won't wrap within the execution of the program.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i = Instant::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(11);")]
            ///
            /// assert_eq!(i.duration_since_epoch().ticks(), 11);
            /// ```
            #[inline]
            pub const fn duration_since_epoch(self) -> Duration<$i, F> {
                Duration::<$i, F>::from_ticks(self.ticks())
            }

            /// Duration between `Instant`s.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i1 = Instant::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(1);")]
            #[doc = concat!("let i2 = Instant::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(2);")]
            ///
            /// assert_eq!(i1.checked_duration_since(i2), None);
            /// assert_eq!(i2.checked_duration_since(i1).unwrap().ticks(), 1);
            /// ```
            #[inline]
            pub const fn checked_duration_since(
                self,
                other: Self,
            ) -> Option<Duration<$i, F>> {
                match self.const_cmp(other) {
                    Ordering::Greater | Ordering::Equal => {
                        Some(Duration::<$i, F>::from_ticks(
                            self.ticks.wrapping_sub(other.ticks),
                        ))
                    }
                    Ordering::Less => None,
                }
            }

            /// Subtract a `Duration` from an `Instant` while checking for overflow.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i = Instant::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(1);")]
            #[doc = concat!("let d = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(1);")]
            ///
            /// assert_eq!(i.checked_sub_duration(d).unwrap().ticks(), 0);
            /// ```
            pub const fn checked_sub_duration<const O: Fraction>(
                self,
                other: Duration<$i, O>,
            ) -> Option<Self> {
                if Helpers::<F, O>::SAME_BASE {
                    Some(Self::from_ticks(
                        self.ticks.wrapping_sub(other.ticks()),
                    ))
                } else {
                    if let Some(lh) = other
                        .ticks()
                        .checked_mul(Helpers::<F, O>::LD_TIMES_RN as $i)
                    {
                        let ticks = lh / Helpers::<F, O>::RD_TIMES_LN as $i;

                        Some(Self::from_ticks(
                            self.ticks.wrapping_sub(ticks),
                        ))
                    } else {
                        None
                    }
                }
            }

            /// Add a `Duration` to an `Instant` while checking for overflow.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i = Instant::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(1);")]
            #[doc = concat!("let d = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(1);")]
            ///
            /// assert_eq!(i.checked_add_duration(d).unwrap().ticks(), 2);
            /// ```
            pub const fn checked_add_duration<const O: Fraction>(
                self,
                other: Duration<$i, O>,
            ) -> Option<Self> {
                if Helpers::<F, O>::SAME_BASE {
                    Some(Self::from_ticks(
                        self.ticks.wrapping_add(other.ticks()),
                    ))
                } else {
                    if let Some(lh) = other
                        .ticks()
                        .checked_mul(Helpers::<F, O>::LD_TIMES_RN as $i)
                    {
                        let ticks = lh / Helpers::<F, O>::RD_TIMES_LN as $i;

                        Some(Self::from_ticks(
                            self.ticks.wrapping_add(ticks),
                        ))
                    } else {
                        None
                    }
                }
            }
        }

        impl<const F: Fraction> PartialOrd for Instant<$i, F> {
            /// This implementation deviates from the definition of
            /// [PartialOrd::partial_cmp](core::cmp::PartialOrd::partial_cmp):
            ///
            /// It takes into account that ticks might wrap around. If the absolute
            /// values of `self` and `other` differ by more than half the possible range, it is
            /// assumed that an overflow occured and the result is reversed.
            ///
            /// That breaks the transitivity invariant: a < b and b < c no longer implies a < c.
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.const_cmp(*other))
            }
        }

        impl<const F: Fraction> Ord for Instant<$i, F> {
            /// This implementation deviates from the definition of
            /// [Ord::cmp](core::cmp::Ord::cmp):
            ///
            /// It takes into account that ticks might wrap around. If the absolute
            /// values of `self` and `other` differ by more than half the possible range, it is
            /// assumed that an overflow occured and the result is reversed.
            ///
            /// That breaks the transitivity invariant: a < b and b < c no longer implies a < c.
            #[inline]
            fn cmp(&self, other: &Self) -> Ordering {
                self.const_cmp(*other)
            }
        }

        impl<const F: Fraction> PartialEq for Instant<$i, F> {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.ticks.eq(&other.ticks)
            }
        }

        impl<const F: Fraction> Eq for Instant<$i, F> {}

        // Instant - Instant = Duration
        // We have limited this to use same numerator and denominator in both left and right hand sides,
        // this allows for the extension traits to work. For usage with different fraction, use
        // `checked_duration_since`.
        impl<const F: Fraction> ops::Sub<Instant<$i, F>>
            for Instant<$i, F>
        {
            type Output = Duration<$i, F>;

            #[inline]
            fn sub(self, other: Self) -> Self::Output {
                if let Some(v) = self.checked_duration_since(other) {
                    v
                } else {
                    panic!("Sub failed! Other > self");
                }
            }
        }

        // Instant - Duration = Instant
        // We have limited this to use same numerator and denominator in both left and right hand sides,
        // this allows for the extension traits to work. For usage with different fraction, use
        // `checked_sub_duration`.
        impl<const F: Fraction> ops::Sub<Duration<$i, F>>
            for Instant<$i, F>
        {
            type Output = Self;

            #[inline]
            fn sub(self, other: Duration<$i, F>) -> Self::Output {
                if let Some(v) = self.checked_sub_duration(other) {
                    v
                } else {
                    panic!("Sub failed! Overflow");
                }
            }
        }

        // Instant -= Duration
        // We have limited this to use same numerator and denominator in both left and right hand sides,
        // this allows for the extension traits to work. For usage with different fraction, use
        // `checked_sub_duration`.
        impl<const F: Fraction> ops::SubAssign<Duration<$i, F>>
            for Instant<$i, F>
        {
            #[inline]
            fn sub_assign(&mut self, other: Duration<$i, F>) {
                *self = *self - other;
            }
        }

        // Instant + Duration = Instant
        // We have limited this to use same numerator and denominator in both left and right hand sides,
        // this allows for the extension traits to work. For usage with different fraction, use
        // `checked_add_duration`.
        impl<const F: Fraction> ops::Add<Duration<$i, F>>
            for Instant<$i, F>
        {
            type Output = Self;

            #[inline]
            fn add(self, other: Duration<$i, F>) -> Self::Output {
                if let Some(v) = self.checked_add_duration(other) {
                    v
                } else {
                    panic!("Add failed! Overflow");
                }
            }
        }

        // Instant += Duration
        // We have limited this to use same numerator and denominator in both left and right hand sides,
        // this allows for the extension traits to work. For usage with different fraction, use
        // `checked_add_duration`.
        impl<const F: Fraction> ops::AddAssign<Duration<$i, F>>
            for Instant<$i, F>
        {
            #[inline]
            fn add_assign(&mut self, other: Duration<$i, F>) {
                *self = *self + other;
            }
        }

        #[cfg(feature = "defmt")]
        impl<const F: Fraction> defmt::Format for Instant<$i, F> {
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

        impl<const F: Fraction> core::fmt::Display for Instant<$i, F> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                if F.const_eq(Fraction::new(3600, 1)) {
                    write!(f, "{} h", self.ticks)
                } else if F.const_eq(Fraction::new(60, 1)) {
                    write!(f, "{} min", self.ticks)
                } else if F.const_eq(Fraction::ONE) {
                    write!(f, "{} s", self.ticks)
                } else if F.const_eq(Fraction::MILLI) {
                    write!(f, "{} ms", self.ticks)
                } else if F.const_eq(Fraction::MICRO) {
                    write!(f, "{} us", self.ticks)
                } else if F.const_eq(Fraction::NANO) {
                    write!(f, "{} ns", self.ticks)
                } else {
                    write!(f, "{} ticks @ ({}/{})", self.ticks, F.num, F.denom)
                }
            }
        }
    };
}

impl_instant_for_integer!(u32);
impl_instant_for_integer!(u64);

//
// Operations between u32 Duration and u64 Instant
//

// Instant - Duration = Instant
// We have limited this to use same numerator and denominator in both left and right hand sides,
// this allows for the extension traits to work. For usage with different fraction, use
// `checked_sub_duration`.
impl<const F: Fraction> ops::Sub<Duration<u32, F>> for Instant<u64, F> {
    type Output = Self;

    #[inline]
    fn sub(self, other: Duration<u32, F>) -> Self::Output {
        if let Some(v) = self.checked_sub_duration(other.into()) {
            v
        } else {
            panic!("Sub failed! Overflow");
        }
    }
}

// Instant -= Duration
// We have limited this to use same numerator and denominator in both left and right hand sides,
// this allows for the extension traits to work. For usage with different fraction, use
// `checked_sub_duration`.
impl<const F: Fraction> ops::SubAssign<Duration<u32, F>> for Instant<u64, F> {
    #[inline]
    fn sub_assign(&mut self, other: Duration<u32, F>) {
        *self = *self - other;
    }
}

// Instant + Duration = Instant
// We have limited this to use same numerator and denominator in both left and right hand sides,
// this allows for the extension traits to work. For usage with different fraction, use
// `checked_add_duration`.
impl<const F: Fraction> ops::Add<Duration<u32, F>> for Instant<u64, F> {
    type Output = Instant<u64, F>;

    #[inline]
    fn add(self, other: Duration<u32, F>) -> Self::Output {
        if let Some(v) = self.checked_add_duration(other.into()) {
            v
        } else {
            panic!("Add failed! Overflow");
        }
    }
}

// Instant += Duration
// We have limited this to use same numerator and denominator in both left and right hand sides,
// this allows for the extension traits to work. For usage with different fraction, use
// `checked_add_duration`.
impl<const F: Fraction> ops::AddAssign<Duration<u32, F>> for Instant<u64, F> {
    #[inline]
    fn add_assign(&mut self, other: Duration<u32, F>) {
        *self = *self + other;
    }
}

// impl<const L: Fraction, const R: Fraction>
//     ops::Add<Duration<u32, R>> for Duration<u64, L>
// {
//     type Output = Self;
//
//     #[inline]
//     fn add(self, other: Duration<u32, R>) -> Self::Output {
//         self.add(Duration::<u64, L>::from_ticks(
//             other.ticks() as u64
//         ))
//     }
// }
