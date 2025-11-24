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
    assert_eq!(secs.as_ticks(), 2); // 2.5 seconds truncates to 2
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
