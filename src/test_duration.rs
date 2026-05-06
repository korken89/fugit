use crate::Duration;
use crate::Instant;

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
    assert!(Duration::<u32, 1, 1_000>::from_ticks(2) > Duration::<u32, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u32, 1, 1_000>::from_ticks(2) >= Duration::<u32, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u32, 1, 1_000>::from_ticks(1) >= Duration::<u32, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u32, 1, 1_000>::from_ticks(1) < Duration::<u32, 1, 1_000>::from_ticks(2));
    assert!(Duration::<u32, 1, 1_000>::from_ticks(1) <= Duration::<u32, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u32, 1, 1_000>::from_ticks(1) <= Duration::<u32, 1, 1_000>::from_ticks(2));
    assert!(Duration::<u32, 1, 1_000>::from_ticks(1) == Duration::<u32, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u32, 1, 1_000>::from_ticks(1) != Duration::<u32, 1, 1_000>::from_ticks(2));

    // Different fraction
    assert!(Duration::<u32, 1, 10_000>::from_ticks(11) > Duration::<u32, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u32, 1, 10_000>::from_ticks(11) >= Duration::<u32, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u32, 1, 10_000>::from_ticks(10) >= Duration::<u32, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u32, 1, 10_000>::from_ticks(11) < Duration::<u32, 1, 1_000>::from_ticks(2));
    assert!(Duration::<u32, 1, 10_000>::from_ticks(1) <= Duration::<u32, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u32, 1, 10_000>::from_ticks(10) <= Duration::<u32, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u32, 1, 10_000>::from_ticks(10) == Duration::<u32, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u32, 1, 10_000>::from_ticks(9) != Duration::<u32, 1, 1_000>::from_ticks(2));

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
    assert!(Duration::<u64, 1, 1_000>::from_ticks(2) > Duration::<u64, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u64, 1, 1_000>::from_ticks(2) >= Duration::<u64, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u64, 1, 1_000>::from_ticks(1) >= Duration::<u64, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u64, 1, 1_000>::from_ticks(1) < Duration::<u64, 1, 1_000>::from_ticks(2));
    assert!(Duration::<u64, 1, 1_000>::from_ticks(1) <= Duration::<u64, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u64, 1, 1_000>::from_ticks(1) <= Duration::<u64, 1, 1_000>::from_ticks(2));
    assert!(Duration::<u64, 1, 1_000>::from_ticks(1) == Duration::<u64, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u64, 1, 1_000>::from_ticks(1) != Duration::<u64, 1, 1_000>::from_ticks(2));

    // Different fraction
    assert!(Duration::<u64, 1, 10_000>::from_ticks(11) > Duration::<u64, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u64, 1, 10_000>::from_ticks(11) >= Duration::<u64, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u64, 1, 10_000>::from_ticks(10) >= Duration::<u64, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u64, 1, 10_000>::from_ticks(11) < Duration::<u64, 1, 1_000>::from_ticks(2));
    assert!(Duration::<u64, 1, 10_000>::from_ticks(1) <= Duration::<u64, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u64, 1, 10_000>::from_ticks(10) <= Duration::<u64, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u64, 1, 10_000>::from_ticks(10) == Duration::<u64, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u64, 1, 10_000>::from_ticks(9) != Duration::<u64, 1, 1_000>::from_ticks(2));

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
    assert!(Duration::<u64, 1, 1_000>::from_ticks(2) > Duration::<u32, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u64, 1, 1_000>::from_ticks(2) >= Duration::<u32, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u64, 1, 1_000>::from_ticks(1) >= Duration::<u32, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u64, 1, 1_000>::from_ticks(1) < Duration::<u32, 1, 1_000>::from_ticks(2));
    assert!(Duration::<u64, 1, 1_000>::from_ticks(1) <= Duration::<u32, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u64, 1, 1_000>::from_ticks(1) <= Duration::<u32, 1, 1_000>::from_ticks(2));
    assert!(Duration::<u64, 1, 1_000>::from_ticks(1) == Duration::<u32, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u64, 1, 1_000>::from_ticks(1) != Duration::<u32, 1, 1_000>::from_ticks(2));

    // Different fraction
    assert!(Duration::<u64, 1, 10_000>::from_ticks(11) > Duration::<u32, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u64, 1, 10_000>::from_ticks(11) >= Duration::<u32, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u64, 1, 10_000>::from_ticks(10) >= Duration::<u32, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u64, 1, 10_000>::from_ticks(11) < Duration::<u32, 1, 1_000>::from_ticks(2));
    assert!(Duration::<u64, 1, 10_000>::from_ticks(1) <= Duration::<u32, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u64, 1, 10_000>::from_ticks(10) <= Duration::<u32, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u64, 1, 10_000>::from_ticks(10) == Duration::<u32, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u64, 1, 10_000>::from_ticks(9) != Duration::<u32, 1, 1_000>::from_ticks(2));

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
    assert!(Duration::<u32, 1, 1_000>::from_ticks(2) > Duration::<u64, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u32, 1, 1_000>::from_ticks(2) >= Duration::<u64, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u32, 1, 1_000>::from_ticks(1) >= Duration::<u64, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u32, 1, 1_000>::from_ticks(1) < Duration::<u64, 1, 1_000>::from_ticks(2));
    assert!(Duration::<u32, 1, 1_000>::from_ticks(1) <= Duration::<u64, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u32, 1, 1_000>::from_ticks(1) <= Duration::<u64, 1, 1_000>::from_ticks(2));
    assert!(Duration::<u32, 1, 1_000>::from_ticks(1) == Duration::<u64, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u32, 1, 1_000>::from_ticks(1) != Duration::<u64, 1, 1_000>::from_ticks(2));

    // Different fraction
    assert!(Duration::<u32, 1, 10_000>::from_ticks(11) > Duration::<u64, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u32, 1, 10_000>::from_ticks(11) >= Duration::<u64, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u32, 1, 10_000>::from_ticks(10) >= Duration::<u64, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u32, 1, 10_000>::from_ticks(11) < Duration::<u64, 1, 1_000>::from_ticks(2));
    assert!(Duration::<u32, 1, 10_000>::from_ticks(1) <= Duration::<u64, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u32, 1, 10_000>::from_ticks(10) <= Duration::<u64, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u32, 1, 10_000>::from_ticks(10) == Duration::<u64, 1, 1_000>::from_ticks(1));
    assert!(Duration::<u32, 1, 10_000>::from_ticks(9) != Duration::<u64, 1, 1_000>::from_ticks(2));

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
    assert_eq!(d.as_ticks(), 1_310_720);

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

    // Rounding
    let d: Duration<u32, 1, 1> = Duration::<u32, 1, 1_000>::from_ticks(500).convert();
    assert_eq!(d.as_ticks(), 1); // Rounds up
    let d: Duration<u32, 1, 1> = Duration::<u32, 1, 1_000>::from_ticks(499).convert();
    assert_eq!(d.as_ticks(), 0); // Rounds down
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

    // Rounding
    let d: Duration<u64, 1, 1> = Duration::<u64, 1, 1_000>::from_ticks(500).convert();
    assert_eq!(d.as_ticks(), 1); // Rounds up
    let d: Duration<u64, 1, 1> = Duration::<u64, 1, 1_000>::from_ticks(499).convert();
    assert_eq!(d.as_ticks(), 0); // Rounds down
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

    let d: Duration<u32, 1, 1_000_000> = 1_000_000_000.picos();
    assert_eq!(d.as_ticks(), 1_000);

    let d: Duration<u32, 1, 1_000_000> = 40_000_000.picos_at_least();
    assert_eq!(d.as_ticks(), 40);

    let d: Duration<u32, 1, 1_000_000> = 40_000_075.picos_at_least();
    assert_eq!(d.as_ticks(), 41);

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

    let d: Duration<u64, 1, 10_000> = 10_000_000_000.picos();
    assert_eq!(d.as_ticks(), 100);

    let d: Duration<u64, 1, 1_000_000> = 40_000_000.picos_at_least();
    assert_eq!(d.as_ticks(), 40);

    let d: Duration<u64, 1, 1_000_000> = 40_000_075.picos_at_least();
    assert_eq!(d.as_ticks(), 41);

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
fn duration_picosecond_support() {
    // This test verifies that u64 const generics enable picosecond precision
    // Picoseconds: 1 second = 1_000_000_000_000 picoseconds
    // This value exceeds u32::MAX (4_294_967_295), so it requires u64

    type PicosDurationU64 = Duration<u64, 1, 1_000_000_000_000>;

    // Create a duration of 1 second in picoseconds
    let one_sec = PicosDurationU64::from_ticks(1_000_000_000_000);
    assert_eq!(one_sec.as_ticks(), 1_000_000_000_000);

    // Test conversion from milliseconds to picoseconds
    let one_ms = Duration::<u64, 1, 1_000>::from_ticks(1);
    let one_ms_in_picos: PicosDurationU64 = one_ms.convert();
    assert_eq!(one_ms_in_picos.as_ticks(), 1_000_000_000);

    // Test conversion from microseconds to picoseconds
    let one_us = Duration::<u64, 1, 1_000_000>::from_ticks(1);
    let one_us_in_picos: PicosDurationU64 = one_us.convert();
    assert_eq!(one_us_in_picos.as_ticks(), 1_000_000);

    // Test conversion from nanoseconds to picoseconds
    let one_ns = Duration::<u64, 1, 1_000_000_000>::from_ticks(1);
    let one_ns_in_picos: PicosDurationU64 = one_ns.convert();
    assert_eq!(one_ns_in_picos.as_ticks(), 1_000);

    // Test arithmetic with picosecond durations
    let picos_1 = PicosDurationU64::from_ticks(5_000);
    let picos_2 = PicosDurationU64::from_ticks(3_000);
    let sum = picos_1 + picos_2;
    assert_eq!(sum.as_ticks(), 8_000);

    // Test conversion from picoseconds to seconds
    let picos = PicosDurationU64::from_ticks(2_500_000_000_000);
    let secs: Duration<u64, 1, 1> = picos.convert();
    assert_eq!(secs.as_ticks(), 3); // 2.5 seconds rounds to 3
}

