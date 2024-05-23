use super::Fraction;
use crate::helpers::Helpers;
use crate::Duration;
use core::cmp::Ordering;
use core::convert;
use core::ops;

/// Represents a frequency.
///
/// The generic `T` can either be `u32` or `u64`, and the const generics represent the ratio of the
/// raw contained within the rate: `rate in Hz = NOM / DENOM * raw`
#[derive(Clone, Copy, Debug)]
pub struct Rate<T, const F: Fraction> {
    pub(crate) raw: T,
}

macro_rules! impl_rate_for_integer {
    ($i:ty) => {
        impl<const F: Fraction> Rate<$i, F> {
            /// Create a `Rate` from a raw value.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let _d = Rate::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_raw(1);")]
            /// ```
            #[inline]
            pub const fn from_raw(raw: $i) -> Self {
                assert!(F.num > 0);
                assert!(F.denom > 0);

                Rate { raw }
            }

            /// Extract the raw value from a `Rate`.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let d = Rate::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_raw(234);")]
            ///
            /// assert_eq!(d.raw(), 234);
            /// ```
            #[inline]
            pub const fn raw(&self) -> $i {
                self.raw
            }

            /// Add two rates while checking for overflow.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let r1 = Rate::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_raw(1);")]
            #[doc = concat!("let r2 = Rate::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_raw(2);")]
            #[doc = concat!("let r3 = Rate::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_raw(", stringify!($i), "::MAX);")]
            ///
            /// assert_eq!(r1.checked_add(r2).unwrap().raw(), 3);
            /// assert_eq!(r1.checked_add(r3), None);
            /// ```
            pub const fn checked_add<const O: Fraction>(
                self,
                other: Rate<$i, O>,
            ) -> Option<Self> {
                if Helpers::<F, O>::SAME_BASE {
                    if let Some(raw) = self.raw.checked_add(other.raw) {
                        Some(Self::from_raw(raw))
                    } else {
                        None
                    }
                } else {
                    if let Some(lh) = other
                        .raw
                        .checked_mul(Helpers::<F, O>::LD_TIMES_RN as $i)
                    {
                        let raw = lh / Helpers::<F, O>::RD_TIMES_LN as $i;

                        if let Some(raw) = self.raw.checked_add(raw) {
                            Some(Self::from_raw(raw))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            }

            /// Subtract two rates while checking for overflow.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let r1 = Rate::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_raw(1);")]
            #[doc = concat!("let r2 = Rate::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_raw(2);")]
            #[doc = concat!("let r3 = Rate::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_raw(", stringify!($i), "::MAX);")]
            ///
            /// assert_eq!(r2.checked_sub(r1).unwrap().raw(), 1);
            /// assert_eq!(r1.checked_sub(r3), None);
            /// ```
            pub const fn checked_sub<const O: Fraction>(
                self,
                other: Rate<$i, O>,
            ) -> Option<Self> {
                if Helpers::<F, O>::SAME_BASE {
                    if let Some(raw) = self.raw.checked_sub(other.raw) {
                        Some(Self::from_raw(raw))
                    } else {
                        None
                    }
                } else {
                    if let Some(lh) = other
                        .raw
                        .checked_mul(Helpers::<F, O>::LD_TIMES_RN as $i)
                    {
                        let raw = lh / Helpers::<F, O>::RD_TIMES_LN as $i;

                        if let Some(raw) = self.raw.checked_sub(raw) {
                            Some(Self::from_raw(raw))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
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
            #[doc = concat!("let r1 = Rate::<", stringify!($i), ", { Fraction::new(1, 1_00) }>::from_raw(1);")]
            #[doc = concat!("let r2 = Rate::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_raw(1);")]
            ///
            /// assert_eq!(r1.const_partial_cmp(r2), Some(core::cmp::Ordering::Greater));
            /// ```
            #[inline]
            pub const fn const_partial_cmp<const R: Fraction>(
                self,
                other: Rate<$i, R>
            ) -> Option<Ordering> {
                if Helpers::<F, R>::SAME_BASE {
                    // If we are in the same base, comparison in trivial
                    Some(Self::_const_cmp(self.raw, other.raw))
                } else {
                    let lh = self
                        .raw
                        .checked_mul(Helpers::<F, R>::RD_TIMES_LN as $i);
                    let rh = other
                        .raw
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
            #[doc = concat!("let r1 = Rate::<", stringify!($i), ", { Fraction::new(1, 1_00) }>::from_raw(1);")]
            #[doc = concat!("let r2 = Rate::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_raw(10);")]
            ///
            /// assert!(r1.const_eq(r2));
            /// ```
            #[inline]
            pub const fn const_eq<const R: Fraction>(
                self,
                other: Rate<$i, R>
            ) -> bool {
                if Helpers::<F, R>::SAME_BASE {
                    // If we are in the same base, comparison in trivial
                    self.raw == other.raw
                } else {
                    let lh = self
                        .raw
                        .checked_mul(Helpers::<F, R>::RD_TIMES_LN as $i);
                    let rh = other
                        .raw
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
            #[doc = concat!("let r1 = Rate::<", stringify!($i), ", { Fraction::new(1, 1_00) }>::from_raw(1);")]
            #[doc = concat!("let r2 = Rate::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::const_try_from(r1);")]
            ///
            /// assert_eq!(r2.unwrap().raw(), 10);
            /// ```
            pub const fn const_try_from<const I: Fraction>(
                rate: Rate<$i, I>,
            ) -> Option<Self> {
                if Helpers::<I, F>::SAME_BASE {
                    Some(Self::from_raw(rate.raw))
                } else {
                    if let Some(lh) = (rate.raw as u64)
                        .checked_mul(Helpers::<I, F>::RD_TIMES_LN)
                    {
                        let raw = lh / Helpers::<I, F>::LD_TIMES_RN;

                        if raw <= <$i>::MAX as u64 {
                            Some(Self::from_raw(raw as $i))
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
            #[doc = concat!("let r1 = Rate::<", stringify!($i), ", { Fraction::new(1, 1_00) }>::from_raw(1);")]
            #[doc = concat!("let r2: Option<Rate::<", stringify!($i), ", { Fraction::new(1, 1_000) }>> = r1.const_try_into();")]
            ///
            /// assert_eq!(r2.unwrap().raw(), 10);
            /// ```
            #[inline]
            pub const fn const_try_into<const O: Fraction>(
                self,
            ) -> Option<Rate<$i, O>> {
                Rate::<$i, O>::const_try_from(self)
            }

            /// Const try into duration, checking for divide-by-zero.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let r1 = Rate::<", stringify!($i), ", { Fraction::new(1, 1) }>::from_raw(1);")]
            #[doc = concat!("let d1: Option<Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>> = r1.try_into_duration();")]
            ///
            /// assert_eq!(d1.unwrap().ticks(), 1_000);
            /// ```
            pub const fn try_into_duration<const O: Fraction>(
                self,
            ) -> Option<Duration<$i, O>> {
                Duration::<$i, O>::try_from_rate(self)
            }

            /// Convert from rate to duration.
            pub const fn into_duration<const O: Fraction>(
                self,
            ) -> Duration<$i, O> {
                if let Some(v) = self.try_into_duration() {
                    v
                } else {
                    panic!("Into duration failed, divide-by-zero!");
                }
            }

            /// Const try from duration, checking for divide-by-zero.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let d1 = Duration::<", stringify!($i), ", { Fraction::new(1, 1_000) }>::from_ticks(2);")]
            #[doc = concat!("let r1 = Rate::<", stringify!($i), ", { Fraction::new(1, 1) }>::try_from_duration(d1);")]
            ///
            /// assert_eq!(r1.unwrap().raw(), 500);
            /// ```
            #[inline]
            pub const fn try_from_duration<const I: Fraction>(
                duration: Duration<$i, I>,
            ) -> Option<Self> {
                if duration.ticks > 0 {
                    Some(Self::from_raw(
                        Helpers::<I, F>::RATE_TO_DURATION_NUMERATOR as $i
                        / duration.ticks
                    ))
                } else {
                    None
                }
            }

            /// Convert from duration to rate.
            #[inline]
            pub const fn from_duration<const I: Fraction>(
                duration: Duration<$i, I>,
            ) -> Self {
                if let Some(v) = Self::try_from_duration(duration) {
                    v
                } else {
                    panic!("From duration failed, divide-by-zero!");
                }
            }

            /// Convert between bases for a rate.
            ///
            /// Unfortunately not a `From` impl due to collision with the std lib.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let r1 = Rate::<", stringify!($i), ", { Fraction::new(1, 100) }>::from_raw(1);")]
            #[doc = concat!("let r2: Rate::<", stringify!($i), ", { Fraction::new(1, 1_000) }> = r1.convert();")]
            ///
            /// assert_eq!(r2.raw(), 10);
            /// ```
            ///
            /// Can be used in const contexts. Compilation will fail if the conversion causes overflow
            ///
            /// ```compile_fail
            /// # use fugit::*;
            #[doc = concat!("const RAW: ", stringify!($i), "= ", stringify!($i), "::MAX - 10;")]
            #[doc = concat!("const R1: Rate::<", stringify!($i), ", { Fraction::new(1, 100) }> = Rate::<", stringify!($i), ", { Fraction::new(1, 100) }>::from_raw(RAW);")]
            /// // Fails conversion due to overflow
            #[doc = concat!("const R2: Rate::<", stringify!($i), ", { Fraction::new(1, 200) }> = R1.convert();")]
            /// ```
            pub const fn convert<const O: Fraction>(
                self,
            ) -> Rate<$i, O> {
                if let Some(v) = self.const_try_into() {
                    v
                } else {
                    panic!("Convert failed!");
                }
            }

            /// Convert the Rate to an interger number of Hz.
            #[inline]
            #[allow(non_snake_case)]
            pub const fn to_Hz(&self) -> $i {
                    (Helpers::<{ Fraction::ONE }, F>::LD_TIMES_RN as $i * self.raw)
                        / Helpers::<{ Fraction::ONE }, F>::RD_TIMES_LN as $i
            }

            /// Convert the Rate to an interger number of kHz.
            #[inline]
            #[allow(non_snake_case)]
            pub const fn to_kHz(&self) -> $i {
                    (Helpers::<{ Fraction::KILO }, F>::LD_TIMES_RN as $i * self.raw)
                        / Helpers::<{ Fraction::KILO }, F>::RD_TIMES_LN as $i
            }

            /// Convert the Rate to an interger number of MHz.
            #[inline]
            #[allow(non_snake_case)]
            pub const fn to_MHz(&self) -> $i {
                    (Helpers::<{ Fraction::MEGA }, F>::LD_TIMES_RN as $i * self.raw)
                        / Helpers::<{ Fraction::MEGA }, F>::RD_TIMES_LN as $i
            }

            /// Shorthand for creating a rate which represents hertz.
            #[inline]
            #[allow(non_snake_case)]
            pub const fn Hz(val: $i) -> Self {
                Self::from_raw(
                    (Helpers::<{ Fraction::ONE }, F>::RD_TIMES_LN as $i * val)
                        / Helpers::<{ Fraction::ONE }, F>::LD_TIMES_RN as $i,
                )
            }

            /// Shorthand for creating a rate which represents kilohertz.
            #[inline]
            #[allow(non_snake_case)]
            pub const fn kHz(val: $i) -> Self {
                Self::from_raw(
                    (Helpers::<{ Fraction::KILO }, F>::RD_TIMES_LN as $i * val)
                        / Helpers::<{ Fraction::KILO }, F>::LD_TIMES_RN as $i,
                )
            }

            /// Shorthand for creating a rate which represents megahertz.
            #[inline]
            #[allow(non_snake_case)]
            pub const fn MHz(val: $i) -> Self {
                Self::from_raw(
                    (Helpers::<{ Fraction::MEGA }, F>::RD_TIMES_LN as $i * val)
                        / Helpers::<{ Fraction::MEGA }, F>::LD_TIMES_RN as $i,
                )
            }

            /// Shorthand for creating a rate which represents nanoseconds.
            #[inline]
            pub const fn nanos(val: $i) -> Self {
                Self::from_duration(crate::Duration::<$i, { Fraction::NANO }>::from_ticks(val))
            }

            /// Shorthand for creating a rate which represents microseconds.
            #[inline]
            pub const fn micros(val: $i) -> Self {
                Self::from_duration(crate::Duration::<$i, { Fraction::MICRO }>::from_ticks(val))
            }

            /// Shorthand for creating a rate which represents milliseconds.
            #[inline]
            pub const fn millis(val: $i) -> Self {
                Self::from_duration(crate::Duration::<$i, { Fraction::KILO }>::from_ticks(val))
            }
        }

        impl<const L: Fraction, const R: Fraction>
            PartialOrd<Rate<$i, R>> for Rate<$i, L>
        {
            #[inline]
            fn partial_cmp(&self, other: &Rate<$i, R>) -> Option<Ordering> {
                self.const_partial_cmp(*other)
            }
        }

        impl<const F: Fraction> Ord for Rate<$i, F> {
            #[inline]
            fn cmp(&self, other: &Self) -> Ordering {
                Self::_const_cmp(self.raw, other.raw)
            }
        }

        impl<const L: Fraction, const R: Fraction>
            PartialEq<Rate<$i, R>> for Rate<$i, L>
        {
            #[inline]
            fn eq(&self, other: &Rate<$i, R>) -> bool {
                self.const_eq(*other)
            }
        }

        impl<const F: Fraction> Eq for Rate<$i, F> {}

        // Rate - Rate = Rate (only same base until const_generics_defaults is
        // stabilized)
        impl<const F: Fraction> ops::Sub for Rate<$i, F> {
            type Output = Self;

            #[inline]
            fn sub(self, other: Self) -> Self::Output {
                if let Some(v) = self.checked_sub(other) {
                    v
                } else {
                    panic!("Sub failed!");
                }
            }
        }

        // Rate + Rate = Rate (only same base until const_generics_defaults is
        // stabilized)
        impl<const F: Fraction> ops::Add for Rate<$i, F> {
            type Output = Self;

            #[inline]
            fn add(self, other: Self) -> Self::Output {
                if let Some(v) = self.checked_add(other) {
                    v
                } else {
                    panic!("Add failed!");
                }
            }
        }

        // Rate += Rate
        impl<const F: Fraction> ops::AddAssign for Rate<$i, F> {
            #[inline]
            fn add_assign(&mut self, other: Self) {
                *self = *self + other;
            }
        }

        // integer * Rate = Rate
        impl<const F: Fraction> ops::Mul<Rate<$i, F>> for u32 {
            type Output = Rate<$i, F>;

            #[inline]
            fn mul(self, mut other: Rate<$i, F>) -> Self::Output {
                other.raw *= self as $i;
                other
            }
        }

        // Rate * integer = Rate
        impl<const F: Fraction> ops::Mul<u32> for Rate<$i, F> {
            type Output = Self;

            #[inline]
            fn mul(mut self, other: u32) -> Self::Output {
                self.raw *= other as $i;
                self
            }
        }

        // Rate *= integer
        impl<const F: Fraction> ops::MulAssign<u32> for Rate<$i, F>
        {
            #[inline]
            fn mul_assign(&mut self, other: u32) {
                *self = *self * other;
            }
        }

        // Rate / integer = Rate
        impl<const F: Fraction> ops::Div<u32> for Rate<$i, F> {
            type Output = Self;

            #[inline]
            fn div(mut self, other: u32) -> Self::Output {
                self.raw /= other as $i;
                self
            }
        }

        // Rate / Rate = integer
        impl<const L: Fraction, const R: Fraction> ops::Div<Rate<$i, R>>
            for Rate<$i, L>
        {
            type Output = $i;

            #[inline]
            fn div(self, other: Rate<$i, R>) -> Self::Output {
                let conv: Rate<$i, R> = self.convert();
                conv.raw / other.raw
            }
        }

        // Rate /= integer
        impl<const F: Fraction> ops::DivAssign<u32>
            for Rate<$i, F>
        {
            #[inline]
            fn div_assign(&mut self, other: u32) {
                *self = *self / other;
            }
        }

        #[cfg(feature = "defmt")]
        impl<const F: Fraction> defmt::Format for Rate<$i, F>
        {
            fn format(&self, f: defmt::Formatter) {
                if F.const_eq(Fraction::ONE) {
                    defmt::write!(f, "{} Hz", self.raw)
                } else if F.const_eq(Fraction::KILO) {
                    defmt::write!(f, "{} kHz", self.raw)
                } else if F.const_eq(Fraction::MEGA) {
                    defmt::write!(f, "{} MHz", self.raw)
                } else if F.const_eq(Fraction::new(1_000_000_000, 1)) {
                    defmt::write!(f, "{} GHz", self.raw)
                } else {
                    defmt::write!(f, "{} raw @ ({}/{})", self.raw, F.num, F.denom)
                }
            }
        }

        impl<const F: Fraction> core::fmt::Display for Rate<$i, F> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                if F.const_eq(Fraction::ONE) {
                    write!(f, "{} Hz", self.raw)
                } else if F.const_eq(Fraction::KILO) {
                    write!(f, "{} kHz", self.raw)
                } else if F.const_eq(Fraction::MEGA) {
                    write!(f, "{} MHz", self.raw)
                } else if F.const_eq(Fraction::new(1_000_000_000, 1)) {
                    write!(f, "{} GHz", self.raw)
                } else {
                    write!(f, "{} raw @ ({}/{})", self.raw, F.num, F.denom)
                }
            }
        }
    };
}

impl_rate_for_integer!(u32);
impl_rate_for_integer!(u64);

//
// Operations between u32 and u64 Rate
//

impl<const F: Fraction> From<Rate<u32, F>> for Rate<u64, F> {
    #[inline]
    fn from(val: Rate<u32, F>) -> Rate<u64, F> {
        Rate::<u64, F>::from_raw(val.raw() as u64)
    }
}

impl<const F: Fraction> convert::TryFrom<Rate<u64, F>> for Rate<u32, F> {
    type Error = ();

    #[inline]
    fn try_from(val: Rate<u64, F>) -> Result<Rate<u32, F>, ()> {
        Ok(Rate::<u32, F>::from_raw(
            val.raw().try_into().map_err(|_| ())?,
        ))
    }
}

// Rate - Rate = Rate (to make shorthands work, until const_generics_defaults is
// stabilized)
impl<const F: Fraction> ops::Sub<Rate<u32, F>> for Rate<u64, F> {
    type Output = Self;

    #[inline]
    fn sub(self, other: Rate<u32, F>) -> Self::Output {
        if let Some(v) = self.checked_sub(Self::from_raw(other.raw() as u64)) {
            v
        } else {
            panic!("Sub failed!");
        }
    }
}

// Rate -= Rate (to make shorthands work, until const_generics_defaults is stabilized)
impl<const F: Fraction> ops::SubAssign<Rate<u32, F>> for Rate<u64, F> {
    #[inline]
    fn sub_assign(&mut self, other: Rate<u32, F>) {
        *self = *self - other;
    }
}

// Rate + Rate = Rate (to make shorthands work, until const_generics_defaults is
// stabilized)
impl<const F: Fraction> ops::Add<Rate<u32, F>> for Rate<u64, F> {
    type Output = Self;

    #[inline]
    fn add(self, other: Rate<u32, F>) -> Self::Output {
        if let Some(v) = self.checked_add(Self::from_raw(other.raw() as u64)) {
            v
        } else {
            panic!("Add failed!");
        }
    }
}

// Rate += Rate (to make shorthands work, until const_generics_defaults is stabilized)
impl<const F: Fraction> ops::AddAssign<Rate<u32, F>> for Rate<u64, F> {
    #[inline]
    fn add_assign(&mut self, other: Rate<u32, F>) {
        *self = *self + other;
    }
}

impl<const L: Fraction, const R: Fraction> PartialOrd<Rate<u32, R>> for Rate<u64, L> {
    #[inline]
    fn partial_cmp(&self, other: &Rate<u32, R>) -> Option<Ordering> {
        self.partial_cmp(&Rate::<u64, R>::from_raw(other.raw() as u64))
    }
}

impl<const L: Fraction, const R: Fraction> PartialEq<Rate<u32, R>> for Rate<u64, L> {
    #[inline]
    fn eq(&self, other: &Rate<u32, R>) -> bool {
        self.eq(&Rate::<u64, R>::from_raw(other.raw() as u64))
    }
}

impl<const L: Fraction, const R: Fraction> PartialOrd<Rate<u64, R>> for Rate<u32, L> {
    #[inline]
    fn partial_cmp(&self, other: &Rate<u64, R>) -> Option<Ordering> {
        Rate::<u64, L>::from_raw(self.raw as u64).partial_cmp(other)
    }
}

impl<const L: Fraction, const R: Fraction> PartialEq<Rate<u64, R>> for Rate<u32, L> {
    #[inline]
    fn eq(&self, other: &Rate<u64, R>) -> bool {
        Rate::<u64, L>::from_raw(self.raw as u64).eq(other)
    }
}

/// Extension trait for simple short-hands for u32 Rate
pub trait ExtU32 {
    /// Shorthand for creating a rate which represents hertz.
    #[allow(non_snake_case)]
    fn Hz<const F: Fraction>(self) -> Rate<u32, F>;

    /// Shorthand for creating a rate which represents kilohertz.
    #[allow(non_snake_case)]
    fn kHz<const F: Fraction>(self) -> Rate<u32, F>;

    /// Shorthand for creating a rate which represents megahertz.
    #[allow(non_snake_case)]
    fn MHz<const F: Fraction>(self) -> Rate<u32, F>;
}

impl ExtU32 for u32 {
    #[inline]
    #[allow(non_snake_case)]
    fn Hz<const F: Fraction>(self) -> Rate<u32, F> {
        Rate::<u32, F>::Hz(self)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn kHz<const F: Fraction>(self) -> Rate<u32, F> {
        Rate::<u32, F>::kHz(self)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn MHz<const F: Fraction>(self) -> Rate<u32, F> {
        Rate::<u32, F>::MHz(self)
    }
}

/// Extension trait for simple short-hands for u64 Rate
pub trait ExtU64 {
    /// Shorthand for creating a rate which represents hertz.
    #[allow(non_snake_case)]
    fn Hz<const F: Fraction>(self) -> Rate<u64, F>;

    /// Shorthand for creating a rate which represents kilohertz.
    #[allow(non_snake_case)]
    fn kHz<const F: Fraction>(self) -> Rate<u64, F>;

    /// Shorthand for creating a rate which represents megahertz.
    #[allow(non_snake_case)]
    fn MHz<const F: Fraction>(self) -> Rate<u64, F>;
}

impl ExtU64 for u64 {
    #[inline]
    #[allow(non_snake_case)]
    fn Hz<const F: Fraction>(self) -> Rate<u64, F> {
        Rate::<u64, F>::Hz(self)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn kHz<const F: Fraction>(self) -> Rate<u64, F> {
        Rate::<u64, F>::kHz(self)
    }

    #[inline]
    #[allow(non_snake_case)]
    fn MHz<const F: Fraction>(self) -> Rate<u64, F> {
        Rate::<u64, F>::MHz(self)
    }
}
