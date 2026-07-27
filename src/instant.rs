use crate::duration::Duration;
use crate::helpers::{assert_conversion_fits, Helpers};
use crate::kind::{Kind, Monotonic, Wrapping};
use core::cmp::Ordering;
use core::marker::PhantomData;
use core::ops;

/// Represents an instant in time.
///
/// The generic `T` can either be `u32` or `u64`, and the const generics represent the ratio of the
/// ticks contained within the instant: `instant in seconds = NOM / DENOM * ticks`
///
/// `K` says how the timeline is read. See [`kind`](crate::kind), or use an alias such as
/// [`WrappingInstant`](crate::WrappingInstant).
///
/// Adding or subtracting a [`Duration`] in a different time base is subject to the same base
/// ratio limits as [`Duration`] itself: bases differing by more than `T::MAX` are a compile-time
/// error rather than a silent truncation.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Instant<T, const NOM: u64, const DENOM: u64, K> {
    ticks: T,
    #[cfg_attr(feature = "serde", serde(skip))]
    _kind: PhantomData<K>,
}

#[cfg(feature = "postcard_max_size")]
impl<T: postcard::experimental::max_size::MaxSize, const NOM: u64, const DENOM: u64, K>
    postcard::experimental::max_size::MaxSize for Instant<T, NOM, DENOM, K>
{
    const POSTCARD_MAX_SIZE: usize = T::POSTCARD_MAX_SIZE;
}

impl<T: core::fmt::Debug, const NOM: u64, const DENOM: u64, K: Kind> core::fmt::Debug
    for Instant<T, NOM, DENOM, K>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(K::DEBUG_NAME)
            .field("ticks", &self.ticks)
            .finish()
    }
}

/// Construction, equality and formatting: everything whose behaviour does not depend on how
/// the instant's timeline is read.
macro_rules! impl_instant_shared {
    ($i:ty) => {
        impl<const NOM: u64, const DENOM: u64, K: Kind> Instant<$i, NOM, DENOM, K> {
            /// Create an `Instant` from a ticks value.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let _i = WrappingInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            /// ```
            #[inline]
            pub const fn from_ticks(ticks: $i) -> Self {
                const { assert!(NOM > 0) };
                const { assert!(DENOM > 0) };

                Instant {
                    ticks,
                    _kind: PhantomData,
                }
            }

            /// Extract the ticks from an `Instant`.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i = WrappingInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(234);")]
            ///
            /// assert_eq!(i.as_ticks(), 234);
            /// ```
            #[inline]
            pub const fn as_ticks(&self) -> $i {
                self.ticks
            }
        }

        #[cfg(feature = "defmt")]
        impl<const NOM: u64, const DENOM: u64, K: Kind> defmt::Format for Instant<$i, NOM, DENOM, K> {
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
                } else if NOM == 1 && DENOM == 1_000_000_000_000 {
                    defmt::write!(f, "{} ps", self.ticks)
                } else {
                    defmt::write!(f, "{} ticks @ ({}/{})", self.ticks, NOM, DENOM)
                }
            }
        }

        impl<const NOM: u64, const DENOM: u64, K: Kind> core::fmt::Display
            for Instant<$i, NOM, DENOM, K>
        {
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
                } else if NOM == 1 && DENOM == 1_000_000_000_000 {
                    write!(f, "{} ps", self.ticks)
                } else {
                    write!(f, "{} ticks @ ({}/{})", self.ticks, NOM, DENOM)
                }
            }
        }
    };
}

