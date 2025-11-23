////////////////////////////////////////////////////////////////////////////////
//
// Instant tests
//
////////////////////////////////////////////////////////////////////////////////

use crate::{Duration, Instant};

#[test]
fn instant_compare_u32() {
    // Wrapping
    assert!(
        Instant::<u32, 1, 1_000>::from_ticks(1) > Instant::<u32, 1, 1_000>::from_ticks(u32::MAX)
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
        Instant::<u64, 1, 1_000>::from_ticks(1) > Instant::<u64, 1, 1_000>::from_ticks(u64::MAX)
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
