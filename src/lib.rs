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

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

mod aliases;
mod duration;
mod helpers;
mod instant;

pub use aliases::*;
pub use duration::{Duration, ExtU32, ExtU64};
pub use instant::Instant;

#[cfg(test)]
mod test {
    use crate::Duration;
    use crate::Instant;

    fn take_ms(d: Duration<u32, 1, 1_000>) -> Duration<u32, 1, 1_000> {
        d
    }

    #[test]
    fn duration_functions() {
        assert_eq!(
            take_ms(Duration::<u32, 1, 100>::from_ticks(1).convert()),
            Duration::<u32, 1, 1_000>::from_ticks(10)
        );
    }

    #[test]
    fn duration_compare_u32() {
        // Same fraction
        assert!(
            Duration::<u32, 1, 1_000>::from_ticks(2) > Duration::<u32, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u32, 1, 1_000>::from_ticks(2) >= Duration::<u32, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u32, 1, 1_000>::from_ticks(1) >= Duration::<u32, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u32, 1, 1_000>::from_ticks(1) < Duration::<u32, 1, 1_000>::from_ticks(2)
        );
        assert!(
            Duration::<u32, 1, 1_000>::from_ticks(1) <= Duration::<u32, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u32, 1, 1_000>::from_ticks(1) <= Duration::<u32, 1, 1_000>::from_ticks(2)
        );
        assert!(
            Duration::<u32, 1, 1_000>::from_ticks(1) == Duration::<u32, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u32, 1, 1_000>::from_ticks(1) != Duration::<u32, 1, 1_000>::from_ticks(2)
        );

        // Different fraction
        assert!(
            Duration::<u32, 1, 10_000>::from_ticks(11) > Duration::<u32, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u32, 1, 10_000>::from_ticks(11) >= Duration::<u32, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u32, 1, 10_000>::from_ticks(10) >= Duration::<u32, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u32, 1, 10_000>::from_ticks(11) < Duration::<u32, 1, 1_000>::from_ticks(2)
        );
        assert!(
            Duration::<u32, 1, 10_000>::from_ticks(1) <= Duration::<u32, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u32, 1, 10_000>::from_ticks(10) <= Duration::<u32, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u32, 1, 10_000>::from_ticks(10) == Duration::<u32, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u32, 1, 10_000>::from_ticks(9) != Duration::<u32, 1, 1_000>::from_ticks(2)
        );

        // From instants
        assert!(
            Instant::<u32, 1, 1_000>::from_ticks(10) - Instant::<u32, 1, 1_000>::from_ticks(5)
                > Duration::<u32, 1, 1_000>::from_ticks(4)
        );
        assert!(
            Instant::<u32, 1, 1_000>::from_ticks(10) - Instant::<u32, 1, 1_000>::from_ticks(5)
                >= Duration::<u32, 1, 1_000>::from_ticks(4)
        );
        assert!(
            Instant::<u32, 1, 1_000>::from_ticks(10) - Instant::<u32, 1, 1_000>::from_ticks(5)
                >= Duration::<u32, 1, 1_000>::from_ticks(5)
        );
        assert!(
            Instant::<u32, 1, 1_000>::from_ticks(10) - Instant::<u32, 1, 1_000>::from_ticks(5)
                < Duration::<u32, 1, 1_000>::from_ticks(6)
        );
        assert!(
            Instant::<u32, 1, 1_000>::from_ticks(10) - Instant::<u32, 1, 1_000>::from_ticks(5)
                <= Duration::<u32, 1, 1_000>::from_ticks(6)
        );
        assert!(
            Instant::<u32, 1, 1_000>::from_ticks(10) - Instant::<u32, 1, 1_000>::from_ticks(5)
                <= Duration::<u32, 1, 1_000>::from_ticks(5)
        );
        assert!(
            Instant::<u32, 1, 1_000>::from_ticks(10) - Instant::<u32, 1, 1_000>::from_ticks(5)
                == Duration::<u32, 1, 1_000>::from_ticks(5)
        );
        assert!(
            Instant::<u32, 1, 1_000>::from_ticks(10) - Instant::<u32, 1, 1_000>::from_ticks(5)
                != Duration::<u32, 1, 1_000>::from_ticks(4)
        );
    }

