//! `fugit` provides a comprehensive library of [`Duration`] and [`Instant`] for the handling of
//! time in embedded systems. The library is specifically designed to maximize const-ification
//! which allows for most comparisons and changes of time-base to be made at compile time, rather
//! than run time.
//!
//! The library is aimed at ease-of-use and performance first.
//!
//! ```
//! use fugit::{Duration, ExtU32, Fraction};
//!
//! // Efficient short-hands (`.millis()`, ...)
//! let d = Duration::<u32, { Fraction::new(1, 1_000) }>::from_ticks(111);
//!
//! let sum1 = d + 300.millis();
//! //             ^^^ Compile time move of base, only a sum is needed and no change of base
//!
//!
//! // -----------------------
//!
//! // Best effort for fixed types
//! fn bar(d1: Duration<u32, { Fraction::new(1, 1_000) }>, d2: Duration<u32, { Fraction::new(1, 1_000_000) }>) {
//!     let sum = d1 + d2.convert();
//!     //        ^^^^^^^ Run time move of base, will use a `mul` and `div` instruction (Cortex-M3+) to
//!     //                perform the move of base.
//!     //                The `.convert()` explicitly signals the move of base.
//!
//!     let ops = d1 > d2;
//!     //        ^^^^^^^ Run time comparison of different base, will use 2 `mul` instructions
//!     //                (Cortex-M3+) to perform the comparison.
//! }
//!
//! fn baz(d1: Duration<u64, { Fraction::new(1, 1_000) }>, d2: Duration<u64, { Fraction::new(1, 1_000_000) }>) {
//!     let sum = d1 + d2.convert();
//!     //        ^^^^^^^ Run time move of base, will use a `mul` insruction and `div`
//!     //                soft-impl (Cortex-M3+) to perform the move of base.
//!     //                The `.convert()` explicitly signals the move of base.
//!
//!     let ops = d1 > d2;
//!     //        ^^^^^^^ Run time comparison of different base, will use 4 `mul` instructions
//!     //                (Cortex-M3+) to perform the comparison.
//! }
//! ```

#![feature(adt_const_params)]
#![feature(generic_const_exprs)] // Needed only for several type aliases
#![allow(incomplete_features)]
#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

mod aliases;
mod duration;
mod helpers;
mod instant;
mod rate;

pub use aliases::*;
pub use duration::{Duration, ExtU32, ExtU32Ceil, ExtU64, ExtU64Ceil};
pub use instant::Instant;
pub use rate::{ExtU32 as RateExtU32, ExtU64 as RateExtU64, Rate};
/// A fractional value
///
/// Used primarily to define the _scaling factor_ for the [`Duration`], [`Rate`] and [`Instant`] types.
#[derive(Clone, Copy, PartialEq, Eq, Debug, core::marker::ConstParamTy)]
pub struct Fraction {
    /// Numerator
    pub num: u64,
    /// Denomnator
    pub denom: u64,
}

impl Fraction {
    const NANO: Self = Self::new(1, 1_000_000_000);
    const MICRO: Self = Self::new(1, 1_000_000);
    const MILLI: Self = Self::new(1, 1_000);
    const ONE: Self = Self::new(1, 1);
    const KILO: Self = Self::new(1_000, 1);
    const MEGA: Self = Self::new(1_000_000, 1);

    /// Construct a new `Fraction`.
    pub const fn new(numerator: u64, denominator: u64) -> Self {
        Self {
            num: numerator,
            denom: denominator,
        }
    }

    const fn const_eq(self, other: Self) -> bool {
        self.num == other.num && self.denom == other.denom
    }

    const fn factor_f32(self) -> f32 {
        self.num as f32 / self.denom as f32
    }

    const fn factor_f64(self) -> f64 {
        self.num as f64 / self.denom as f64
    }

    const fn inv_factor_f32(self) -> f32 {
        self.denom as f32 / self.num as f32
    }

    const fn inv_factor_f64(self) -> f64 {
        self.denom as f64 / self.num as f64
    }
}

#[cfg(test)]
mod test_duration;

#[cfg(test)]
mod test_instant;

#[cfg(test)]
mod test_rate;