/// Comparison and arithmetic for instants read as a circular counter: ticks wrap, and the
/// ordering is wrap-aware and therefore only partial.
macro_rules! impl_instant_wrapping {
    ($i:ty) => {
        impl<const NOM: u64, const DENOM: u64> Instant<$i, NOM, DENOM, Wrapping> {
            /// Const partial comparison of `Instant`s.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i1 = WrappingInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            #[doc = concat!("let i2 = WrappingInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(2);")]
            ///
            /// assert_eq!(i1.const_partial_cmp(i2), Some(core::cmp::Ordering::Less));
            /// ```
            ///
            /// This function takes into account that ticks might wrap around: `self - other` is
            /// read as a signed offset, so an `Instant` just past the wrap point comes after one
            /// just before it:
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i1 = WrappingInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(", stringify!($i),"::MAX);")]
            #[doc = concat!("let i2 = WrappingInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            ///
            /// assert_eq!(i1.const_partial_cmp(i2), Some(core::cmp::Ordering::Less));
            /// ```
            ///
            /// Returns `None` when the two are exactly half the tick range apart. There
            /// `self - other` and `other - self` are the same value, so neither can be said to
            /// come first:
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i1 = WrappingInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(0);")]
            #[doc = concat!("let i2 = WrappingInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(1 << (", stringify!($i), "::BITS - 1));")]
            ///
            /// assert_eq!(i1.const_partial_cmp(i2), None);
            /// ```
            #[inline]
            pub const fn const_partial_cmp(self, other: Self) -> Option<Ordering> {
                const HALF: $i = <$i>::MAX / 2 + 1;

                let v = self.ticks.wrapping_sub(other.ticks);

                // not using `cmp` due to it being non-const
                if v == 0 {
                    Some(Ordering::Equal)
                } else if v < HALF {
                    Some(Ordering::Greater)
                } else if v > HALF {
                    Some(Ordering::Less)
                } else {
                    None
                }
            }

            /// Whether this `Instant` comes before `other`.
            ///
            /// Wrap-aware, like [`const_partial_cmp`](Self::const_partial_cmp), and usable in
            /// const contexts where `<` is not. Instants exactly half the tick range apart are
            /// incomparable, so this and [`is_after`](Self::is_after) are both `false` for them.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("const I1: WrappingInstant<", stringify!($i), ", 1, 1_000>")]
            #[doc = concat!("    = WrappingInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(", stringify!($i), "::MAX);")]
            #[doc = concat!("const I2: WrappingInstant<", stringify!($i), ", 1, 1_000>")]
            #[doc = concat!("    = WrappingInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            /// const BEFORE: bool = I1.is_before(I2);
            ///
            /// assert!(BEFORE);
            /// ```
            #[inline]
            pub const fn is_before(self, other: Self) -> bool {
                matches!(self.const_partial_cmp(other), Some(Ordering::Less))
            }

            /// Whether this `Instant` comes after `other`.
            ///
            /// Wrap-aware, like [`const_partial_cmp`](Self::const_partial_cmp), and usable in
            /// const contexts where `>` is not. Instants exactly half the tick range apart are
            /// incomparable, so this and [`is_before`](Self::is_before) are both `false` for them.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("const I1: WrappingInstant<", stringify!($i), ", 1, 1_000>")]
            #[doc = concat!("    = WrappingInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            #[doc = concat!("const I2: WrappingInstant<", stringify!($i), ", 1, 1_000>")]
            #[doc = concat!("    = WrappingInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(", stringify!($i), "::MAX);")]
            /// const AFTER: bool = I1.is_after(I2);
            ///
            /// assert!(AFTER);
            /// ```
            #[inline]
            pub const fn is_after(self, other: Self) -> bool {
                matches!(self.const_partial_cmp(other), Some(Ordering::Greater))
            }

            /// Duration between `Instant`s.
            ///
            /// Returns `None` if `self` is before `other`, or if the two cannot be
            /// ordered at all, under the wrap-aware ordering used by
            /// [`const_partial_cmp`](Self::const_partial_cmp).
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i1 = WrappingInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            #[doc = concat!("let i2 = WrappingInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(2);")]
            ///
            /// assert_eq!(i1.checked_duration_since(i2), None);
            /// assert_eq!(i2.checked_duration_since(i1).unwrap().as_ticks(), 1);
            /// ```
            #[inline]
            pub const fn checked_duration_since(
                self,
                other: Self,
            ) -> Option<Duration<$i, NOM, DENOM>> {
                match self.const_partial_cmp(other) {
                    Some(Ordering::Greater) | Some(Ordering::Equal) => {
                        Some(Duration::<$i, NOM, DENOM>::from_ticks(
                            self.ticks.wrapping_sub(other.ticks),
                        ))
                    }
                    Some(Ordering::Less) | None => None,
                }
            }

            /// Try to convert `other` into the `Self::NOM / Self::DENOM` timebase, and
            /// subtract it from this [`Instant`].
            ///
            /// Returns `None` only if the time-base conversion fails. The subtraction itself
            /// is wrapping, as [`Instant`]s are circular.
            ///
            /// The implementations of [`core::ops::Sub`] and [`core::ops::SubAssign`] are
            /// implemented by `unwrap`-ing the value returned by this function.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i = WrappingInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            #[doc = concat!("let d = Duration::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            ///
            /// assert_eq!(i.convert_sub_duration(d).unwrap().as_ticks(), 0);
            /// ```
            ///
            /// The subtraction itself is wrapping:
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i = WrappingInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            #[doc = concat!("let d = Duration::<", stringify!($i), ", 1, 1_000>::from_ticks(2);")]
            ///
            #[doc = concat!("assert_eq!(i.convert_sub_duration(d).unwrap().as_ticks(), ", stringify!($i), "::MAX);")]
            /// ```
            ///
            /// Overflow during [`Duration`] base conversion returns `None`:
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i = WrappingInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            /// // A duration whose conversion from the `1/500` base to the `1/1000` base
            /// // will overflow.
            #[doc = concat!("let d = Duration::<", stringify!($i), ", 1, 500>::from_ticks(", stringify!($i),"::MAX / 2 + 1);")]
            ///
            /// assert_eq!(i.convert_sub_duration(d), None);
            /// ```
            pub const fn convert_sub_duration<const O_NOM: u64, const O_DENOM: u64>(
                self,
                other: Duration<$i, O_NOM, O_DENOM>,
            ) -> Option<Self> {
                assert_conversion_fits!(
                    $i,
                    Helpers::<NOM, DENOM, O_NOM, O_DENOM>,
                    "Instant::convert_sub_duration"
                );

                if Helpers::<NOM, DENOM, O_NOM, O_DENOM>::SAME_BASE {
                    Some(Self::from_ticks(
                        self.ticks.wrapping_sub(other.as_ticks()),
                    ))
                } else {
                    if let Some(lh) = other
                        .as_ticks()
                        .checked_mul(Helpers::<NOM, DENOM, O_NOM, O_DENOM>::LD_TIMES_RN as $i)
                    {
                        let ticks = lh / Helpers::<NOM, DENOM, O_NOM, O_DENOM>::RD_TIMES_LN as $i;

                        Some(Self::from_ticks(self.ticks.wrapping_sub(ticks)))
                    } else {
                        None
                    }
                }
            }

            /// Try to convert `other` into the `Self::NOM / Self::DENOM` timebase, and
            /// add it to this [`Instant`].
            ///
            /// Returns `None` only if the time-base conversion fails. The addition itself
            /// is wrapping, as [`Instant`]s are circular.
            ///
            /// The implementations of [`core::ops::Add`] and [`core::ops::AddAssign`] are
            /// implemented by `unwrap`-ing the value returned by this function.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i = WrappingInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            #[doc = concat!("let d = Duration::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            ///
            /// assert_eq!(i.convert_add_duration(d).unwrap().as_ticks(), 2);
            /// ```
            ///
            /// The addition itself is wrapping:
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i = WrappingInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            #[doc = concat!("let d = Duration::<", stringify!($i), ", 1, 1_000>::from_ticks(", stringify!($i), "::MAX);")]
            ///
            /// assert_eq!(i.convert_add_duration(d).unwrap().as_ticks(), 0);
            /// ```
            ///
            /// Overflow during [`Duration`] base conversion returns `None`:
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i = WrappingInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            /// // A duration whose conversion from the `1/500` base to the `1/1000` base
            /// // will overflow.
            #[doc = concat!("let d = Duration::<", stringify!($i), ", 1, 500>::from_ticks(", stringify!($i),"::MAX / 2 + 1);")]
            ///
            /// assert_eq!(i.convert_add_duration(d), None);
            /// ```
            pub const fn convert_add_duration<const O_NOM: u64, const O_DENOM: u64>(
                self,
                other: Duration<$i, O_NOM, O_DENOM>,
            ) -> Option<Self> {
                assert_conversion_fits!(
                    $i,
                    Helpers::<NOM, DENOM, O_NOM, O_DENOM>,
                    "Instant::convert_add_duration"
                );

                if Helpers::<NOM, DENOM, O_NOM, O_DENOM>::SAME_BASE {
                    Some(Self::from_ticks(
                        self.ticks.wrapping_add(other.as_ticks()),
                    ))
                } else {
                    if let Some(lh) = other
                        .as_ticks()
                        .checked_mul(Helpers::<NOM, DENOM, O_NOM, O_DENOM>::LD_TIMES_RN as $i)
                    {
                        let ticks = lh / Helpers::<NOM, DENOM, O_NOM, O_DENOM>::RD_TIMES_LN as $i;

                        Some(Self::from_ticks(self.ticks.wrapping_add(ticks)))
                    } else {
                        None
                    }
                }
            }
        }

        impl<const NOM: u64, const DENOM: u64> PartialOrd for Instant<$i, NOM, DENOM, Wrapping> {
            /// This implementation deviates from the definition of
            /// [PartialOrd::partial_cmp](core::cmp::PartialOrd::partial_cmp):
            ///
            /// It takes into account that ticks might wrap around. If the absolute
            /// values of `self` and `other` differ by more than half the possible range, it is
            /// assumed that an overflow occured and the result is reversed.
            ///
            /// That breaks the transitivity invariant: a < b and b < c no longer implies a < c,
            /// which is formally more than [`PartialOrd`] permits. The impl is kept as a
            /// conscious exception so `<` and `>` remain usable, see [`Wrapping`](crate::kind::Wrapping).
            /// [`Ord`](core::cmp::Ord) is for that reason deliberately not implemented, as its
            /// users - `BTreeMap`, `sort`, `max` - need a transitive total order and silently
            /// misbehave without one.
            ///
            /// Instants exactly half the tick range apart are incomparable, see
            /// [`const_partial_cmp`](Instant::const_partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.const_partial_cmp(*other)
            }
        }

        // Instant - Duration = Instant
        // We have limited this to use same numerator and denominator in both left and right hand sides,
        // this allows for the extension traits to work. For usage with different fraction, use
        // [`Self::convert_sub_duration`].
        impl<const NOM: u64, const DENOM: u64> ops::Sub<Duration<$i, NOM, DENOM>>
            for Instant<$i, NOM, DENOM, Wrapping>
        {
            type Output = Self;

            #[inline]
            #[track_caller]
            fn sub(self, other: Duration<$i, NOM, DENOM>) -> Self::Output {
                if let Some(v) = self.convert_sub_duration(other) {
                    v
                } else {
                    panic!("Sub failed! Overflow");
                }
            }
        }

        // Instant + Duration = Instant
        // We have limited this to use same numerator and denominator in both left and right hand sides,
        // this allows for the extension traits to work. For usage with different fraction, use
        // [`Self::convert_add_duration`].
        impl<const NOM: u64, const DENOM: u64> ops::Add<Duration<$i, NOM, DENOM>>
            for Instant<$i, NOM, DENOM, Wrapping>
        {
            type Output = Self;

            #[inline]
            #[track_caller]
            fn add(self, other: Duration<$i, NOM, DENOM>) -> Self::Output {
                if let Some(v) = self.convert_add_duration(other) {
                    v
                } else {
                    panic!("Add failed! Overflow");
                }
            }
        }
    };
}