    #[test]
    fn duration_compare_u64() {
        // Same fraction
        assert!(
            Duration::<u64, 1, 1_000>::from_ticks(2) > Duration::<u64, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u64, 1, 1_000>::from_ticks(2) >= Duration::<u64, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u64, 1, 1_000>::from_ticks(1) >= Duration::<u64, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u64, 1, 1_000>::from_ticks(1) < Duration::<u64, 1, 1_000>::from_ticks(2)
        );
        assert!(
            Duration::<u64, 1, 1_000>::from_ticks(1) <= Duration::<u64, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u64, 1, 1_000>::from_ticks(1) <= Duration::<u64, 1, 1_000>::from_ticks(2)
        );
        assert!(
            Duration::<u64, 1, 1_000>::from_ticks(1) == Duration::<u64, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u64, 1, 1_000>::from_ticks(1) != Duration::<u64, 1, 1_000>::from_ticks(2)
        );

        // Different fraction
        assert!(
            Duration::<u64, 1, 10_000>::from_ticks(11) > Duration::<u64, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u64, 1, 10_000>::from_ticks(11) >= Duration::<u64, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u64, 1, 10_000>::from_ticks(10) >= Duration::<u64, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u64, 1, 10_000>::from_ticks(11) < Duration::<u64, 1, 1_000>::from_ticks(2)
        );
        assert!(
            Duration::<u64, 1, 10_000>::from_ticks(1) <= Duration::<u64, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u64, 1, 10_000>::from_ticks(10) <= Duration::<u64, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u64, 1, 10_000>::from_ticks(10) == Duration::<u64, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u64, 1, 10_000>::from_ticks(9) != Duration::<u64, 1, 1_000>::from_ticks(2)
        );

        // From instants
        assert!(
            Instant::<u64, 1, 1_000>::from_ticks(10) - Instant::<u64, 1, 1_000>::from_ticks(5)
                > Duration::<u64, 1, 1_000>::from_ticks(4)
        );
        assert!(
            Instant::<u64, 1, 1_000>::from_ticks(10) - Instant::<u64, 1, 1_000>::from_ticks(5)
                >= Duration::<u64, 1, 1_000>::from_ticks(4)
        );
        assert!(
            Instant::<u64, 1, 1_000>::from_ticks(10) - Instant::<u64, 1, 1_000>::from_ticks(5)
                >= Duration::<u64, 1, 1_000>::from_ticks(5)
        );
        assert!(
            Instant::<u64, 1, 1_000>::from_ticks(10) - Instant::<u64, 1, 1_000>::from_ticks(5)
                < Duration::<u64, 1, 1_000>::from_ticks(6)
        );
        assert!(
            Instant::<u64, 1, 1_000>::from_ticks(10) - Instant::<u64, 1, 1_000>::from_ticks(5)
                <= Duration::<u64, 1, 1_000>::from_ticks(6)
        );
        assert!(
            Instant::<u64, 1, 1_000>::from_ticks(10) - Instant::<u64, 1, 1_000>::from_ticks(5)
                <= Duration::<u64, 1, 1_000>::from_ticks(5)
        );
        assert!(
            Instant::<u64, 1, 1_000>::from_ticks(10) - Instant::<u64, 1, 1_000>::from_ticks(5)
                == Duration::<u64, 1, 1_000>::from_ticks(5)
        );
        assert!(
            Instant::<u64, 1, 1_000>::from_ticks(10) - Instant::<u64, 1, 1_000>::from_ticks(5)
                != Duration::<u64, 1, 1_000>::from_ticks(4)
        );
    }

    #[test]
    fn duration_compare_u64_u32() {
        // Same fraction
        assert!(
            Duration::<u64, 1, 1_000>::from_ticks(2) > Duration::<u32, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u64, 1, 1_000>::from_ticks(2) >= Duration::<u32, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u64, 1, 1_000>::from_ticks(1) >= Duration::<u32, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u64, 1, 1_000>::from_ticks(1) < Duration::<u32, 1, 1_000>::from_ticks(2)
        );
        assert!(
            Duration::<u64, 1, 1_000>::from_ticks(1) <= Duration::<u32, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u64, 1, 1_000>::from_ticks(1) <= Duration::<u32, 1, 1_000>::from_ticks(2)
        );
        assert!(
            Duration::<u64, 1, 1_000>::from_ticks(1) == Duration::<u32, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u64, 1, 1_000>::from_ticks(1) != Duration::<u32, 1, 1_000>::from_ticks(2)
        );

        // Different fraction
        assert!(
            Duration::<u64, 1, 10_000>::from_ticks(11) > Duration::<u32, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u64, 1, 10_000>::from_ticks(11) >= Duration::<u32, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u64, 1, 10_000>::from_ticks(10) >= Duration::<u32, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u64, 1, 10_000>::from_ticks(11) < Duration::<u32, 1, 1_000>::from_ticks(2)
        );
        assert!(
            Duration::<u64, 1, 10_000>::from_ticks(1) <= Duration::<u32, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u64, 1, 10_000>::from_ticks(10) <= Duration::<u32, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u64, 1, 10_000>::from_ticks(10) == Duration::<u32, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u64, 1, 10_000>::from_ticks(9) != Duration::<u32, 1, 1_000>::from_ticks(2)
        );

        // From instants
        assert!(
            Instant::<u64, 1, 1_000>::from_ticks(10) - Instant::<u64, 1, 1_000>::from_ticks(5)
                > Duration::<u32, 1, 1_000>::from_ticks(4)
        );
        assert!(
            Instant::<u64, 1, 1_000>::from_ticks(10) - Instant::<u64, 1, 1_000>::from_ticks(5)
                >= Duration::<u32, 1, 1_000>::from_ticks(4)
        );
        assert!(
            Instant::<u64, 1, 1_000>::from_ticks(10) - Instant::<u64, 1, 1_000>::from_ticks(5)
                >= Duration::<u32, 1, 1_000>::from_ticks(5)
        );
        assert!(
            Instant::<u64, 1, 1_000>::from_ticks(10) - Instant::<u64, 1, 1_000>::from_ticks(5)
                < Duration::<u32, 1, 1_000>::from_ticks(6)
        );
        assert!(
            Instant::<u64, 1, 1_000>::from_ticks(10) - Instant::<u64, 1, 1_000>::from_ticks(5)
                <= Duration::<u32, 1, 1_000>::from_ticks(6)
        );
        assert!(
            Instant::<u64, 1, 1_000>::from_ticks(10) - Instant::<u64, 1, 1_000>::from_ticks(5)
                <= Duration::<u32, 1, 1_000>::from_ticks(5)
        );
        assert!(
            Instant::<u64, 1, 1_000>::from_ticks(10) - Instant::<u64, 1, 1_000>::from_ticks(5)
                == Duration::<u32, 1, 1_000>::from_ticks(5)
        );
        assert!(
            Instant::<u64, 1, 1_000>::from_ticks(10) - Instant::<u64, 1, 1_000>::from_ticks(5)
                != Duration::<u32, 1, 1_000>::from_ticks(4)
        );
    }

