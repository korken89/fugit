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
mod rate;

pub use aliases::*;
pub use duration::{Duration, ExtU32, ExtU32Ceil, ExtU64, ExtU64Ceil};
pub use instant::Instant;
pub use rate::{ExtU32 as RateExtU32, ExtU64 as RateExtU64, Rate};

#[cfg(test)]
mod test {
    use crate::Duration;
    use crate::Instant;
    use crate::Rate;
    use crate::{
        Hertz, HertzU32, HertzU64, Kilohertz, KilohertzU32, KilohertzU64, Megahertz, MegahertzU32,
        MegahertzU64, TimerRate, TimerRateU32, TimerRateU64,
    };

    ////////////////////////////////////////////////////////////////////////////////
    //
    // Duration tests
    //
    ////////////////////////////////////////////////////////////////////////////////

    #[test]
    fn large_duration_converstion() {
        use crate::ExtU64;

        let sum = Duration::<u64, 1, 80_000_000>::from_ticks(0) + 15.minutes();

        assert_eq!(
            sum,
            Duration::<u64, 1, 80_000_000>::from_ticks(80_000_000 * 60 * 15)
        );
    }

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
        assert_eq!(d.as_ticks(), 1_310_719);

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

        assert_eq!(
            Duration::<u32, 1, 100>::from_ticks(5) / Duration::<u32, 1, 1_000>::from_ticks(2),
            25
        );

        assert_eq!(
            Duration::<u32, 1, 1_000>::from_ticks(2) / Duration::<u32, 1, 100>::from_ticks(5),
            0
        );

        assert_eq!(
            Duration::<u32, 1, 1_000>::from_ticks(500) / Duration::<u32, 1, 100>::from_ticks(5),
            10
        );
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

        assert_eq!(
            Duration::<u64, 1, 1_00>::from_ticks(5) / Duration::<u64, 1, 1_000>::from_ticks(2),
            25
        );

        assert_eq!(
            Duration::<u64, 1, 1_000>::from_ticks(2) / Duration::<u64, 1, 1_00>::from_ticks(5),
            0
        );

        assert_eq!(
            Duration::<u64, 1, 1_000>::from_ticks(500) / Duration::<u64, 1, 1_00>::from_ticks(5),
            10
        );
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
        use crate::{ExtU32, ExtU32Ceil};

        let d: Duration<u32, 1, 10_000> = 100_000_000.nanos();
        assert_eq!(d.as_ticks(), 1_000);

        let d: Duration<u32, 1, 1_000_000> = 40_000.nanos_at_least();
        assert_eq!(d.as_ticks(), 40);

        let d: Duration<u32, 1, 1_000_000> = 40_075.nanos_at_least();
        assert_eq!(d.as_ticks(), 41);

        let d: Duration<u32, 1, 1_000> = 4001.micros_at_least();
        assert_eq!(d.as_ticks(), 5);

        let d: Duration<u32, 1, 10_000> = 100_000.micros();
        assert_eq!(d.as_ticks(), 1_000);

        let d: Duration<u32, 1, 10_000> = 1.millis();
        assert_eq!(d.as_ticks(), 10);

        let d: Duration<u32, 1, 10_000> = 1.secs();
        assert_eq!(d.as_ticks(), 10_000);

        let d: Duration<u32, 1, 10_000> = 1.minutes();
        assert_eq!(d.as_ticks(), 600_000);

        let d: Duration<u32, 1, 10_000> = 1.hours();
        assert_eq!(d.as_ticks(), 36_000_000);

        let d = Duration::<u32, 1, 10_000>::from_millis(10);
        assert_eq!(d.as_ticks(), 100);

        let d = Duration::<u32, 1, 10_000>::Hz(200);
        assert_eq!(d.as_ticks(), 50);

        let d = Duration::<u32, 1, 1>::from_ticks(2);
        assert_eq!(d.as_secs(), 2);
        assert_eq!(d.as_nanos(), 2_000_000_000);

        let d = Duration::<u32, 1, 1_000_000_000>::from_ticks(2_000_000_000);
        assert_eq!(d.as_secs(), 2);
        assert_eq!(d.as_nanos(), 2_000_000_000);