#[test]
fn duration_rem_u32() {
    // Same base - Duration % Duration
    let d1 = Duration::<u32, 1, 1_000>::from_ticks(10);
    let d2 = Duration::<u32, 1, 1_000>::from_ticks(3);
    assert_eq!(d1 % d2, Duration::<u32, 1, 1_000>::from_ticks(1));

    // RemAssign Duration
    let mut d1 = Duration::<u32, 1, 1_000>::from_ticks(10);
    d1 %= Duration::<u32, 1, 1_000>::from_ticks(3);
    assert_eq!(d1, Duration::<u32, 1, 1_000>::from_ticks(1));

    // Test checking if a duration is a multiple of another
    use crate::ExtU32;
    let d1: Duration<u32, 1, 1_000> = 100.millis();
    let d2: Duration<u32, 1, 1_000> = 25.millis();
    assert_eq!(d1 % d2, Duration::<u32, 1, 1_000>::from_ticks(0)); // 100ms is a multiple of 25ms

    let d1: Duration<u32, 1, 1_000> = 100.millis();
    let d2: Duration<u32, 1, 1_000> = 30.millis();
    assert_eq!(d1 % d2, Duration::<u32, 1, 1_000>::from_ticks(10)); // 100ms mod 30ms = 10ms
}

#[test]
fn duration_rem_u64() {
    // Same base - Duration % Duration
    let d1 = Duration::<u64, 1, 1_000>::from_ticks(10);
    let d2 = Duration::<u64, 1, 1_000>::from_ticks(3);
    assert_eq!(d1 % d2, Duration::<u64, 1, 1_000>::from_ticks(1));

    // RemAssign Duration
    let mut d1 = Duration::<u64, 1, 1_000>::from_ticks(10);
    d1 %= Duration::<u64, 1, 1_000>::from_ticks(3);
    assert_eq!(d1, Duration::<u64, 1, 1_000>::from_ticks(1));
}