    #[test]
    fn duration_compare_u32_u64() {
        // Same fraction
        assert!(
            Duration::<u32, 1, 1_000>::from_ticks(2) > Duration::<u64, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u32, 1, 1_000>::from_ticks(2) >= Duration::<u64, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u32, 1, 1_000>::from_ticks(1) >= Duration::<u64, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u32, 1, 1_000>::from_ticks(1) < Duration::<u64, 1, 1_000>::from_ticks(2)
        );
        assert!(
            Duration::<u32, 1, 1_000>::from_ticks(1) <= Duration::<u64, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u32, 1, 1_000>::from_ticks(1) <= Duration::<u64, 1, 1_000>::from_ticks(2)
        );
        assert!(
            Duration::<u32, 1, 1_000>::from_ticks(1) == Duration::<u64, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u32, 1, 1_000>::from_ticks(1) != Duration::<u64, 1, 1_000>::from_ticks(2)
        );

        // Different fraction
        assert!(
            Duration::<u32, 1, 10_000>::from_ticks(11) > Duration::<u64, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u32, 1, 10_000>::from_ticks(11) >= Duration::<u64, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u32, 1, 10_000>::from_ticks(10) >= Duration::<u64, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u32, 1, 10_000>::from_ticks(11) < Duration::<u64, 1, 1_000>::from_ticks(2)
        );
        assert!(
            Duration::<u32, 1, 10_000>::from_ticks(1) <= Duration::<u64, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u32, 1, 10_000>::from_ticks(10) <= Duration::<u64, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u32, 1, 10_000>::from_ticks(10) == Duration::<u64, 1, 1_000>::from_ticks(1)
        );
        assert!(
            Duration::<u32, 1, 10_000>::from_ticks(9) != Duration::<u64, 1, 1_000>::from_ticks(2)
        );

        // From instants
        assert!(
            Instant::<u32, 1, 1_000>::from_ticks(10) - Instant::<u32, 1, 1_000>::from_ticks(5)
                > Duration::<u64, 1, 1_000>::from_ticks(4)
        );
        assert!(
            Instant::<u32, 1, 1_000>::from_ticks(10) - Instant::<u32, 1, 1_000>::from_ticks(5)
                >= Duration::<u64, 1, 1_000>::from_ticks(4)
        );
        assert!(
            Instant::<u32, 1, 1_000>::from_ticks(10) - Instant::<u32, 1, 1_000>::from_ticks(5)
                >= Duration::<u64, 1, 1_000>::from_ticks(5)
        );
        assert!(
            Instant::<u32, 1, 1_000>::from_ticks(10) - Instant::<u32, 1, 1_000>::from_ticks(5)
                < Duration::<u64, 1, 1_000>::from_ticks(6)
        );
        assert!(
            Instant::<u32, 1, 1_000>::from_ticks(10) - Instant::<u32, 1, 1_000>::from_ticks(5)
                <= Duration::<u64, 1, 1_000>::from_ticks(6)
        );
        assert!(
            Instant::<u32, 1, 1_000>::from_ticks(10) - Instant::<u32, 1, 1_000>::from_ticks(5)
                <= Duration::<u64, 1, 1_000>::from_ticks(5)
        );
        assert!(
            Instant::<u32, 1, 1_000>::from_ticks(10) - Instant::<u32, 1, 1_000>::from_ticks(5)
                == Duration::<u64, 1, 1_000>::from_ticks(5)
        );
        assert!(
            Instant::<u32, 1, 1_000>::from_ticks(10) - Instant::<u32, 1, 1_000>::from_ticks(5)
                != Duration::<u64, 1, 1_000>::from_ticks(4)
        );
    }