        let d = Duration::<u32, 1, 10_000>::from_ticks(100);
        assert_eq!(d.as_nanos(), 10_000_000);

        let d = Duration::<u32, 1, 10_000>::from_ticks(100);
        assert_eq!(d.as_micros(), 10_000);

        let d = Duration::<u32, 1, 10_000>::from_ticks(100);
        assert_eq!(d.as_millis(), 10);

        let d = Duration::<u32, 1, 10_000>::from_ticks(100_000);
        assert_eq!(d.as_secs(), 10);

        let d = Duration::<u32, 1, 10_000>::from_ticks(1_800_000);
        assert_eq!(d.as_minutes(), 3);

        let d = Duration::<u32, 1, 10_000>::from_ticks(180_000_000);
        assert_eq!(d.as_hours(), 5);
    }

    #[test]
    fn duration_shorthands_u64() {
        use crate::{ExtU64, ExtU64Ceil};

        let d: Duration<u64, 1, 10_000> = 100_000_000.nanos();
        assert_eq!(d.as_ticks(), 1_000);

        let d: Duration<u64, 1, 1_000_000> = 40_000.nanos_at_least();
        assert_eq!(d.as_ticks(), 40);

        let d: Duration<u64, 1, 1_000_000> = 40_075.nanos_at_least();
        assert_eq!(d.as_ticks(), 41);

        let d: Duration<u64, 1, 1_000> = 4001.micros_at_least();
        assert_eq!(d.as_ticks(), 5);

        let d: Duration<u64, 1, 10_000> = 100_000.micros();
        assert_eq!(d.as_ticks(), 1_000);

        let d: Duration<u64, 1, 10_000> = 1.millis();
        assert_eq!(d.as_ticks(), 10);

        let d: Duration<u64, 1, 10_000> = 1.secs();
        assert_eq!(d.as_ticks(), 10_000);

        let d: Duration<u64, 1, 10_000> = 1.minutes();
        assert_eq!(d.as_ticks(), 600_000);

        let d: Duration<u64, 1, 10_000> = 1.hours();
        assert_eq!(d.as_ticks(), 36_000_000);

        let d = Duration::<u64, 1, 10_000>::from_millis(10);
        assert_eq!(d.as_ticks(), 100);

        let d = Duration::<u64, 1, 10_000>::Hz(200);
        assert_eq!(d.as_ticks(), 50);

        let d = Duration::<u32, 1, 1>::from_ticks(2);
        assert_eq!(d.as_secs(), 2);
        assert_eq!(d.as_nanos(), 2_000_000_000);

        let d = Duration::<u32, 1, 1_000_000_000>::from_ticks(2_000_000_000);
        assert_eq!(d.as_secs(), 2);
        assert_eq!(d.as_nanos(), 2_000_000_000);

        let d = Duration::<u64, 1, 10_000>::from_ticks(100);
        assert_eq!(d.as_nanos(), 10_000_000);

        let d = Duration::<u64, 1, 10_000>::from_ticks(100);
        assert_eq!(d.as_micros(), 10_000);

        let d = Duration::<u64, 1, 10_000>::from_ticks(100);
        assert_eq!(d.as_millis(), 10);

        let d = Duration::<u64, 1, 10_000>::from_ticks(100_000);
        assert_eq!(d.as_secs(), 10);

        let d = Duration::<u64, 1, 10_000>::from_ticks(1_800_000);
        assert_eq!(d.as_minutes(), 3);

        let d = Duration::<u64, 1, 10_000>::from_ticks(180_000_000);
        assert_eq!(d.as_hours(), 5);
    }

    #[test]
    fn duration_is_zero() {
        let d = Duration::<u64, 1, 1_000>::from_ticks(0);
        assert!(d.is_zero());
        let d = Duration::<u64, 1, 1_000>::from_ticks(1);
        assert!(!d.is_zero());
        let d = Duration::<u32, 1, 1_000>::from_ticks(0);
        assert!(d.is_zero());
        let d = Duration::<u32, 1, 1_000>::from_ticks(1);
        assert!(!d.is_zero());
    }

    #[test]
    fn duration_as_secs_f32() {
        let d = Duration::<u32, 1, 1>::from_ticks(5);
        assert!((d.as_secs_f32() - 5.0).abs() < 1e-6);

        let d = Duration::<u32, 1, 1_000>::from_ticks(1_500);
        assert!((d.as_secs_f32() - 1.5).abs() < 1e-6);

        let d = Duration::<u64, 1, 1>::from_ticks(10);
        assert!((d.as_secs_f32() - 10.0).abs() < 1e-6);

        let d = Duration::<u64, 60, 1>::from_ticks(2);
        assert!((d.as_secs_f32() - 120.0).abs() < 1e-6);
    }

    #[test]
    fn duration_as_secs_f64() {
        let d = Duration::<u32, 1, 1>::from_ticks(5);
        assert_eq!(d.as_secs_f64(), 5.0);

        let d = Duration::<u32, 1, 1_000>::from_ticks(1_500);
        assert_eq!(d.as_secs_f64(), 1.5);

        let d = Duration::<u64, 1, 1>::from_ticks(10);
        assert_eq!(d.as_secs_f64(), 10.0);

        let d = Duration::<u64, 60, 1>::from_ticks(2);
        assert_eq!(d.as_secs_f64(), 120.0);
    }

    ////////////////////////////////////////////////////////////////////////////////
    //
    // Instant tests
    //
    ////////////////////////////////////////////////////////////////////////////////

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

    ////////////////////////////////////////////////////////////////////////////////
    //
    // Rate tests
    //
    ////////////////////////////////////////////////////////////////////////////////

    fn take_khz(r: Rate<u32, 1_000, 1>) -> Rate<u32, 1_000, 1> {
        r
    }

    #[test]
    fn rate_functions() {
        assert_eq!(
            take_khz(Rate::<u32, 10_000, 1>::from_raw(1).convert()),
            Rate::<u32, 1_000, 1>::from_raw(10)
        );
    }

    #[test]
    fn rate_compare_u32() {
        // Same fraction
        assert!(Rate::<u32, 1_000, 1>::from_raw(2) > Rate::<u32, 1_000, 1>::from_raw(1));
        assert!(Rate::<u32, 1_000, 1>::from_raw(2) >= Rate::<u32, 1_000, 1>::from_raw(1));
        assert!(Rate::<u32, 1_000, 1>::from_raw(1) >= Rate::<u32, 1_000, 1>::from_raw(1));
        assert!(Rate::<u32, 1_000, 1>::from_raw(1) < Rate::<u32, 1_000, 1>::from_raw(2));
        assert!(Rate::<u32, 1_000, 1>::from_raw(1) <= Rate::<u32, 1_000, 1>::from_raw(1));
        assert!(Rate::<u32, 1_000, 1>::from_raw(1) <= Rate::<u32, 1_000, 1>::from_raw(2));
        assert!(Rate::<u32, 1_000, 1>::from_raw(1) == Rate::<u32, 1_000, 1>::from_raw(1));
        assert!(Rate::<u32, 1_000, 1>::from_raw(1) != Rate::<u32, 1_000, 1>::from_raw(2));

        // Different fraction
        assert!(Rate::<u32, 1_000, 1>::from_raw(11) > Rate::<u32, 10_000, 1>::from_raw(1));
        assert!(Rate::<u32, 1_000, 1>::from_raw(11) >= Rate::<u32, 10_000, 1>::from_raw(1));
        assert!(Rate::<u32, 1_000, 1>::from_raw(10) >= Rate::<u32, 10_000, 1>::from_raw(1));
        assert!(Rate::<u32, 1_000, 1>::from_raw(11) < Rate::<u32, 10_000, 1>::from_raw(2));
        assert!(Rate::<u32, 1_000, 1>::from_raw(1) <= Rate::<u32, 10_000, 1>::from_raw(1));
        assert!(Rate::<u32, 1_000, 1>::from_raw(10) <= Rate::<u32, 10_000, 1>::from_raw(1));
        assert!(Rate::<u32, 1_000, 1>::from_raw(10) == Rate::<u32, 10_000, 1>::from_raw(1));
        assert!(Rate::<u32, 1_000, 1>::from_raw(9) != Rate::<u32, 10_000, 1>::from_raw(2));
    }

    #[test]
    fn rate_compare_u64() {
        // Same fraction
        assert!(Rate::<u64, 1_000, 1>::from_raw(2) > Rate::<u64, 1_000, 1>::from_raw(1));
        assert!(Rate::<u64, 1_000, 1>::from_raw(2) >= Rate::<u64, 1_000, 1>::from_raw(1));
        assert!(Rate::<u64, 1_000, 1>::from_raw(1) >= Rate::<u64, 1_000, 1>::from_raw(1));
        assert!(Rate::<u64, 1_000, 1>::from_raw(1) < Rate::<u64, 1_000, 1>::from_raw(2));
        assert!(Rate::<u64, 1_000, 1>::from_raw(1) <= Rate::<u64, 1_000, 1>::from_raw(1));
        assert!(Rate::<u64, 1_000, 1>::from_raw(1) <= Rate::<u64, 1_000, 1>::from_raw(2));
        assert!(Rate::<u64, 1_000, 1>::from_raw(1) == Rate::<u64, 1_000, 1>::from_raw(1));
        assert!(Rate::<u64, 1_000, 1>::from_raw(1) != Rate::<u64, 1_000, 1>::from_raw(2));

        // Different fraction
        assert!(Rate::<u64, 1_000, 1>::from_raw(11) > Rate::<u64, 10_000, 1>::from_raw(1));
        assert!(Rate::<u64, 1_000, 1>::from_raw(11) >= Rate::<u64, 10_000, 1>::from_raw(1));
        assert!(Rate::<u64, 1_000, 1>::from_raw(10) >= Rate::<u64, 10_000, 1>::from_raw(1));
        assert!(Rate::<u64, 1_000, 1>::from_raw(11) < Rate::<u64, 10_000, 1>::from_raw(2));
        assert!(Rate::<u64, 1_000, 1>::from_raw(1) <= Rate::<u64, 10_000, 1>::from_raw(1));
        assert!(Rate::<u64, 1_000, 1>::from_raw(10) <= Rate::<u64, 10_000, 1>::from_raw(1));
        assert!(Rate::<u64, 1_000, 1>::from_raw(10) == Rate::<u64, 10_000, 1>::from_raw(1));
        assert!(Rate::<u64, 1_000, 1>::from_raw(9) != Rate::<u64, 10_000, 1>::from_raw(2));
    }

    #[test]
    fn rate_compare_u64_u32() {
        // Same fraction
        assert!(Rate::<u64, 1_000, 1>::from_raw(2) > Rate::<u32, 1_000, 1>::from_raw(1));
        assert!(Rate::<u64, 1_000, 1>::from_raw(2) >= Rate::<u32, 1_000, 1>::from_raw(1));
        assert!(Rate::<u64, 1_000, 1>::from_raw(1) >= Rate::<u32, 1_000, 1>::from_raw(1));
        assert!(Rate::<u64, 1_000, 1>::from_raw(1) < Rate::<u32, 1_000, 1>::from_raw(2));
        assert!(Rate::<u64, 1_000, 1>::from_raw(1) <= Rate::<u32, 1_000, 1>::from_raw(1));
        assert!(Rate::<u64, 1_000, 1>::from_raw(1) <= Rate::<u32, 1_000, 1>::from_raw(2));
        assert!(Rate::<u64, 1_000, 1>::from_raw(1) == Rate::<u32, 1_000, 1>::from_raw(1));
        assert!(Rate::<u64, 1_000, 1>::from_raw(1) != Rate::<u32, 1_000, 1>::from_raw(2));

        // Different fraction
        assert!(Rate::<u64, 1_000, 1>::from_raw(11) > Rate::<u32, 10_000, 1>::from_raw(1));
        assert!(Rate::<u64, 1_000, 1>::from_raw(11) >= Rate::<u32, 10_000, 1>::from_raw(1));
        assert!(Rate::<u64, 1_000, 1>::from_raw(10) >= Rate::<u32, 10_000, 1>::from_raw(1));
        assert!(Rate::<u64, 1_000, 1>::from_raw(11) < Rate::<u32, 10_000, 1>::from_raw(2));
        assert!(Rate::<u64, 1_000, 1>::from_raw(1) <= Rate::<u32, 10_000, 1>::from_raw(1));
        assert!(Rate::<u64, 1_000, 1>::from_raw(10) <= Rate::<u32, 10_000, 1>::from_raw(1));
        assert!(Rate::<u64, 1_000, 1>::from_raw(10) == Rate::<u32, 10_000, 1>::from_raw(1));
        assert!(Rate::<u64, 1_000, 1>::from_raw(9) != Rate::<u32, 10_000, 1>::from_raw(2));
    }

    #[test]
    fn rate_compare_u32_u64() {
        // Same fraction
        assert!(Rate::<u32, 1_000, 1>::from_raw(2) > Rate::<u64, 1_000, 1>::from_raw(1));
        assert!(Rate::<u32, 1_000, 1>::from_raw(2) >= Rate::<u64, 1_000, 1>::from_raw(1));
        assert!(Rate::<u32, 1_000, 1>::from_raw(1) >= Rate::<u64, 1_000, 1>::from_raw(1));
        assert!(Rate::<u32, 1_000, 1>::from_raw(1) < Rate::<u64, 1_000, 1>::from_raw(2));
        assert!(Rate::<u32, 1_000, 1>::from_raw(1) <= Rate::<u64, 1_000, 1>::from_raw(1));
        assert!(Rate::<u32, 1_000, 1>::from_raw(1) <= Rate::<u64, 1_000, 1>::from_raw(2));
        assert!(Rate::<u32, 1_000, 1>::from_raw(1) == Rate::<u64, 1_000, 1>::from_raw(1));
        assert!(Rate::<u32, 1_000, 1>::from_raw(1) != Rate::<u64, 1_000, 1>::from_raw(2));

        // Different fraction
        assert!(Rate::<u32, 1_000, 1>::from_raw(11) > Rate::<u64, 10_000, 1>::from_raw(1));
        assert!(Rate::<u32, 1_000, 1>::from_raw(11) >= Rate::<u64, 10_000, 1>::from_raw(1));
        assert!(Rate::<u32, 1_000, 1>::from_raw(10) >= Rate::<u64, 10_000, 1>::from_raw(1));
        assert!(Rate::<u32, 1_000, 1>::from_raw(11) < Rate::<u64, 10_000, 1>::from_raw(2));
        assert!(Rate::<u32, 1_000, 1>::from_raw(1) <= Rate::<u64, 10_000, 1>::from_raw(1));
        assert!(Rate::<u32, 1_000, 1>::from_raw(10) <= Rate::<u64, 10_000, 1>::from_raw(1));
        assert!(Rate::<u32, 1_000, 1>::from_raw(10) == Rate::<u64, 10_000, 1>::from_raw(1));
        assert!(Rate::<u32, 1_000, 1>::from_raw(9) != Rate::<u64, 10_000, 1>::from_raw(2));
    }

    #[test]
    fn rate_rate_math_u32() {
        use crate::RateExtU32;

        // Same base
        let sum: Rate<u32, 1_000, 1> =
            Rate::<u32, 1_000, 1>::from_raw(10) + Rate::<u32, 1_000, 1>::from_raw(1);
        assert_eq!(sum, Rate::<u32, 1_000, 1>::from_raw(11));

        let diff: Rate<u32, 1_000, 1> =
            Rate::<u32, 1_000, 1>::from_raw(10) - Rate::<u32, 1_000, 1>::from_raw(1);
        assert_eq!(diff, Rate::<u32, 1_000, 1>::from_raw(9));

        // Different base
        let sum: Rate<u32, 10_000, 1> =
            Rate::<u32, 10_000, 1>::from_raw(10) + Rate::<u32, 1_000, 1>::from_raw(10).convert();
        assert_eq!(sum, Rate::<u32, 10_000, 1>::from_raw(11));

        let diff: Rate<u32, 10_000, 1> =
            Rate::<u32, 10_000, 1>::from_raw(10) - Rate::<u32, 1_000, 1>::from_raw(10).convert();
        assert_eq!(diff, Rate::<u32, 10_000, 1>::from_raw(9));

        // Short hand vs u32 (should not need `.into()`)
        let sum = Rate::<u32, 1_000, 1>::from_raw(1) + 1.MHz();
        assert_eq!(sum, Rate::<u32, 1_000, 1>::from_raw(1001));

        assert_eq!(
            Rate::<u32, 1_000, 1>::from_raw(5) / Rate::<u32, 100, 1>::from_raw(2),
            25
        );

        assert_eq!(
            Rate::<u32, 100, 1>::from_raw(2) / Rate::<u32, 1_000, 1>::from_raw(5),
            0
        );

        assert_eq!(
            Rate::<u32, 100, 1>::from_raw(500) / Rate::<u32, 1_000, 1>::from_raw(5),
            10
        );
    }

    #[test]
    fn rate_rate_math_u64() {
        use crate::RateExtU64;

        // Same base
        let sum: Rate<u64, 1_000, 1> =
            Rate::<u64, 1_000, 1>::from_raw(10) + Rate::<u64, 1_000, 1>::from_raw(1);
        assert_eq!(sum, Rate::<u64, 1_000, 1>::from_raw(11));

        let diff: Rate<u64, 1_000, 1> =
            Rate::<u64, 1_000, 1>::from_raw(10) - Rate::<u64, 1_000, 1>::from_raw(1);
        assert_eq!(diff, Rate::<u64, 1_000, 1>::from_raw(9));

        // Different base
        let sum: Rate<u64, 10_000, 1> =
            Rate::<u64, 10_000, 1>::from_raw(10) + Rate::<u64, 1_000, 1>::from_raw(10).convert();
        assert_eq!(sum, Rate::<u64, 10_000, 1>::from_raw(11));

        let diff: Rate<u64, 10_000, 1> =
            Rate::<u64, 10_000, 1>::from_raw(10) - Rate::<u64, 1_000, 1>::from_raw(10).convert();
        assert_eq!(diff, Rate::<u64, 10_000, 1>::from_raw(9));

        // Short hand vs u64 (should not need `.into()`)
        let sum = Rate::<u64, 1_000, 1>::from_raw(1) + 1.MHz();
        assert_eq!(sum, Rate::<u64, 1_000, 1>::from_raw(1001));

        assert_eq!(
            Rate::<u64, 1_000, 1>::from_raw(5) / Rate::<u64, 100, 1>::from_raw(2),
            25
        );

        assert_eq!(
            Rate::<u64, 100, 1>::from_raw(2) / Rate::<u64, 1_000, 1>::from_raw(5),
            0
        );

        assert_eq!(
            Rate::<u64, 100, 1>::from_raw(500) / Rate::<u64, 1_000, 1>::from_raw(5),
            10
        );
    }

    #[test]
    fn rate_rate_math_u64_u32() {
        // Same base
        let sum: Rate<u64, 1_000, 1> =
            Rate::<u64, 1_000, 1>::from_raw(10) + Rate::<u32, 1_000, 1>::from_raw(1);
        assert_eq!(sum, Rate::<u64, 1_000, 1>::from_raw(11));

        let diff: Rate<u64, 1_000, 1> =
            Rate::<u64, 1_000, 1>::from_raw(10) - Rate::<u32, 1_000, 1>::from_raw(1);
        assert_eq!(diff, Rate::<u64, 1_000, 1>::from_raw(9));

        // Different base
        let sum: Rate<u64, 10_000, 1> =
            Rate::<u64, 10_000, 1>::from_raw(10) + Rate::<u32, 1_000, 1>::from_raw(10).convert();
        assert_eq!(sum, Rate::<u64, 10_000, 1>::from_raw(11));

        let diff: Rate<u64, 10_000, 1> =
            Rate::<u64, 10_000, 1>::from_raw(10) - Rate::<u32, 1_000, 1>::from_raw(10).convert();
        assert_eq!(diff, Rate::<u64, 10_000, 1>::from_raw(9));
    }

    #[test]
    fn rate_shorthands_u32() {
        use crate::RateExtU32;

        let r: Rate<u32, 1, 1> = 1.Hz();
        assert_eq!(r.to_raw(), 1);

        let r: Rate<u32, 1, 1> = 1.kHz();
        assert_eq!(r.to_raw(), 1_000);

        let r: Rate<u32, 1, 1> = 1.MHz();
        assert_eq!(r.to_raw(), 1_000_000);

        let r = Rate::<u32, 1, 1>::kHz(20);
        assert_eq!(r.to_raw(), 20_000);

        let r = Rate::<u32, 1, 1>::micros(50);
        assert_eq!(r.to_raw(), 20_000);
    }

    #[test]
    fn rate_shorthands_u64() {
        use crate::RateExtU64;

        let r: Rate<u64, 1, 1> = 1.Hz();
        assert_eq!(r.to_raw(), 1);

        let r: Rate<u64, 1, 1> = 1.kHz();
        assert_eq!(r.to_raw(), 1_000);

        let r: Rate<u64, 1, 1> = 1.MHz();
        assert_eq!(r.to_raw(), 1_000_000);

        let r = Rate::<u64, 1, 1>::kHz(20);
        assert_eq!(r.to_raw(), 20_000);

        let r = Rate::<u64, 1, 1>::micros(50);
        assert_eq!(r.to_raw(), 20_000);
    }

    #[test]
    fn rate_duration_conversion() {
        let r = Rate::<u32, 1_000, 1>::from_raw(1);
        let d: Duration<u32, 1, 1_000_000> = r.to_duration();
        assert_eq!(d.as_ticks(), 1_000);
        let d2 = Duration::<u32, 1, 1_000_000>::from_rate(r);
        assert_eq!(d2.as_ticks(), 1_000);

        let r = Rate::<u64, 1_000, 1>::from_raw(1);
        let d: Duration<u64, 1, 1_000_000> = r.to_duration();
        assert_eq!(d.as_ticks(), 1_000);
        let d2 = Duration::<u64, 1, 1_000_000>::from_rate(r);
        assert_eq!(d2.as_ticks(), 1_000);
    }

    #[test]
    fn rate_alias() {
        assert_eq!(
            TimerRate::<u32, 1>::from_raw(1),
            TimerRateU32::<1>::from_raw(1)
        );
        assert_eq!(
            TimerRate::<u64, 1>::from_raw(1),
            TimerRateU64::<1>::from_raw(1)
        );
        assert_eq!(Hertz::<u32>::from_raw(1), TimerRateU32::<1>::from_raw(1));
        assert_eq!(HertzU32::from_raw(1), TimerRateU32::<1>::from_raw(1));
        assert_eq!(HertzU64::from_raw(1), TimerRateU64::<1>::from_raw(1));
        assert_eq!(
            Kilohertz::<u32>::from_raw(1),
            TimerRateU32::<1_000>::from_raw(1)
        );
        assert_eq!(
            KilohertzU32::from_raw(1),
            TimerRateU32::<1_000>::from_raw(1)
        );
        assert_eq!(
            KilohertzU64::from_raw(1),
            TimerRateU64::<1_000>::from_raw(1)
        );
        assert_eq!(
            Megahertz::<u32>::from_raw(1),
            TimerRateU32::<1_000_000>::from_raw(1)
        );
        assert_eq!(
            MegahertzU32::from_raw(1),
            TimerRateU32::<1_000_000>::from_raw(1)
        );
        assert_eq!(
            MegahertzU64::from_raw(1),
            TimerRateU64::<1_000_000>::from_raw(1)
        );
    }

    #[test]
    fn duration_constants() {
        // Test ZERO constant
        let zero_u32 = Duration::<u32, 1, 1_000>::ZERO;
        assert_eq!(zero_u32.as_ticks(), 0);
        assert!(zero_u32.is_zero());

        let zero_u64 = Duration::<u64, 1, 1_000>::ZERO;
        assert_eq!(zero_u64.as_ticks(), 0);
        assert!(zero_u64.is_zero());

        // Test MAX constant
        let max_u32 = Duration::<u32, 1, 1_000>::MAX;
        assert_eq!(max_u32.as_ticks(), u32::MAX);

        let max_u64 = Duration::<u64, 1, 1_000>::MAX;
        assert_eq!(max_u64.as_ticks(), u64::MAX);
    }

    #[test]
    fn duration_checked_mul_div() {
        let d = Duration::<u32, 1, 1_000>::from_ticks(100);

        // checked_mul
        assert_eq!(d.checked_mul(3).unwrap().as_ticks(), 300);
        assert_eq!(d.checked_mul(0).unwrap().as_ticks(), 0);
        assert_eq!(Duration::<u32, 1, 1_000>::MAX.checked_mul(2), None);

        // checked_div
        assert_eq!(d.checked_div(2).unwrap().as_ticks(), 50);
        assert_eq!(d.checked_div(3).unwrap().as_ticks(), 33); // Truncates
        assert_eq!(d.checked_div(0), None);

        // div_ceil
        assert_eq!(d.div_ceil(3).as_ticks(), 34); // Rounds up
        assert_eq!(d.div_ceil(2).as_ticks(), 50); // Exact division
        let d2 = Duration::<u32, 1, 1_000>::from_ticks(30);
        assert_eq!(d2.div_ceil(3).as_ticks(), 10); // Exact division
        let d3 = Duration::<u32, 1, 1_000>::from_ticks(31);
        assert_eq!(d3.div_ceil(3).as_ticks(), 11); // Rounds up
    }

    #[test]
    fn duration_saturating_ops() {
        let d1 = Duration::<u32, 1, 1_000>::from_ticks(100);
        let d2 = Duration::<u32, 1, 1_000>::from_ticks(50);
        let max = Duration::<u32, 1, 1_000>::MAX;

        // saturating_add
        assert_eq!(d1.saturating_add(d2).as_ticks(), 150);
        assert_eq!(max.saturating_add(d1).as_ticks(), u32::MAX);

        // saturating_sub
        assert_eq!(d1.saturating_sub(d2).as_ticks(), 50);
        assert_eq!(d2.saturating_sub(d1).as_ticks(), 0);

        // saturating_mul
        assert_eq!(d1.saturating_mul(3).as_ticks(), 300);
        assert_eq!(max.saturating_mul(2).as_ticks(), u32::MAX);
    }

    #[test]
    fn duration_from_float() {
        // from_secs_f32
        let d = Duration::<u32, 1, 1_000>::from_secs_f32(1.5);
        assert_eq!(d.as_ticks(), 1_500);

        let d = Duration::<u32, 1, 1_000>::from_secs_f32(1.5005);
        assert_eq!(d.as_ticks(), 1_501);

        let d = Duration::<u32, 1, 1_000>::from_secs_f32(1.4994);
        assert_eq!(d.as_ticks(), 1_499);

        // from_secs_f64
        let d = Duration::<u64, 1, 1_000>::from_secs_f64(1.5);
        assert_eq!(d.as_ticks(), 1_500);

        let d = Duration::<u64, 1, 1_000>::from_secs_f64(1.5005);
        assert_eq!(d.as_ticks(), 1_501);

        let d = Duration::<u64, 1, 1_000>::from_secs_f64(1.4994);
        assert_eq!(d.as_ticks(), 1_499);

        // Test round-trip
        let d = Duration::<u32, 1, 1_000>::from_ticks(1_234);
        let f = d.as_secs_f32();
        let d2 = Duration::<u32, 1, 1_000>::from_secs_f32(f);
        assert_eq!(d.as_ticks(), d2.as_ticks());
    }

    #[test]
    fn rate_conversion_rounding() {
        // Issue #50: Rate conversion should round to nearest instead of truncating
        // Test with simple conversions that should round up

        // Convert a rate where the division would have a remainder >= 0.5
        let rate1 = Rate::<u32, 1, 1>::from_raw(100);
        let rate2: Rate<u32, 1, 3> = rate1.const_try_into().unwrap();
        // 100 Hz -> base 1/3: 100 * 3 / 1 = 300, so raw should be 300
        assert_eq!(rate2.to_raw(), 300);

        // Test conversion that needs rounding
        let rate3 = Rate::<u32, 1, 1>::from_raw(5);
        let rate4: Rate<u32, 1, 3> = rate3.const_try_into().unwrap();
        // 5 Hz -> base 1/3: 5 * 3 / 1 = 15
        assert_eq!(rate4.to_raw(), 15);
    }
}