#[test]
fn duration_checked_rem() {
    // Successful checked_rem
    let d1 = Duration::<u32, 1, 1_000>::from_ticks(10);
    let d2 = Duration::<u32, 1, 1_000>::from_ticks(3);
    assert_eq!(
        d1.checked_rem(d2),
        Some(Duration::<u32, 1, 1_000>::from_ticks(1))
    );

    // Division by zero should return None
    let d1 = Duration::<u32, 1, 1_000>::from_ticks(10);
    let d2 = Duration::<u32, 1, 1_000>::from_ticks(0);
    assert_eq!(d1.checked_rem(d2), None);

    // Different base
    let d1 = Duration::<u32, 1, 1_000>::from_ticks(350); // 350 ms
    let d2 = Duration::<u32, 1, 10_000>::from_ticks(1000); // 100 ms
    assert_eq!(
        d1.checked_rem(d2),
        Some(Duration::<u32, 1, 1_000>::from_ticks(50)) // 350 ms % 100 ms = 50 ms
    );
}

#[test]
fn duration_from_core_duration() {
    use core::convert::TryFrom;

    // Converting to milliseconds (u32).
    let std_duration = core::time::Duration::new(2, 500_000_000); // 2.5 seconds
    let fugit_duration: Duration<u32, 1, 1_000> = Duration::try_from(std_duration).unwrap();
    assert_eq!(fugit_duration.as_ticks(), 2_500);

    // Converting to microseconds (u64).
    let std_duration = core::time::Duration::new(1, 234_567_000); // 1.234567 seconds
    let fugit_duration: Duration<u64, 1, 1_000_000> = Duration::try_from(std_duration).unwrap();
    assert_eq!(fugit_duration.as_ticks(), 1_234_567);

    // Converting to seconds (u32).
    let std_duration = core::time::Duration::new(42, 0);
    let fugit_duration: Duration<u32, 1, 1> = Duration::try_from(std_duration).unwrap();
    assert_eq!(fugit_duration.as_ticks(), 42);

    // Rounding test - should round to nearest.
    let std_duration = core::time::Duration::new(0, 1_500_000); // 1.5 ms = 1_500_000 ns
    let fugit_duration: Duration<u32, 1, 1_000> = Duration::try_from(std_duration).unwrap();
    assert_eq!(fugit_duration.as_ticks(), 2); // Should round to 2ms

    // Another rounding test.
    let std_duration = core::time::Duration::new(0, 1_499_999); // ~1.5 ms
    let fugit_duration: Duration<u32, 1, 1_000> = Duration::try_from(std_duration).unwrap();
    assert_eq!(fugit_duration.as_ticks(), 1); // Should round to 1ms

    // Zero duration.
    let std_duration = core::time::Duration::new(0, 0);
    let fugit_duration: Duration<u64, 1, 1_000> = Duration::try_from(std_duration).unwrap();
    assert_eq!(fugit_duration.as_ticks(), 0);
    assert!(fugit_duration.is_zero());

    // Nanoseconds to nanoseconds (should be exact).
    let std_duration = core::time::Duration::new(1, 500_000_000); // 1.5 seconds
    let fugit_duration: Duration<u64, 1, 1_000_000_000> = Duration::try_from(std_duration).unwrap();
    assert_eq!(fugit_duration.as_ticks(), 1_500_000_000);

    // Overflow test - should return None when converting large u64 duration to u32.
    let std_duration = core::time::Duration::new(u64::MAX / 1_000, 0); // Very large duration
    let result: Result<Duration<u32, 1, 1_000>, ()> = Duration::try_from(std_duration);
    assert!(result.is_err());

    // Conversion with both seconds and nanoseconds.
    let std_duration = core::time::Duration::new(5, 123_456_789); // 5.123456789 seconds
    let fugit_duration: Duration<u64, 1, 1_000_000_000> = Duration::try_from(std_duration).unwrap();
    assert_eq!(fugit_duration.as_ticks(), 5_123_456_789);
}