    #[test]
    fn duration_duration_math_u32() {
        use crate::ExtU32;

        // Same base
        let sum: Duration<u32, 1, 1_000> =
            Duration::<u32, 1, 1_000>::from_ticks(10) + Duration::<u32, 1, 1_000>::from_ticks(1);
        assert_eq!(sum, Duration::<u32, 1, 1_000>::from_ticks(11));

        let mut sum = Duration::<u32, 1, 1_000>::from_ticks(10);
        sum += Duration::<u32, 1, 1_000>::from_ticks(1);
        assert_eq!(sum, Duration::<u32, 1, 1_000>::from_ticks(11));

        let diff: Duration<u32, 1, 1_000> =
            Duration::<u32, 1, 1_000>::from_ticks(10) - Duration::<u32, 1, 1_000>::from_ticks(1);
        assert_eq!(diff, Duration::<u32, 1, 1_000>::from_ticks(9));

        let mut diff = Duration::<u32, 1, 1_000>::from_ticks(10);
        diff -= Duration::<u32, 1, 1_000>::from_ticks(1);
        assert_eq!(diff, Duration::<u32, 1, 1_000>::from_ticks(9));

        // Different base
        let sum: Duration<u32, 1, 10_000> = Duration::<u32, 1, 10_000>::from_ticks(10)
            + Duration::<u32, 1, 1_000>::from_ticks(1).convert();
        assert_eq!(sum, Duration::<u32, 1, 1_000>::from_ticks(2));

        let mut sum = Duration::<u32, 1, 1_000>::from_ticks(1);
        sum += Duration::<u32, 1, 10_000>::from_ticks(10).convert();
        assert_eq!(sum, Duration::<u32, 1, 1_000>::from_ticks(2));

        let diff: Duration<u32, 1, 10_000> = Duration::<u32, 1, 10_000>::from_ticks(10)
            - Duration::<u32, 1, 1_000>::from_ticks(1).convert();
        assert_eq!(diff, Duration::<u32, 1, 10_000>::from_ticks(0));

        let mut diff = Duration::<u32, 1, 1_000>::from_ticks(1);
        diff -= Duration::<u32, 1, 10_000>::from_ticks(10).convert();
        assert_eq!(diff, Duration::<u32, 1, 1_000>::from_ticks(0));

        // Short hand vs u32 (should not need `.into()`)
        let sum = Duration::<u32, 1, 10_000>::from_ticks(10) + 1.millis();
        assert_eq!(sum, Duration::<u32, 1, 10_000>::from_ticks(20));

        let mut sum = Duration::<u32, 1, 10_000>::from_ticks(10);
        sum += 1.millis();
        assert_eq!(sum, Duration::<u32, 1, 10_000>::from_ticks(20));

        // Fixed in v0.3.2
        let d: Duration<u32, 1, 1_000> = Duration::<u32, 1, 32_768>::from_ticks(42949672).convert();
        assert_eq!(d.ticks(), 1_310_719);

        // Division and multiplication by integers
        let mul: Duration<u32, 1, 1_000> = Duration::<u32, 1, 1_000>::from_ticks(10) * 2;
        assert_eq!(mul, Duration::<u32, 1, 1_000>::from_ticks(20));

        let mut mul = Duration::<u32, 1, 1_000>::from_ticks(10);
        mul *= 2;
        assert_eq!(mul, Duration::<u32, 1, 1_000>::from_ticks(20));

        let div: Duration<u32, 1, 1_000> = Duration::<u32, 1, 1_000>::from_ticks(10) / 2;
        assert_eq!(div, Duration::<u32, 1, 1_000>::from_ticks(5));

        let mut div = Duration::<u32, 1, 1_000>::from_ticks(10);
        div /= 2;
        assert_eq!(div, Duration::<u32, 1, 1_000>::from_ticks(5));
    }