/// Comparison and arithmetic for instants on a timeline that does not wrap: tick math is plain
/// integer math, and the ordering is total.
macro_rules! impl_instant_monotonic {
    ($i:ty) => {
        impl<const NOM: u64, const DENOM: u64> Instant<$i, NOM, DENOM, Monotonic> {
            /// Const comparison of `Instant`s.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i1 = MonotonicInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            #[doc = concat!("let i2 = MonotonicInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(2);")]
            ///
            /// assert_eq!(i1.const_cmp(i2), core::cmp::Ordering::Less);
            /// ```
            #[inline]
            pub const fn const_cmp(self, other: Self) -> Ordering {
                // not using `cmp` due to it being non-const
                if self.ticks < other.ticks {
                    Ordering::Less
                } else if self.ticks > other.ticks {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }

            /// Whether this `Instant` comes before `other`.
            ///
            /// Usable in const contexts, where `<` is not.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("const I1: MonotonicInstant<", stringify!($i), ", 1, 1_000>")]
            #[doc = concat!("    = MonotonicInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            #[doc = concat!("const I2: MonotonicInstant<", stringify!($i), ", 1, 1_000>")]
            #[doc = concat!("    = MonotonicInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(2);")]
            /// const BEFORE: bool = I1.is_before(I2);
            ///
            /// assert!(BEFORE);
            /// ```
            #[inline]
            pub const fn is_before(self, other: Self) -> bool {
                self.ticks < other.ticks
            }

            /// Whether this `Instant` comes after `other`.
            ///
            /// Usable in const contexts, where `>` is not.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("const I1: MonotonicInstant<", stringify!($i), ", 1, 1_000>")]
            #[doc = concat!("    = MonotonicInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(2);")]
            #[doc = concat!("const I2: MonotonicInstant<", stringify!($i), ", 1, 1_000>")]
            #[doc = concat!("    = MonotonicInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            /// const AFTER: bool = I1.is_after(I2);
            ///
            /// assert!(AFTER);
            /// ```
            #[inline]
            pub const fn is_after(self, other: Self) -> bool {
                self.ticks > other.ticks
            }

            /// The duration between this instant and `Instant::from_ticks(0)`.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i = MonotonicInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(11);")]
            ///
            /// assert_eq!(i.duration_since_epoch().as_ticks(), 11);
            /// ```
            #[inline]
            pub const fn duration_since_epoch(self) -> Duration<$i, NOM, DENOM> {
                Duration::<$i, NOM, DENOM>::from_ticks(self.as_ticks())
            }

            /// Duration between `Instant`s.
            ///
            /// Returns `None` if `self` is before `other`.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i1 = MonotonicInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            #[doc = concat!("let i2 = MonotonicInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(2);")]
            ///
            /// assert_eq!(i1.checked_duration_since(i2), None);
            /// assert_eq!(i2.checked_duration_since(i1).unwrap().as_ticks(), 1);
            /// ```
            #[inline]
            pub const fn checked_duration_since(
                self,
                other: Self,
            ) -> Option<Duration<$i, NOM, DENOM>> {
                match self.ticks.checked_sub(other.ticks) {
                    Some(ticks) => Some(Duration::<$i, NOM, DENOM>::from_ticks(ticks)),
                    None => None,
                }
            }

            /// Try to convert `other` into the `Self::NOM / Self::DENOM` timebase, and
            /// subtract it from this [`Instant`].
            ///
            /// Returns `None` if the time-base conversion fails, or if the subtraction would go
            /// below zero. Note that those two causes are not distinguished.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i = MonotonicInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            #[doc = concat!("let d = Duration::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            ///
            /// assert_eq!(i.checked_sub_duration(d).unwrap().as_ticks(), 0);
            /// ```
            ///
            /// Going below zero returns `None` rather than wrapping:
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i = MonotonicInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            #[doc = concat!("let d = Duration::<", stringify!($i), ", 1, 1_000>::from_ticks(2);")]
            ///
            /// assert_eq!(i.checked_sub_duration(d), None);
            /// ```
            pub const fn checked_sub_duration<const O_NOM: u64, const O_DENOM: u64>(
                self,
                other: Duration<$i, O_NOM, O_DENOM>,
            ) -> Option<Self> {
                assert_conversion_fits!(
                    $i,
                    Helpers::<NOM, DENOM, O_NOM, O_DENOM>,
                    "Instant::checked_sub_duration"
                );

                match Duration::<$i, NOM, DENOM>::from_ticks(self.ticks).checked_sub(other) {
                    Some(d) => Some(Self::from_ticks(d.as_ticks())),
                    None => None,
                }
            }

            /// Try to convert `other` into the `Self::NOM / Self::DENOM` timebase, and
            /// add it to this [`Instant`].
            ///
            /// Returns `None` if the time-base conversion fails, or if the addition would
            /// overflow the storage type. Note that those two causes are not distinguished.
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i = MonotonicInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            #[doc = concat!("let d = Duration::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            ///
            /// assert_eq!(i.checked_add_duration(d).unwrap().as_ticks(), 2);
            /// ```
            ///
            /// Overflowing the storage type returns `None` rather than wrapping:
            ///
            /// ```
            /// # use fugit::*;
            #[doc = concat!("let i = MonotonicInstant::<", stringify!($i), ", 1, 1_000>::from_ticks(1);")]
            #[doc = concat!("let d = Duration::<", stringify!($i), ", 1, 1_000>::from_ticks(", stringify!($i), "::MAX);")]
            ///
            /// assert_eq!(i.checked_add_duration(d), None);
            /// ```
            pub const fn checked_add_duration<const O_NOM: u64, const O_DENOM: u64>(
                self,
                other: Duration<$i, O_NOM, O_DENOM>,
            ) -> Option<Self> {
                assert_conversion_fits!(
                    $i,
                    Helpers::<NOM, DENOM, O_NOM, O_DENOM>,
                    "Instant::checked_add_duration"
                );

                match Duration::<$i, NOM, DENOM>::from_ticks(self.ticks).checked_add(other) {
                    Some(d) => Some(Self::from_ticks(d.as_ticks())),
                    None => None,
                }
            }
        }

        impl<const NOM: u64, const DENOM: u64> Ord for Instant<$i, NOM, DENOM, Monotonic> {
            #[inline]
            fn cmp(&self, other: &Self) -> Ordering {
                self.ticks.cmp(&other.ticks)
            }
        }

        impl<const NOM: u64, const DENOM: u64> PartialOrd for Instant<$i, NOM, DENOM, Monotonic> {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        // Instant - Duration = Instant
        // We have limited this to use same numerator and denominator in both left and right hand sides,
        // this allows for the extension traits to work. For usage with different fraction, use
        // [`Self::checked_sub_duration`].
        //
        // Plain `-`, so this inherits `overflow-checks` from the final binary: a panic in debug and
        // a wrap in release, exactly like the underlying integer.
        impl<const NOM: u64, const DENOM: u64> ops::Sub<Duration<$i, NOM, DENOM>>
            for Instant<$i, NOM, DENOM, Monotonic>
        {
            type Output = Self;

            #[inline]
            #[track_caller]
            fn sub(self, other: Duration<$i, NOM, DENOM>) -> Self::Output {
                Self::from_ticks(self.ticks - other.as_ticks())
            }
        }

        // Instant + Duration = Instant
        // We have limited this to use same numerator and denominator in both left and right hand sides,
        // this allows for the extension traits to work. For usage with different fraction, use
        // [`Self::checked_add_duration`].
        //
        // Plain `+`, see the `Sub` impl above.
        impl<const NOM: u64, const DENOM: u64> ops::Add<Duration<$i, NOM, DENOM>>
            for Instant<$i, NOM, DENOM, Monotonic>
        {
            type Output = Self;

            #[inline]
            #[track_caller]
            fn add(self, other: Duration<$i, NOM, DENOM>) -> Self::Output {
                Self::from_ticks(self.ticks + other.as_ticks())
            }
        }
    };
}

/// `Instant - Instant = Duration`, the one operator body both kinds share.
macro_rules! impl_instant_ops {
    ($i:ty, $k:ty) => {
        // Instant - Instant = Duration
        // We have limited this to use same numerator and denominator in both left and right hand sides,
        // this allows for the extension traits to work. For usage with different fraction, use
        // `checked_duration_since`.
        impl<const NOM: u64, const DENOM: u64> ops::Sub<Instant<$i, NOM, DENOM, $k>>
            for Instant<$i, NOM, DENOM, $k>
        {
            type Output = Duration<$i, NOM, DENOM>;

            #[inline]
            #[track_caller]
            fn sub(self, other: Self) -> Self::Output {
                if let Some(v) = self.checked_duration_since(other) {
                    v
                } else {
                    panic!("Sub failed! Other is not before self");
                }
            }
        }
    };
}

impl_instant_shared!(u32);
impl_instant_shared!(u64);

impl_instant_wrapping!(u32);
impl_instant_wrapping!(u64);

impl_instant_monotonic!(u32);
impl_instant_monotonic!(u64);

impl_instant_ops!(u32, Wrapping);
impl_instant_ops!(u64, Wrapping);
impl_instant_ops!(u32, Monotonic);
impl_instant_ops!(u64, Monotonic);

// Instant -= Duration and Instant += Duration, for every duration the instant can `Sub`/`Add`,
// resolving through the kind's own operator.
impl<T, D, const NOM: u64, const DENOM: u64, K> ops::SubAssign<D> for Instant<T, NOM, DENOM, K>
where
    Self: Copy + ops::Sub<D, Output = Self>,
{
    #[inline]
    #[track_caller]
    fn sub_assign(&mut self, other: D) {
        *self = *self - other;
    }
}

impl<T, D, const NOM: u64, const DENOM: u64, K> ops::AddAssign<D> for Instant<T, NOM, DENOM, K>
where
    Self: Copy + ops::Add<D, Output = Self>,
{
    #[inline]
    #[track_caller]
    fn add_assign(&mut self, other: D) {
        *self = *self + other;
    }
}

//
// Operations between a `u64` `Instant` and a `u32` `Duration`.
//
// These need no macro. They widen the `Duration` and defer to the same-base operator rather than
// naming a conversion method, and the widening is exact, so the bodies hold for any timeline.
//

// Instant - Duration = Instant
// We have limited this to use same numerator and denominator in both left and right hand sides,
// this allows for the extension traits to work. For usage with different fraction, use the
// kind's `convert_sub_duration`/`checked_sub_duration`.
impl<const NOM: u64, const DENOM: u64, K: Kind> ops::Sub<Duration<u32, NOM, DENOM>>
    for Instant<u64, NOM, DENOM, K>
where
    Instant<u64, NOM, DENOM, K>: ops::Sub<Duration<u64, NOM, DENOM>, Output = Self>,
{
    type Output = Self;

    #[inline]
    #[track_caller]
    fn sub(self, other: Duration<u32, NOM, DENOM>) -> Self::Output {
        self - Duration::<u64, NOM, DENOM>::from(other)
    }
}

// Instant + Duration = Instant
// We have limited this to use same numerator and denominator in both left and right hand sides,
// this allows for the extension traits to work. For usage with different fraction, use the
// kind's `convert_add_duration`/`checked_add_duration`.
impl<const NOM: u64, const DENOM: u64, K: Kind> ops::Add<Duration<u32, NOM, DENOM>>
    for Instant<u64, NOM, DENOM, K>
where
    Instant<u64, NOM, DENOM, K>: ops::Add<Duration<u64, NOM, DENOM>, Output = Self>,
{
    type Output = Self;

    #[inline]
    #[track_caller]
    fn add(self, other: Duration<u32, NOM, DENOM>) -> Self::Output {
        self + Duration::<u64, NOM, DENOM>::from(other)
    }
}

// impl<const L_NOM: u64, const L_DENOM: u64, const R_NOM: u64, const R_DENOM: u64>
//     ops::Add<Duration<u32, R_NOM, R_DENOM>> for Duration<u64, L_NOM, L_DENOM>
// {
//     type Output = Duration<u64, L_NOM, L_DENOM>;
//
//     #[inline]
//     fn add(self, other: Duration<u32, R_NOM, R_DENOM>) -> Self::Output {
//         self.add(Duration::<u64, L_NOM, L_DENOM>::from_ticks(
//             other.as_ticks() as u64
//         ))
//     }
// }