#[test]
fn duration_into_core_duration() {
    use core::convert::From;

    // Converting from milliseconds (u32).
    let fugit_duration = Duration::<u32, 1, 1_000>::from_ticks(2_500);
    let std_duration = core::time::Duration::from(fugit_duration);
    assert_eq!(std_duration.as_secs(), 2);
    assert_eq!(std_duration.subsec_nanos(), 500_000_000);

    // Converting from microseconds (u64).
    let fugit_duration = Duration::<u64, 1, 1_000_000>::from_ticks(1_234_567);
    let std_duration = core::time::Duration::from(fugit_duration);
    assert_eq!(std_duration.as_secs(), 1);
    assert_eq!(std_duration.subsec_nanos(), 234_567_000);

    // Converting from seconds (u32).
    let fugit_duration = Duration::<u32, 1, 1>::from_ticks(42);
    let std_duration = core::time::Duration::from(fugit_duration);
    assert_eq!(std_duration.as_secs(), 42);
    assert_eq!(std_duration.subsec_nanos(), 0);

    // Zero duration.
    let fugit_duration = Duration::<u64, 1, 1_000>::from_ticks(0);
    let std_duration = core::time::Duration::from(fugit_duration);
    assert_eq!(std_duration.as_secs(), 0);
    assert_eq!(std_duration.subsec_nanos(), 0);

    // Nanoseconds to nanoseconds.
    let fugit_duration = Duration::<u64, 1, 1_000_000_000>::from_ticks(1_500_000_000);
    let std_duration = core::time::Duration::from(fugit_duration);
    assert_eq!(std_duration.as_secs(), 1);
    assert_eq!(std_duration.subsec_nanos(), 500_000_000);

    // Conversion with fractional part.
    let fugit_duration = Duration::<u64, 1, 1_000_000_000>::from_ticks(5_123_456_789);
    let std_duration = core::time::Duration::from(fugit_duration);
    assert_eq!(std_duration.as_secs(), 5);
    assert_eq!(std_duration.subsec_nanos(), 123_456_789);

    // Round-trip test.
    let original = core::time::Duration::new(10, 250_000_000);
    let fugit = Duration::<u64, 1, 1_000>::try_from(original).unwrap();
    let back = core::time::Duration::from(fugit);
    assert_eq!(back.as_secs(), 10);
    assert_eq!(back.subsec_nanos(), 250_000_000);
}