    #[test]
    fn duration_duration_math_u64() {
        use crate::ExtU64;

        // Same base
        let sum: Duration<u64, 1, 1_000> =
            Duration::<u64, 1, 1_000>::from_ticks(10) + Duration::<u64, 1, 1_000>::from_ticks(1);
        assert_eq!(sum, Duration::<u64, 1, 1_000>::from_ticks(11));

        let mut sum = Duration::<u64, 1, 1_000>::from_ticks(10);
        sum += Duration::<u64, 1, 1_000>::from_ticks(1);
        assert_eq!(sum, Duration::<u64, 1, 1_000>::from_ticks(11));

        let diff: Duration<u64, 1, 1_000> =
            Duration::<u64, 1, 1_000>::from_ticks(10) - Duration::<u64, 1, 1_000>::from_ticks(1);
        assert_eq!(diff, Duration::<u64, 1, 1_000>::from_ticks(9));

        let mut diff = Duration::<u64, 1, 1_000>::from_ticks(10);
        diff -= Duration::<u64, 1, 1_000>::from_ticks(1);
        assert_eq!(diff, Duration::<u64, 1, 1_000>::from_ticks(9));

        // Different base
        let sum: Duration<u64, 1, 10_000> = Duration::<u64, 1, 10_000>::from_ticks(10)
            + Duration::<u64, 1, 1_000>::from_ticks(1).convert();
        assert_eq!(sum, Duration::<u64, 1, 1_000>::from_ticks(2));

        let mut sum = Duration::<u64, 1, 1_000>::from_ticks(1);
        sum += Duration::<u64, 1, 10_000>::from_ticks(10).convert();
        assert_eq!(sum, Duration::<u64, 1, 1_000>::from_ticks(2));

        let diff: Duration<u64, 1, 10_000> = Duration::<u64, 1, 10_000>::from_ticks(10)
            - Duration::<u64, 1, 1_000>::from_ticks(1).convert();
        assert_eq!(diff, Duration::<u64, 1, 1_000>::from_ticks(0));

        let mut diff = Duration::<u64, 1, 1_000>::from_ticks(1);
        diff -= Duration::<u64, 1, 10_000>::from_ticks(10).convert();
        assert_eq!(diff, Duration::<u64, 1, 1_000>::from_ticks(0));

        // Short hand vs u64 (should not need `.into()`)
        let sum = Duration::<u64, 1, 10_000>::from_ticks(10) + 1.millis();
        assert_eq!(sum, Duration::<u64, 1, 10_000>::from_ticks(20));

        let mut sum = Duration::<u64, 1, 10_000>::from_ticks(10);
        sum += 1.millis();
        assert_eq!(sum, Duration::<u64, 1, 10_000>::from_ticks(20));

        // Division and multiplication by integers
        let mul: Duration<u64, 1, 1_000> = Duration::<u64, 1, 1_000>::from_ticks(10) * 2;
        assert_eq!(mul, Duration::<u64, 1, 1_000>::from_ticks(20));

        let mut mul = Duration::<u64, 1, 1_000>::from_ticks(10);
        mul *= 2;
        assert_eq!(mul, Duration::<u64, 1, 1_000>::from_ticks(20));

        let div: Duration<u64, 1, 1_000> = Duration::<u64, 1, 1_000>::from_ticks(10) / 2;
        assert_eq!(div, Duration::<u64, 1, 1_000>::from_ticks(5));

        let mut div = Duration::<u64, 1, 1_000>::from_ticks(10);
        div /= 2;
        assert_eq!(div, Duration::<u64, 1, 1_000>::from_ticks(5));
    }

    #[test]
    fn duration_duration_math_u64_u32() {
        // Same base
        let sum: Duration<u64, 1, 1_000> =
            Duration::<u64, 1, 1_000>::from_ticks(10) + Duration::<u32, 1, 1_000>::from_ticks(1);
        assert_eq!(sum, Duration::<u64, 1, 1_000>::from_ticks(11));

        let mut sum = Duration::<u64, 1, 1_000>::from_ticks(10);
        sum += Duration::<u32, 1, 1_000>::from_ticks(1);
        assert_eq!(sum, Duration::<u64, 1, 1_000>::from_ticks(11));

        let diff: Duration<u64, 1, 1_000> =
            Duration::<u64, 1, 1_000>::from_ticks(10) - Duration::<u32, 1, 1_000>::from_ticks(1);
        assert_eq!(diff, Duration::<u64, 1, 1_000>::from_ticks(9));

        let mut diff = Duration::<u64, 1, 1_000>::from_ticks(10);
        diff -= Duration::<u32, 1, 1_000>::from_ticks(1);
        assert_eq!(diff, Duration::<u64, 1, 1_000>::from_ticks(9));

        // Different base
        let sum: Duration<u64, 1, 10_000> = Duration::<u64, 1, 10_000>::from_ticks(10)
            + Duration::<u32, 1, 1_000>::from_ticks(1).convert();
        assert_eq!(sum, Duration::<u64, 1, 1_000>::from_ticks(2));

        let mut sum = Duration::<u64, 1, 1_000>::from_ticks(1);
        sum += Duration::<u32, 1, 10_000>::from_ticks(10).convert();
        assert_eq!(sum, Duration::<u64, 1, 1_000>::from_ticks(2));

        let diff: Duration<u64, 1, 10_000> = Duration::<u64, 1, 10_000>::from_ticks(10)
            - Duration::<u32, 1, 1_000>::from_ticks(1).convert();
        assert_eq!(diff, Duration::<u64, 1, 1_000>::from_ticks(0));

        let mut diff = Duration::<u64, 1, 1_000>::from_ticks(1);
        diff -= Duration::<u32, 1, 10_000>::from_ticks(10).convert();
        assert_eq!(diff, Duration::<u64, 1, 1_000>::from_ticks(0));
    }

    #[test]
    fn duration_shorthands_u32() {
        use crate::ExtU32;

        let d: Duration<u32, 1, 10_000> = 100_000.micros();
        assert_eq!(d.ticks(), 1_000);

        let d: Duration<u32, 1, 10_000> = 1.millis();
        assert_eq!(d.ticks(), 10);

        let d: Duration<u32, 1, 10_000> = 1.secs();
        assert_eq!(d.ticks(), 10_000);

        let d: Duration<u32, 1, 10_000> = 1.minutes();
        assert_eq!(d.ticks(), 600_000);

        let d: Duration<u32, 1, 10_000> = 1.hours();
        assert_eq!(d.ticks(), 36_000_000);
    }

