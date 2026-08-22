//! `fugit` provides a comprehensive library of [`Duration`] and [`Instant`] for the handling of
//! time in embedded systems. The library is specifically designed to maximize const-ification
//! which allows for most comparisons and changes of time-base to be made at compile time, rather
//! than run time.
//!
//! The library is aimed at ease-of-use and performance first.
//!
//! ```
//! use fugit::{Duration, ExtU32};
//!
//! // Efficient short-hands (`.millis()`, ...)
//! let d = Duration::<u32, 1, 1_000>::from_ticks(111);
//!
//! let sum1 = d + 300.millis();
//! //             ^^^ Compile time move of base, only a sum is needed and no change of base
//!
//!
//! // -----------------------
//!
//! // Best effort for fixed types
//! fn bar(d1: Duration<u32, 1, 1_000>, d2: Duration<u32, 1, 1_000_000>) {
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
//! fn baz(d1: Duration<u64, 1, 1_000>, d2: Duration<u64, 1, 1_000_000>) {
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
//!
//! # Two kinds of instant
//!
//! [`Instant`] takes a [`kind`] as its last type parameter, because the two
//! readings of a timestamp need different operations to be correct.
//!
//! Pick [`Wrapping`](kind::Wrapping) for a raw hardware counter. Ticks wrap, and comparison is
//! wrap-aware, which is only meaningful within half the tick range and is not transitive, so
//! there is no [`PartialOrd`] or [`Ord`]; compare through [`InstantOrd`] or the inherent
//! `is_before`/`is_after`.
//!
//! Pick [`Monotonic`](kind::Monotonic) for a timeline that does not wrap, such as a counter
//! extended to 64 bits in software. Comparison is a plain integer compare, so it is [`Ord`].
//!
//! Nothing checks the claim that a monotonic timeline does not wrap: [`Instant::from_ticks`]
//! takes any value, so a counter that wrapped before the value arrived is invisible here and
//! [`Ord`] will simply place the instant in the past. The producer owns that guarantee.
//!
//! Use the aliases rather than naming a marker: [`WrappingTimerInstantU32`],
//! [`MonotonicTimerInstantU64`] and friends.

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

mod aliases;
mod duration;
mod helpers;
mod instant;
pub mod kind;
mod rate;

pub use aliases::*;
pub use duration::{Duration, ExtU32, ExtU32Ceil, ExtU64, ExtU64Ceil};
pub use instant::{Instant, InstantOrd};
pub use rate::{ExtU32 as RateExtU32, ExtU64 as RateExtU64, Rate};

#[cfg(test)]
mod test_duration;

#[cfg(test)]
mod test_instant;

#[cfg(test)]
mod test_rate;