#[test]
fn duration_const_try_from_no_rounding_overflow_u64() {
    // Regression: previously the rounding step `lh + LD_TIMES_RN/2` overflowed
    // u64 silently when ticks were close to u64::MAX, producing Some(garbage)
    // instead of either the correct value or None.

    // u64::MAX milliseconds to seconds. lh = u64::MAX, divisor = 1000,
    // remainder 615 so it rounds up.
    let big_ms = Duration::<u64, 1, 1_000>::from_ticks(u64::MAX);
    let secs: Option<Duration<u64, 1, 1>> = big_ms.const_try_into();
    assert_eq!(secs.unwrap().as_ticks(), u64::MAX / 1000 + 1);

    // u64::MAX nanoseconds to microseconds, same divisor.
    let big_ns = Duration::<u64, 1, 1_000_000_000>::from_ticks(u64::MAX);
    let us: Option<Duration<u64, 1, 1_000_000>> = big_ns.const_try_into();
    assert_eq!(us.unwrap().as_ticks(), u64::MAX / 1000 + 1);

    // u64::MAX nanoseconds to seconds. divisor = 1_000_000_000.
    let big_ns = Duration::<u64, 1, 1_000_000_000>::from_ticks(u64::MAX);
    let s: Option<Duration<u64, 1, 1>> = big_ns.const_try_into();
    // u64::MAX / 1_000_000_000 = 18_446_744_073, remainder 709_551_615
    // (above half), so result rounds up by one.
    assert_eq!(s.unwrap().as_ticks(), u64::MAX / 1_000_000_000 + 1);

    // Half-down case: r strictly below half should not round up.
    let d = 1_000u64;
    let q = (u64::MAX / d) - 1;
    let r = d / 2 - 1;
    let ticks = q * d + r;
    let dur = Duration::<u64, 1, 1_000>::from_ticks(ticks);
    let secs: Option<Duration<u64, 1, 1>> = dur.const_try_into();
    assert_eq!(secs.unwrap().as_ticks(), q);

    // Half-up case: r = d/2 should round up.
    let r = d / 2;
    let ticks = q * d + r;
    let dur = Duration::<u64, 1, 1_000>::from_ticks(ticks);
    let secs: Option<Duration<u64, 1, 1>> = dur.const_try_into();
    assert_eq!(secs.unwrap().as_ticks(), q + 1);
}