    #[test]
    fn duration_shorthands_u64() {
        use crate::ExtU64;

        let d: Duration<u64, 1, 10_000> = 100_000.micros();
        assert_eq!(d.ticks(), 1_000);

        let d: Duration<u64, 1, 10_000> = 1.millis();
        assert_eq!(d.ticks(), 10);

        let d: Duration<u64, 1, 10_000> = 1.secs();
        assert_eq!(d.ticks(), 10_000);

        let d: Duration<u64, 1, 10_000> = 1.minutes();
        assert_eq!(d.ticks(), 600_000);

        let d: Duration<u64, 1, 10_000> = 1.hours();
        assert_eq!(d.ticks(), 36_000_000);
    }

    #[test]
    fn instant_compare_u32() {
        // Wrapping
        assert!(
            Instant::<u32, 1, 1_000>::from_ticks(1)
                > Instant::<u32, 1, 1_000>::from_ticks(u32::MAX)
        );
        assert!(
            Instant::<u32, 1, 1_000>::from_ticks(u32::MAX - 1)
                < Instant::<u32, 1, 1_000>::from_ticks(u32::MAX)
        );

        // Non-wrapping
        assert!(Instant::<u32, 1, 1_000>::from_ticks(2) > Instant::<u32, 1, 1_000>::from_ticks(1));
        assert!(Instant::<u32, 1, 1_000>::from_ticks(2) >= Instant::<u32, 1, 1_000>::from_ticks(1));
        assert!(Instant::<u32, 1, 1_000>::from_ticks(1) >= Instant::<u32, 1, 1_000>::from_ticks(1));
        assert!(Instant::<u32, 1, 1_000>::from_ticks(1) < Instant::<u32, 1, 1_000>::from_ticks(2));
        assert!(Instant::<u32, 1, 1_000>::from_ticks(1) <= Instant::<u32, 1, 1_000>::from_ticks(1));
        assert!(Instant::<u32, 1, 1_000>::from_ticks(1) <= Instant::<u32, 1, 1_000>::from_ticks(2));
        assert!(Instant::<u32, 1, 1_000>::from_ticks(1) == Instant::<u32, 1, 1_000>::from_ticks(1));
        assert!(Instant::<u32, 1, 1_000>::from_ticks(1) != Instant::<u32, 1, 1_000>::from_ticks(2));

        // Checked duration since non-wrapping
        assert_eq!(
            Instant::<u32, 1, 1_000>::from_ticks(1)
                .checked_duration_since(Instant::<u32, 1, 1_000>::from_ticks(1)),
            Some(Duration::<u32, 1, 1_000>::from_ticks(0))
        );
        assert_eq!(
            Instant::<u32, 1, 1_000>::from_ticks(2)
                .checked_duration_since(Instant::<u32, 1, 1_000>::from_ticks(1)),
            Some(Duration::<u32, 1, 1_000>::from_ticks(1))
        );
        assert_eq!(
            Instant::<u32, 1, 1_000>::from_ticks(2)
                .checked_duration_since(Instant::<u32, 1, 1_000>::from_ticks(3)),
            None
        );

        // Checked duration since wrapping
        assert_eq!(
            Instant::<u32, 1, 1_000>::from_ticks(2)
                .checked_duration_since(Instant::<u32, 1, 1_000>::from_ticks(u32::MAX)),
            Some(Duration::<u32, 1, 1_000>::from_ticks(3))
        );
        assert_eq!(
            Instant::<u32, 1, 1_000>::from_ticks(2)
                .checked_duration_since(Instant::<u32, 1, 1_000>::from_ticks(u32::MAX - 1)),
            Some(Duration::<u32, 1, 1_000>::from_ticks(4))
        );
    }

    #[test]
    fn instant_compare_u64() {
        // Wrapping
        assert!(
            Instant::<u64, 1, 1_000>::from_ticks(1)
                > Instant::<u64, 1, 1_000>::from_ticks(u64::MAX)
        );
        assert!(
            Instant::<u64, 1, 1_000>::from_ticks(u64::MAX - 1)
                < Instant::<u64, 1, 1_000>::from_ticks(u64::MAX)
        );

        // Non-wrapping
        assert!(Instant::<u64, 1, 1_000>::from_ticks(2) > Instant::<u64, 1, 1_000>::from_ticks(1));
        assert!(Instant::<u64, 1, 1_000>::from_ticks(2) >= Instant::<u64, 1, 1_000>::from_ticks(1));
        assert!(Instant::<u64, 1, 1_000>::from_ticks(1) >= Instant::<u64, 1, 1_000>::from_ticks(1));
        assert!(Instant::<u64, 1, 1_000>::from_ticks(1) < Instant::<u64, 1, 1_000>::from_ticks(2));
        assert!(Instant::<u64, 1, 1_000>::from_ticks(1) <= Instant::<u64, 1, 1_000>::from_ticks(1));
        assert!(Instant::<u64, 1, 1_000>::from_ticks(1) <= Instant::<u64, 1, 1_000>::from_ticks(2));
        assert!(Instant::<u64, 1, 1_000>::from_ticks(1) == Instant::<u64, 1, 1_000>::from_ticks(1));
        assert!(Instant::<u64, 1, 1_000>::from_ticks(1) != Instant::<u64, 1, 1_000>::from_ticks(2));

        // Checked duration since non-wrapping
        assert_eq!(
            Instant::<u64, 1, 1_000>::from_ticks(1)
                .checked_duration_since(Instant::<u64, 1, 1_000>::from_ticks(1)),
            Some(Duration::<u64, 1, 1_000>::from_ticks(0))
        );
        assert_eq!(
            Instant::<u64, 1, 1_000>::from_ticks(2)
                .checked_duration_since(Instant::<u64, 1, 1_000>::from_ticks(1)),
            Some(Duration::<u64, 1, 1_000>::from_ticks(1))
        );
        assert_eq!(
            Instant::<u64, 1, 1_000>::from_ticks(2)
                .checked_duration_since(Instant::<u64, 1, 1_000>::from_ticks(3)),
            None
        );

        // Checked duration since wrapping
        assert_eq!(
            Instant::<u64, 1, 1_000>::from_ticks(2)
                .checked_duration_since(Instant::<u64, 1, 1_000>::from_ticks(u64::MAX)),
            Some(Duration::<u64, 1, 1_000>::from_ticks(3))
        );
        assert_eq!(
            Instant::<u64, 1, 1_000>::from_ticks(2)
                .checked_duration_since(Instant::<u64, 1, 1_000>::from_ticks(u64::MAX - 1)),
            Some(Duration::<u64, 1, 1_000>::from_ticks(4))
        );
    }

    #[test]
    fn instant_duration_math_u32() {
        use crate::ExtU32;

        // Instant - Instant, Same base
        let diff: Duration<u32, 1, 1_000> =
            Instant::<u32, 1, 1_000>::from_ticks(10) - Instant::<u32, 1, 1_000>::from_ticks(1);
        assert_eq!(diff, Duration::<u32, 1, 1_000>::from_ticks(9));

        // Instant +- Duration, Same base
        let sum: Instant<u32, 1, 1_000> =
            Instant::<u32, 1, 1_000>::from_ticks(10) + Duration::<u32, 1, 1_000>::from_ticks(1);
        assert_eq!(sum, Instant::<u32, 1, 1_000>::from_ticks(11));

        let mut sum = Instant::<u32, 1, 1_000>::from_ticks(10);
        sum += Duration::<u32, 1, 1_000>::from_ticks(1);
        assert_eq!(sum, Instant::<u32, 1, 1_000>::from_ticks(11));

        let diff: Instant<u32, 1, 1_000> =
            Instant::<u32, 1, 1_000>::from_ticks(10) - Duration::<u32, 1, 1_000>::from_ticks(1);
        assert_eq!(diff, Instant::<u32, 1, 1_000>::from_ticks(9));

        let mut diff = Instant::<u32, 1, 1_000>::from_ticks(10);
        diff -= Duration::<u32, 1, 1_000>::from_ticks(1);
        assert_eq!(diff, Instant::<u32, 1, 1_000>::from_ticks(9));

        // Instant +- Duration, Different base
        let sum: Instant<u32, 1, 10_000> = Instant::<u32, 1, 10_000>::from_ticks(10)
            + Duration::<u32, 1, 1_000>::from_ticks(1).convert();
        assert_eq!(sum, Instant::<u32, 1, 10_000>::from_ticks(20));

        let mut sum = Instant::<u32, 1, 10_000>::from_ticks(10);
        sum += Duration::<u32, 1, 1_000>::from_ticks(1).convert();
        assert_eq!(sum, Instant::<u32, 1, 10_000>::from_ticks(20));

        let diff: Instant<u32, 1, 10_000> = Instant::<u32, 1, 10_000>::from_ticks(10)
            - Duration::<u32, 1, 1_000>::from_ticks(1).convert();
        assert_eq!(diff, Instant::<u32, 1, 10_000>::from_ticks(0));

        let mut diff = Instant::<u32, 1, 10_000>::from_ticks(10);
        diff -= Duration::<u32, 1, 1_000>::from_ticks(1).convert();
        assert_eq!(diff, Instant::<u32, 1, 10_000>::from_ticks(0));

        // Instant + Extension trait
        let sum: Instant<u32, 1, 10_000> = Instant::<u32, 1, 10_000>::from_ticks(10) + 1.millis();
        assert_eq!(sum, Instant::<u32, 1, 10_000>::from_ticks(20));

        // Instant - Extension trait
        let diff: Instant<u32, 1, 10_000> = Instant::<u32, 1, 10_000>::from_ticks(10) - 1.millis();
        assert_eq!(diff, Instant::<u32, 1, 10_000>::from_ticks(0));
    }