#[test]
fn duration_const_try_from_returns_none_on_target_overflow() {
    // When the rounded result genuinely doesn't fit the target type, the
    // function must return None (it must not silently truncate).

    // u32::MAX ms to seconds: u32::MAX % 1000 = 295 (below half), no round-up.
    let many_ms = Duration::<u32, 1, 1_000>::from_ticks(u32::MAX);
    let secs: Option<Duration<u32, 1, 1>> = many_ms.const_try_into();
    assert_eq!(secs.unwrap().as_ticks(), u32::MAX / 1000);

    // u64::MAX ms to seconds in u32: clearly doesn't fit, must be None.
    // (Conversion from u64 Duration into u32 Duration via try_from chain.)
    let big = Duration::<u64, 1, 1_000>::from_ticks(u64::MAX);
    let as_u64_s: Duration<u64, 1, 1> = big.convert();
    let as_u32: Result<Duration<u32, 1, 1>, _> = as_u64_s.try_into();
    assert!(as_u32.is_err());
}

#[test]
#[should_panic(expected = "Duration::as_picos: multiplication overflowed storage type")]
fn duration_as_unit_panics_on_runtime_overflow_u32() {
    // Regression: shorthand methods used to silently wrap on overflow.
    // u32::MAX microseconds as picoseconds = u32::MAX * 1_000_000, overflows u32.
    let d = Duration::<u32, 1, 1_000_000>::from_ticks(u32::MAX);
    let _ = d.as_picos();
}

#[test]
#[should_panic(expected = "Duration::from_minutes: multiplication overflowed storage type")]
fn duration_from_unit_panics_on_runtime_overflow_u32() {
    // u32::MAX minutes as a seconds-based Duration: val * 60 overflows u32.
    let _ = Duration::<u32, 1, 1>::from_minutes(u32::MAX);
}

#[test]
fn duration_shorthand_works_for_valid_u32() {
    // Make sure the const_checked path doesn't break valid uses.
    let d = Duration::<u32, 1, 1_000>::from_ticks(5_000);
    assert_eq!(d.as_secs(), 5);
    assert_eq!(d.as_millis(), 5_000);

    let d = Duration::<u32, 1, 1>::from_ticks(120);
    assert_eq!(d.as_minutes(), 2);
}