    #[test]
    fn instant_duration_math_u64() {
        use crate::ExtU64;

        // Instant - Instant, Same base
        let diff: Duration<u64, 1, 1_000> =
            Instant::<u64, 1, 1_000>::from_ticks(10) - Instant::<u64, 1, 1_000>::from_ticks(1);
        assert_eq!(diff, Duration::<u64, 1, 1_000>::from_ticks(9));

        // Instant +- Duration, Same base
        let sum: Instant<u64, 1, 1_000> =
            Instant::<u64, 1, 1_000>::from_ticks(10) + Duration::<u64, 1, 1_000>::from_ticks(1);
        assert_eq!(sum, Instant::<u64, 1, 1_000>::from_ticks(11));

        let mut sum = Instant::<u64, 1, 1_000>::from_ticks(10);
        sum += Duration::<u64, 1, 1_000>::from_ticks(1);
        assert_eq!(sum, Instant::<u64, 1, 1_000>::from_ticks(11));

        let diff: Instant<u64, 1, 1_000> =
            Instant::<u64, 1, 1_000>::from_ticks(10) - Duration::<u64, 1, 1_000>::from_ticks(1);
        assert_eq!(diff, Instant::<u64, 1, 1_000>::from_ticks(9));

        let mut diff = Instant::<u64, 1, 1_000>::from_ticks(10);
        diff -= Duration::<u64, 1, 1_000>::from_ticks(1);
        assert_eq!(diff, Instant::<u64, 1, 1_000>::from_ticks(9));

        // Instant +- Duration, Different base
        let sum: Instant<u64, 1, 10_000> = Instant::<u64, 1, 10_000>::from_ticks(10)
            + Duration::<u64, 1, 1_000>::from_ticks(1).convert();
        assert_eq!(sum, Instant::<u64, 1, 10_000>::from_ticks(20));

        let mut sum = Instant::<u64, 1, 10_000>::from_ticks(10);
        sum += Duration::<u64, 1, 1_000>::from_ticks(1).convert();
        assert_eq!(sum, Instant::<u64, 1, 10_000>::from_ticks(20));

        let diff: Instant<u64, 1, 10_000> = Instant::<u64, 1, 10_000>::from_ticks(10)
            - Duration::<u64, 1, 1_000>::from_ticks(1).convert();
        assert_eq!(diff, Instant::<u64, 1, 10_000>::from_ticks(0));

        let mut diff = Instant::<u64, 1, 10_000>::from_ticks(10);
        diff -= Duration::<u64, 1, 1_000>::from_ticks(1).convert();
        assert_eq!(diff, Instant::<u64, 1, 10_000>::from_ticks(0));

        // Instant + Extension trait
        let sum: Instant<u64, 1, 10_000> = Instant::<u64, 1, 10_000>::from_ticks(10) + 1.millis();
        assert_eq!(sum, Instant::<u64, 1, 10_000>::from_ticks(20));

        // Instant - Extension trait
        let diff: Instant<u64, 1, 10_000> = Instant::<u64, 1, 10_000>::from_ticks(10) - 1.millis();
        assert_eq!(diff, Instant::<u64, 1, 10_000>::from_ticks(0));
    }

    #[test]
    fn instant_duration_math_u64_u32() {
        // Instant +- Duration, Same base
        let sum: Instant<u64, 1, 1_000> =
            Instant::<u64, 1, 1_000>::from_ticks(10) + Duration::<u32, 1, 1_000>::from_ticks(1);
        assert_eq!(sum, Instant::<u64, 1, 1_000>::from_ticks(11));

        let mut sum = Instant::<u64, 1, 1_000>::from_ticks(10);
        sum += Duration::<u32, 1, 1_000>::from_ticks(1);
        assert_eq!(sum, Instant::<u64, 1, 1_000>::from_ticks(11));

        let diff: Instant<u64, 1, 1_000> =
            Instant::<u64, 1, 1_000>::from_ticks(10) - Duration::<u32, 1, 1_000>::from_ticks(1);
        assert_eq!(diff, Instant::<u64, 1, 1_000>::from_ticks(9));

        let mut diff = Instant::<u64, 1, 1_000>::from_ticks(10);
        diff -= Duration::<u32, 1, 1_000>::from_ticks(1);
        assert_eq!(diff, Instant::<u64, 1, 1_000>::from_ticks(9));

        // Instant +- Duration, Different base
        let sum: Instant<u64, 1, 10_000> = Instant::<u64, 1, 10_000>::from_ticks(10)
            + Duration::<u32, 1, 1_000>::from_ticks(1).convert();
        assert_eq!(sum, Instant::<u64, 1, 10_000>::from_ticks(20));

        let mut sum = Instant::<u64, 1, 10_000>::from_ticks(10);
        sum += Duration::<u32, 1, 1_000>::from_ticks(1).convert();
        assert_eq!(sum, Instant::<u64, 1, 10_000>::from_ticks(20));

        let diff: Instant<u64, 1, 10_000> = Instant::<u64, 1, 10_000>::from_ticks(10)
            - Duration::<u32, 1, 1_000>::from_ticks(1).convert();
        assert_eq!(diff, Instant::<u64, 1, 10_000>::from_ticks(0));

        let mut diff = Instant::<u64, 1, 10_000>::from_ticks(10);
        diff -= Duration::<u32, 1, 1_000>::from_ticks(1).convert();
        assert_eq!(diff, Instant::<u64, 1, 10_000>::from_ticks(0));
    }
}
