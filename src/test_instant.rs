////////////////////////////////////////////////////////////////////////////////
//
// Instant tests
//
////////////////////////////////////////////////////////////////////////////////

use crate::{Duration, MonotonicInstant, WrappingInstant};

#[test]
fn instant_compare_u32() {
    // Wrapping
    assert!(
        WrappingInstant::<u32, 1, 1_000>::from_ticks(1)
            > WrappingInstant::<u32, 1, 1_000>::from_ticks(u32::MAX)
    );
    assert!(
        WrappingInstant::<u32, 1, 1_000>::from_ticks(u32::MAX - 1)
            < WrappingInstant::<u32, 1, 1_000>::from_ticks(u32::MAX)
    );

    // Non-wrapping
    assert!(
        WrappingInstant::<u32, 1, 1_000>::from_ticks(2)
            > WrappingInstant::<u32, 1, 1_000>::from_ticks(1)
    );
    assert!(
        WrappingInstant::<u32, 1, 1_000>::from_ticks(2)
            >= WrappingInstant::<u32, 1, 1_000>::from_ticks(1)
    );
    assert!(
        WrappingInstant::<u32, 1, 1_000>::from_ticks(1)
            >= WrappingInstant::<u32, 1, 1_000>::from_ticks(1)
    );
    assert!(
        WrappingInstant::<u32, 1, 1_000>::from_ticks(1)
            < WrappingInstant::<u32, 1, 1_000>::from_ticks(2)
    );
    assert!(
        WrappingInstant::<u32, 1, 1_000>::from_ticks(1)
            <= WrappingInstant::<u32, 1, 1_000>::from_ticks(1)
    );
    assert!(
        WrappingInstant::<u32, 1, 1_000>::from_ticks(1)
            <= WrappingInstant::<u32, 1, 1_000>::from_ticks(2)
    );
    assert!(
        WrappingInstant::<u32, 1, 1_000>::from_ticks(1)
            == WrappingInstant::<u32, 1, 1_000>::from_ticks(1)
    );
    assert!(
        WrappingInstant::<u32, 1, 1_000>::from_ticks(1)
            != WrappingInstant::<u32, 1, 1_000>::from_ticks(2)
    );

    // Checked duration since non-wrapping
    let one = WrappingInstant::<u32, 1, 1_000>::from_ticks(1);
    let two = WrappingInstant::<u32, 1, 1_000>::from_ticks(2);
    let three = WrappingInstant::<u32, 1, 1_000>::from_ticks(3);

    assert_eq!(
        one.checked_duration_since(one),
        Some(Duration::<u32, 1, 1_000>::from_ticks(0))
    );
    assert_eq!(
        two.checked_duration_since(one),
        Some(Duration::<u32, 1, 1_000>::from_ticks(1))
    );
    assert_eq!(two.checked_duration_since(three), None);

    // Checked duration since wrapping
    let max = WrappingInstant::<u32, 1, 1_000>::from_ticks(u32::MAX);
    let max_minus_one = WrappingInstant::<u32, 1, 1_000>::from_ticks(u32::MAX - 1);

    assert_eq!(
        two.checked_duration_since(max),
        Some(Duration::<u32, 1, 1_000>::from_ticks(3))
    );
    assert_eq!(
        two.checked_duration_since(max_minus_one),
        Some(Duration::<u32, 1, 1_000>::from_ticks(4))
    );
}

#[test]
fn instant_compare_u64() {
    // Wrapping
    assert!(
        WrappingInstant::<u64, 1, 1_000>::from_ticks(1)
            > WrappingInstant::<u64, 1, 1_000>::from_ticks(u64::MAX)
    );
    assert!(
        WrappingInstant::<u64, 1, 1_000>::from_ticks(u64::MAX - 1)
            < WrappingInstant::<u64, 1, 1_000>::from_ticks(u64::MAX)
    );

    // Non-wrapping
    assert!(
        WrappingInstant::<u64, 1, 1_000>::from_ticks(2)
            > WrappingInstant::<u64, 1, 1_000>::from_ticks(1)
    );
    assert!(
        WrappingInstant::<u64, 1, 1_000>::from_ticks(2)
            >= WrappingInstant::<u64, 1, 1_000>::from_ticks(1)
    );
    assert!(
        WrappingInstant::<u64, 1, 1_000>::from_ticks(1)
            >= WrappingInstant::<u64, 1, 1_000>::from_ticks(1)
    );
    assert!(
        WrappingInstant::<u64, 1, 1_000>::from_ticks(1)
            < WrappingInstant::<u64, 1, 1_000>::from_ticks(2)
    );
    assert!(
        WrappingInstant::<u64, 1, 1_000>::from_ticks(1)
            <= WrappingInstant::<u64, 1, 1_000>::from_ticks(1)
    );
    assert!(
        WrappingInstant::<u64, 1, 1_000>::from_ticks(1)
            <= WrappingInstant::<u64, 1, 1_000>::from_ticks(2)
    );
    assert!(
        WrappingInstant::<u64, 1, 1_000>::from_ticks(1)
            == WrappingInstant::<u64, 1, 1_000>::from_ticks(1)
    );
    assert!(
        WrappingInstant::<u64, 1, 1_000>::from_ticks(1)
            != WrappingInstant::<u64, 1, 1_000>::from_ticks(2)
    );

    // Checked duration since non-wrapping
    let one = WrappingInstant::<u64, 1, 1_000>::from_ticks(1);
    let two = WrappingInstant::<u64, 1, 1_000>::from_ticks(2);
    let three = WrappingInstant::<u64, 1, 1_000>::from_ticks(3);

    assert_eq!(
        one.checked_duration_since(one),
        Some(Duration::<u64, 1, 1_000>::from_ticks(0))
    );
    assert_eq!(
        two.checked_duration_since(one),
        Some(Duration::<u64, 1, 1_000>::from_ticks(1))
    );
    assert_eq!(two.checked_duration_since(three), None);

    // Checked duration since wrapping
    let max = WrappingInstant::<u64, 1, 1_000>::from_ticks(u64::MAX);
    let max_minus_one = WrappingInstant::<u64, 1, 1_000>::from_ticks(u64::MAX - 1);

    assert_eq!(
        two.checked_duration_since(max),
        Some(Duration::<u64, 1, 1_000>::from_ticks(3))
    );
    assert_eq!(
        two.checked_duration_since(max_minus_one),
        Some(Duration::<u64, 1, 1_000>::from_ticks(4))
    );
}

#[test]
fn instant_compare_half_range_u32() {
    use core::cmp::Ordering;

    const HALF: u32 = 1 << 31;

    let zero = WrappingInstant::<u32, 1, 1_000>::from_ticks(0);
    let one = WrappingInstant::<u32, 1, 1_000>::from_ticks(1);
    let half = WrappingInstant::<u32, 1, 1_000>::from_ticks(HALF);
    let half_plus_one = WrappingInstant::<u32, 1, 1_000>::from_ticks(HALF + 1);
    let one_plus_half = WrappingInstant::<u32, 1, 1_000>::from_ticks(1 + HALF);

    // The largest unambiguous difference. Used to report `Equal` while the ticks differ.
    assert_eq!(half.const_partial_cmp(one), Some(Ordering::Greater));
    assert_eq!(one.const_partial_cmp(half), Some(Ordering::Less));
    assert!(half != one);

    // Exactly half the range apart: the difference is `HALF` in both directions, so neither
    // comes first and no comparison holds.
    assert_eq!(one.const_partial_cmp(one_plus_half), None);
    assert_eq!(one_plus_half.const_partial_cmp(one), None);
    for holds in [
        one < one_plus_half,
        one > one_plus_half,
        one <= one_plus_half,
        one >= one_plus_half,
        one == one_plus_half,
    ] {
        assert!(!holds);
    }

    // Just past half the range, the comparison flips.
    assert_eq!(half_plus_one.const_partial_cmp(zero), Some(Ordering::Less));
    assert_eq!(
        zero.const_partial_cmp(half_plus_one),
        Some(Ordering::Greater)
    );

    // `checked_duration_since` is unaffected by the fix.
    assert_eq!(
        half.checked_duration_since(one),
        Some(Duration::<u32, 1, 1_000>::from_ticks(HALF - 1))
    );
    assert_eq!(one.checked_duration_since(one_plus_half), None);
    assert_eq!(one_plus_half.checked_duration_since(one), None);
}

#[test]
fn instant_compare_half_range_u64() {
    use core::cmp::Ordering;

    const HALF: u64 = 1 << 63;

    let zero = WrappingInstant::<u64, 1, 1_000>::from_ticks(0);
    let one = WrappingInstant::<u64, 1, 1_000>::from_ticks(1);
    let half = WrappingInstant::<u64, 1, 1_000>::from_ticks(HALF);
    let half_plus_one = WrappingInstant::<u64, 1, 1_000>::from_ticks(HALF + 1);
    let one_plus_half = WrappingInstant::<u64, 1, 1_000>::from_ticks(1 + HALF);

    assert_eq!(half.const_partial_cmp(one), Some(Ordering::Greater));
    assert_eq!(one.const_partial_cmp(half), Some(Ordering::Less));
    assert!(half != one);

    assert_eq!(one.const_partial_cmp(one_plus_half), None);
    assert_eq!(one_plus_half.const_partial_cmp(one), None);
    for holds in [
        one < one_plus_half,
        one > one_plus_half,
        one <= one_plus_half,
        one >= one_plus_half,
        one == one_plus_half,
    ] {
        assert!(!holds);
    }

    assert_eq!(half_plus_one.const_partial_cmp(zero), Some(Ordering::Less));
    assert_eq!(
        zero.const_partial_cmp(half_plus_one),
        Some(Ordering::Greater)
    );

    assert_eq!(
        half.checked_duration_since(one),
        Some(Duration::<u64, 1, 1_000>::from_ticks(HALF - 1))
    );
    assert_eq!(one.checked_duration_since(one_plus_half), None);
    assert_eq!(one_plus_half.checked_duration_since(one), None);
}

#[test]
fn instant_compare_duality_u32() {
    use core::cmp::Ordering;

    const HALF: u32 = 1 << 31;
    let base = 0x1234_5678;

    // `a < b` must hold exactly when `b > a`, and `Equal` must mean equal ticks.
    for offset in [
        0,
        1,
        1_000,
        HALF - 1,
        HALF,
        HALF + 1,
        u32::MAX - 1,
        u32::MAX,
    ] {
        let a = WrappingInstant::<u32, 1, 1_000>::from_ticks(base);
        let b = WrappingInstant::<u32, 1, 1_000>::from_ticks(base.wrapping_add(offset));

        assert_eq!(
            a.const_partial_cmp(b).map(Ordering::reverse),
            b.const_partial_cmp(a),
            "offset {offset:#x}"
        );
        assert_eq!(
            a.const_partial_cmp(b) == Some(Ordering::Equal),
            a == b,
            "offset {offset:#x}"
        );
    }
}

#[test]
fn instant_compare_is_not_transitive_u32() {
    // The wrapping compare is not transitive, which is why `Instant` is deliberately not
    // `Ord`: `BTreeMap`, `sort` and friends silently misbehave on such an ordering.
    let a = WrappingInstant::<u32, 1, 1_000>::from_ticks(0);
    let b = WrappingInstant::<u32, 1, 1_000>::from_ticks(0x6000_0000);
    let c = WrappingInstant::<u32, 1, 1_000>::from_ticks(0xC000_0000);

    assert!(a < b);
    assert!(b < c);
    assert!(a > c);
}

#[test]
fn instant_compare_is_not_transitive_u64() {
    let a = WrappingInstant::<u64, 1, 1_000>::from_ticks(0);
    let b = WrappingInstant::<u64, 1, 1_000>::from_ticks(0x6000_0000_0000_0000);
    let c = WrappingInstant::<u64, 1, 1_000>::from_ticks(0xC000_0000_0000_0000);

    assert!(a < b);
    assert!(b < c);
    assert!(a > c);
}

#[test]
fn instant_largest_fitting_conversion_constants_u32() {
    // The conversion constants are `u64` and get cast to the storage type, so bases whose
    // ratio exceeds it are rejected at compile time. Right below that boundary the
    // conversion must still work: a ratio of exactly `u32::MAX` is the largest legal one.
    const MAX_RATIO: u64 = u32::MAX as u64;

    let i = WrappingInstant::<u32, 1, 1>::from_ticks(1);
    let d = Duration::<u32, 1, MAX_RATIO>::from_ticks(MAX_RATIO as u32);

    // MAX_RATIO ticks of 1/MAX_RATIO s is exactly one second.
    assert_eq!(
        i.convert_add_duration(d),
        Some(WrappingInstant::<u32, 1, 1>::from_ticks(2))
    );
    assert_eq!(
        i.convert_sub_duration(d),
        Some(WrappingInstant::<u32, 1, 1>::from_ticks(0))
    );

    // Sub-second remainders truncate toward zero rather than trapping.
    assert_eq!(
        i.convert_add_duration(Duration::<u32, 1, MAX_RATIO>::from_ticks(1)),
        Some(WrappingInstant::<u32, 1, 1>::from_ticks(1))
    );
}

#[test]
fn instant_duration_math_u32() {
    use crate::ExtU32;

    // Instant - Instant, Same base
    let diff: Duration<u32, 1, 1_000> = WrappingInstant::<u32, 1, 1_000>::from_ticks(10)
        - WrappingInstant::<u32, 1, 1_000>::from_ticks(1);
    assert_eq!(diff, Duration::<u32, 1, 1_000>::from_ticks(9));

    // Instant +- Duration, Same base
    let sum: WrappingInstant<u32, 1, 1_000> =
        WrappingInstant::<u32, 1, 1_000>::from_ticks(10) + Duration::<u32, 1, 1_000>::from_ticks(1);
    assert_eq!(sum, WrappingInstant::<u32, 1, 1_000>::from_ticks(11));

    let mut sum = WrappingInstant::<u32, 1, 1_000>::from_ticks(10);
    sum += Duration::<u32, 1, 1_000>::from_ticks(1);
    assert_eq!(sum, WrappingInstant::<u32, 1, 1_000>::from_ticks(11));

    let diff: WrappingInstant<u32, 1, 1_000> =
        WrappingInstant::<u32, 1, 1_000>::from_ticks(10) - Duration::<u32, 1, 1_000>::from_ticks(1);
    assert_eq!(diff, WrappingInstant::<u32, 1, 1_000>::from_ticks(9));

    let mut diff = WrappingInstant::<u32, 1, 1_000>::from_ticks(10);
    diff -= Duration::<u32, 1, 1_000>::from_ticks(1);
    assert_eq!(diff, WrappingInstant::<u32, 1, 1_000>::from_ticks(9));

    // Instant +- Duration, Different base
    let sum: WrappingInstant<u32, 1, 10_000> = WrappingInstant::<u32, 1, 10_000>::from_ticks(10)
        + Duration::<u32, 1, 1_000>::from_ticks(1).convert();
    assert_eq!(sum, WrappingInstant::<u32, 1, 10_000>::from_ticks(20));

    let mut sum = WrappingInstant::<u32, 1, 10_000>::from_ticks(10);
    sum += Duration::<u32, 1, 1_000>::from_ticks(1).convert();
    assert_eq!(sum, WrappingInstant::<u32, 1, 10_000>::from_ticks(20));

    let diff: WrappingInstant<u32, 1, 10_000> = WrappingInstant::<u32, 1, 10_000>::from_ticks(10)
        - Duration::<u32, 1, 1_000>::from_ticks(1).convert();
    assert_eq!(diff, WrappingInstant::<u32, 1, 10_000>::from_ticks(0));

    let mut diff = WrappingInstant::<u32, 1, 10_000>::from_ticks(10);
    diff -= Duration::<u32, 1, 1_000>::from_ticks(1).convert();
    assert_eq!(diff, WrappingInstant::<u32, 1, 10_000>::from_ticks(0));

    // Instant + Extension trait
    let sum: WrappingInstant<u32, 1, 10_000> =
        WrappingInstant::<u32, 1, 10_000>::from_ticks(10) + 1.millis();
    assert_eq!(sum, WrappingInstant::<u32, 1, 10_000>::from_ticks(20));

    // Instant - Extension trait
    let diff: WrappingInstant<u32, 1, 10_000> =
        WrappingInstant::<u32, 1, 10_000>::from_ticks(10) - 1.millis();
    assert_eq!(diff, WrappingInstant::<u32, 1, 10_000>::from_ticks(0));
}

#[test]
fn instant_duration_math_u64() {
    use crate::ExtU64;

    // Instant - Instant, Same base
    let diff: Duration<u64, 1, 1_000> = WrappingInstant::<u64, 1, 1_000>::from_ticks(10)
        - WrappingInstant::<u64, 1, 1_000>::from_ticks(1);
    assert_eq!(diff, Duration::<u64, 1, 1_000>::from_ticks(9));

    // Instant +- Duration, Same base
    let sum: WrappingInstant<u64, 1, 1_000> =
        WrappingInstant::<u64, 1, 1_000>::from_ticks(10) + Duration::<u64, 1, 1_000>::from_ticks(1);
    assert_eq!(sum, WrappingInstant::<u64, 1, 1_000>::from_ticks(11));

    let mut sum = WrappingInstant::<u64, 1, 1_000>::from_ticks(10);
    sum += Duration::<u64, 1, 1_000>::from_ticks(1);
    assert_eq!(sum, WrappingInstant::<u64, 1, 1_000>::from_ticks(11));

    let diff: WrappingInstant<u64, 1, 1_000> =
        WrappingInstant::<u64, 1, 1_000>::from_ticks(10) - Duration::<u64, 1, 1_000>::from_ticks(1);
    assert_eq!(diff, WrappingInstant::<u64, 1, 1_000>::from_ticks(9));

    let mut diff = WrappingInstant::<u64, 1, 1_000>::from_ticks(10);
    diff -= Duration::<u64, 1, 1_000>::from_ticks(1);
    assert_eq!(diff, WrappingInstant::<u64, 1, 1_000>::from_ticks(9));

    // Instant +- Duration, Different base
    let sum: WrappingInstant<u64, 1, 10_000> = WrappingInstant::<u64, 1, 10_000>::from_ticks(10)
        + Duration::<u64, 1, 1_000>::from_ticks(1).convert();
    assert_eq!(sum, WrappingInstant::<u64, 1, 10_000>::from_ticks(20));

    let mut sum = WrappingInstant::<u64, 1, 10_000>::from_ticks(10);
    sum += Duration::<u64, 1, 1_000>::from_ticks(1).convert();
    assert_eq!(sum, WrappingInstant::<u64, 1, 10_000>::from_ticks(20));

    let diff: WrappingInstant<u64, 1, 10_000> = WrappingInstant::<u64, 1, 10_000>::from_ticks(10)
        - Duration::<u64, 1, 1_000>::from_ticks(1).convert();
    assert_eq!(diff, WrappingInstant::<u64, 1, 10_000>::from_ticks(0));

    let mut diff = WrappingInstant::<u64, 1, 10_000>::from_ticks(10);
    diff -= Duration::<u64, 1, 1_000>::from_ticks(1).convert();
    assert_eq!(diff, WrappingInstant::<u64, 1, 10_000>::from_ticks(0));

    // Instant + Extension trait
    let sum: WrappingInstant<u64, 1, 10_000> =
        WrappingInstant::<u64, 1, 10_000>::from_ticks(10) + 1.millis();
    assert_eq!(sum, WrappingInstant::<u64, 1, 10_000>::from_ticks(20));

    // Instant - Extension trait
    let diff: WrappingInstant<u64, 1, 10_000> =
        WrappingInstant::<u64, 1, 10_000>::from_ticks(10) - 1.millis();
    assert_eq!(diff, WrappingInstant::<u64, 1, 10_000>::from_ticks(0));
}

#[test]
fn instant_duration_math_u64_u32() {
    // Instant +- Duration, Same base
    let sum: WrappingInstant<u64, 1, 1_000> =
        WrappingInstant::<u64, 1, 1_000>::from_ticks(10) + Duration::<u32, 1, 1_000>::from_ticks(1);
    assert_eq!(sum, WrappingInstant::<u64, 1, 1_000>::from_ticks(11));

    let mut sum = WrappingInstant::<u64, 1, 1_000>::from_ticks(10);
    sum += Duration::<u32, 1, 1_000>::from_ticks(1);
    assert_eq!(sum, WrappingInstant::<u64, 1, 1_000>::from_ticks(11));

    let diff: WrappingInstant<u64, 1, 1_000> =
        WrappingInstant::<u64, 1, 1_000>::from_ticks(10) - Duration::<u32, 1, 1_000>::from_ticks(1);
    assert_eq!(diff, WrappingInstant::<u64, 1, 1_000>::from_ticks(9));

    let mut diff = WrappingInstant::<u64, 1, 1_000>::from_ticks(10);
    diff -= Duration::<u32, 1, 1_000>::from_ticks(1);
    assert_eq!(diff, WrappingInstant::<u64, 1, 1_000>::from_ticks(9));

    // Instant +- Duration, Different base
    let sum: WrappingInstant<u64, 1, 10_000> = WrappingInstant::<u64, 1, 10_000>::from_ticks(10)
        + Duration::<u32, 1, 1_000>::from_ticks(1).convert();
    assert_eq!(sum, WrappingInstant::<u64, 1, 10_000>::from_ticks(20));

    let mut sum = WrappingInstant::<u64, 1, 10_000>::from_ticks(10);
    sum += Duration::<u32, 1, 1_000>::from_ticks(1).convert();
    assert_eq!(sum, WrappingInstant::<u64, 1, 10_000>::from_ticks(20));

    let diff: WrappingInstant<u64, 1, 10_000> = WrappingInstant::<u64, 1, 10_000>::from_ticks(10)
        - Duration::<u32, 1, 1_000>::from_ticks(1).convert();
    assert_eq!(diff, WrappingInstant::<u64, 1, 10_000>::from_ticks(0));

    let mut diff = WrappingInstant::<u64, 1, 10_000>::from_ticks(10);
    diff -= Duration::<u32, 1, 1_000>::from_ticks(1).convert();
    assert_eq!(diff, WrappingInstant::<u64, 1, 10_000>::from_ticks(0));
}

#[test]
fn instant_operators_wrap_in_both_directions_u32() {
    assert_eq!(
        WrappingInstant::<u32, 1, 1_000>::from_ticks(u32::MAX)
            + Duration::<u32, 1, 1_000>::from_ticks(1),
        WrappingInstant::<u32, 1, 1_000>::from_ticks(0)
    );
    assert_eq!(
        WrappingInstant::<u32, 1, 1_000>::from_ticks(0) - Duration::<u32, 1, 1_000>::from_ticks(1),
        WrappingInstant::<u32, 1, 1_000>::from_ticks(u32::MAX)
    );
    assert_eq!(
        WrappingInstant::<u32, 1, 1_000>::from_ticks(2)
            - WrappingInstant::<u32, 1, 1_000>::from_ticks(u32::MAX),
        Duration::<u32, 1, 1_000>::from_ticks(3)
    );
}

#[test]
fn instant_operators_wrap_in_both_directions_u64() {
    assert_eq!(
        WrappingInstant::<u64, 1, 1_000>::from_ticks(u64::MAX)
            + Duration::<u64, 1, 1_000>::from_ticks(1),
        WrappingInstant::<u64, 1, 1_000>::from_ticks(0)
    );
    assert_eq!(
        WrappingInstant::<u64, 1, 1_000>::from_ticks(0) - Duration::<u64, 1, 1_000>::from_ticks(1),
        WrappingInstant::<u64, 1, 1_000>::from_ticks(u64::MAX)
    );
    assert_eq!(
        WrappingInstant::<u64, 1, 1_000>::from_ticks(2)
            - WrappingInstant::<u64, 1, 1_000>::from_ticks(u64::MAX),
        Duration::<u64, 1, 1_000>::from_ticks(3)
    );
}

#[test]
#[should_panic]
fn instant_sub_incomparable_instant_panics_u32() {
    const HALF: u32 = 1 << 31;

    // Exactly half the range apart, so neither comes first and no duration exists.
    let _ = WrappingInstant::<u32, 1, 1_000>::from_ticks(1)
        - WrappingInstant::<u32, 1, 1_000>::from_ticks(1 + HALF);
}

#[test]
#[should_panic]
fn instant_sub_incomparable_instant_panics_u64() {
    const HALF: u64 = 1 << 63;

    let _ = WrappingInstant::<u64, 1, 1_000>::from_ticks(1)
        - WrappingInstant::<u64, 1, 1_000>::from_ticks(1 + HALF);
}

#[test]
fn instant_is_before_is_after_agree_with_operators_u32() {
    const HALF: u32 = 1 << 31;

    for (a, b) in [
        (
            WrappingInstant::<u32, 1, 1_000>::from_ticks(1),
            WrappingInstant::<u32, 1, 1_000>::from_ticks(2),
        ),
        (
            WrappingInstant::<u32, 1, 1_000>::from_ticks(2),
            WrappingInstant::<u32, 1, 1_000>::from_ticks(1),
        ),
        (
            WrappingInstant::<u32, 1, 1_000>::from_ticks(1),
            WrappingInstant::<u32, 1, 1_000>::from_ticks(1),
        ),
        (
            WrappingInstant::<u32, 1, 1_000>::from_ticks(u32::MAX),
            WrappingInstant::<u32, 1, 1_000>::from_ticks(1),
        ),
        (
            WrappingInstant::<u32, 1, 1_000>::from_ticks(1),
            WrappingInstant::<u32, 1, 1_000>::from_ticks(u32::MAX),
        ),
        (
            WrappingInstant::<u32, 1, 1_000>::from_ticks(u32::MAX - 1),
            WrappingInstant::<u32, 1, 1_000>::from_ticks(u32::MAX),
        ),
    ] {
        assert_eq!(a.is_before(b), a < b);
        assert_eq!(a.is_after(b), a > b);
    }

    // Incomparable: neither predicate holds, matching `<` and `>`.
    let a = WrappingInstant::<u32, 1, 1_000>::from_ticks(1);
    let b = WrappingInstant::<u32, 1, 1_000>::from_ticks(1 + HALF);
    assert!(!a.is_before(b) && !a.is_after(b));
    assert!(!b.is_before(a) && !b.is_after(a));
}

#[test]
fn instant_is_before_is_after_are_const_u32() {
    // The reason these exist: `<` and `>` cannot be used in a const context.
    const I1: WrappingInstant<u32, 1, 1_000> = WrappingInstant::<u32, 1, 1_000>::from_ticks(1);
    const I2: WrappingInstant<u32, 1, 1_000> = WrappingInstant::<u32, 1, 1_000>::from_ticks(2);

    const BEFORE: bool = I1.is_before(I2);
    const AFTER: bool = I1.is_after(I2);

    assert_eq!((BEFORE, AFTER), (true, false));
}

#[test]
fn instant_is_before_is_after_are_const_u64() {
    const I1: WrappingInstant<u64, 1, 1_000> = WrappingInstant::<u64, 1, 1_000>::from_ticks(1);
    const I2: WrappingInstant<u64, 1, 1_000> = WrappingInstant::<u64, 1, 1_000>::from_ticks(2);

    const BEFORE: bool = I1.is_before(I2);
    const AFTER: bool = I1.is_after(I2);

    assert_eq!((BEFORE, AFTER), (true, false));
}

#[test]
fn instant_hash_follows_ticks_u32() {
    use std::collections::hash_map::DefaultHasher;
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};

    fn hash_of(i: WrappingInstant<u32, 1, 1_000>) -> u64 {
        let mut h = DefaultHasher::new();
        i.hash(&mut h);
        h.finish()
    }

    assert_eq!(
        hash_of(WrappingInstant::<u32, 1, 1_000>::from_ticks(7)),
        hash_of(WrappingInstant::<u32, 1, 1_000>::from_ticks(7))
    );
    assert_ne!(
        hash_of(WrappingInstant::<u32, 1, 1_000>::from_ticks(7)),
        hash_of(WrappingInstant::<u32, 1, 1_000>::from_ticks(8))
    );

    let mut map = HashMap::new();
    map.insert(WrappingInstant::<u32, 1, 1_000>::from_ticks(7), "seven");
    assert_eq!(
        map.get(&WrappingInstant::<u32, 1, 1_000>::from_ticks(7)),
        Some(&"seven")
    );
    assert_eq!(
        map.get(&WrappingInstant::<u32, 1, 1_000>::from_ticks(8)),
        None
    );
}

////////////////////////////////////////////////////////////////////////////////
//
// Monotonic instants
//
////////////////////////////////////////////////////////////////////////////////

#[test]
fn monotonic_instant_compare_u32() {
    // Near MAX the comparison is a plain integer compare, the inverse of the wrapping kind.
    assert!(
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(1)
            < MonotonicInstant::<u32, 1, 1_000>::from_ticks(u32::MAX)
    );
    assert!(
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(u32::MAX - 1)
            < MonotonicInstant::<u32, 1, 1_000>::from_ticks(u32::MAX)
    );

    assert!(
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(2)
            > MonotonicInstant::<u32, 1, 1_000>::from_ticks(1)
    );
    assert!(
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(2)
            >= MonotonicInstant::<u32, 1, 1_000>::from_ticks(1)
    );
    assert!(
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(1)
            >= MonotonicInstant::<u32, 1, 1_000>::from_ticks(1)
    );
    assert!(
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(1)
            < MonotonicInstant::<u32, 1, 1_000>::from_ticks(2)
    );
    assert!(
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(1)
            <= MonotonicInstant::<u32, 1, 1_000>::from_ticks(1)
    );
    assert!(
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(1)
            <= MonotonicInstant::<u32, 1, 1_000>::from_ticks(2)
    );
    assert!(
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(1)
            == MonotonicInstant::<u32, 1, 1_000>::from_ticks(1)
    );
    assert!(
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(1)
            != MonotonicInstant::<u32, 1, 1_000>::from_ticks(2)
    );

    assert_eq!(
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(1)
            .checked_duration_since(MonotonicInstant::<u32, 1, 1_000>::from_ticks(1)),
        Some(Duration::<u32, 1, 1_000>::from_ticks(0))
    );
    assert_eq!(
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(2)
            .checked_duration_since(MonotonicInstant::<u32, 1, 1_000>::from_ticks(1)),
        Some(Duration::<u32, 1, 1_000>::from_ticks(1))
    );
    assert_eq!(
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(2)
            .checked_duration_since(MonotonicInstant::<u32, 1, 1_000>::from_ticks(3)),
        None
    );
    assert_eq!(
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(u32::MAX)
            .checked_duration_since(MonotonicInstant::<u32, 1, 1_000>::from_ticks(0)),
        Some(Duration::<u32, 1, 1_000>::from_ticks(u32::MAX))
    );
    assert_eq!(
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(0)
            .checked_duration_since(MonotonicInstant::<u32, 1, 1_000>::from_ticks(u32::MAX)),
        None
    );
}

#[test]
fn monotonic_instant_compare_u64() {
    assert!(
        MonotonicInstant::<u64, 1, 1_000>::from_ticks(1)
            < MonotonicInstant::<u64, 1, 1_000>::from_ticks(u64::MAX)
    );
    assert!(
        MonotonicInstant::<u64, 1, 1_000>::from_ticks(u64::MAX - 1)
            < MonotonicInstant::<u64, 1, 1_000>::from_ticks(u64::MAX)
    );

    assert!(
        MonotonicInstant::<u64, 1, 1_000>::from_ticks(2)
            > MonotonicInstant::<u64, 1, 1_000>::from_ticks(1)
    );
    assert!(
        MonotonicInstant::<u64, 1, 1_000>::from_ticks(1)
            == MonotonicInstant::<u64, 1, 1_000>::from_ticks(1)
    );
    assert!(
        MonotonicInstant::<u64, 1, 1_000>::from_ticks(1)
            != MonotonicInstant::<u64, 1, 1_000>::from_ticks(2)
    );

    assert_eq!(
        MonotonicInstant::<u64, 1, 1_000>::from_ticks(2)
            .checked_duration_since(MonotonicInstant::<u64, 1, 1_000>::from_ticks(1)),
        Some(Duration::<u64, 1, 1_000>::from_ticks(1))
    );
    assert_eq!(
        MonotonicInstant::<u64, 1, 1_000>::from_ticks(2)
            .checked_duration_since(MonotonicInstant::<u64, 1, 1_000>::from_ticks(3)),
        None
    );
    assert_eq!(
        MonotonicInstant::<u64, 1, 1_000>::from_ticks(u64::MAX)
            .checked_duration_since(MonotonicInstant::<u64, 1, 1_000>::from_ticks(0)),
        Some(Duration::<u64, 1, 1_000>::from_ticks(u64::MAX))
    );
    assert_eq!(
        MonotonicInstant::<u64, 1, 1_000>::from_ticks(0)
            .checked_duration_since(MonotonicInstant::<u64, 1, 1_000>::from_ticks(u64::MAX)),
        None
    );
}

#[test]
fn monotonic_instant_compare_is_transitive_u32() {
    // The exact triple that `instant_compare_is_not_transitive_u32` uses to show why the
    // wrapping kind cannot be `Ord`. Read monotonically, it is a plain total order.
    let a = MonotonicInstant::<u32, 1, 1_000>::from_ticks(0);
    let b = MonotonicInstant::<u32, 1, 1_000>::from_ticks(0x6000_0000);
    let c = MonotonicInstant::<u32, 1, 1_000>::from_ticks(0xC000_0000);

    assert!(a < b);
    assert!(b < c);
    assert!(a < c);
}

#[test]
fn monotonic_instant_ord_works_with_std_collections_u32() {
    use std::cmp::Reverse;
    use std::collections::{BTreeSet, BinaryHeap};

    fn assert_ord<T: Ord>() {}
    assert_ord::<MonotonicInstant<u32, 1, 1_000>>();

    let unsorted = [
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(u32::MAX),
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(3),
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(0),
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(7),
    ];
    let ascending = [
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(0),
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(3),
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(7),
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(u32::MAX),
    ];

    let mut sorted = unsorted;
    sorted.sort();
    assert_eq!(sorted, ascending);

    assert_eq!(
        unsorted.iter().max(),
        Some(&MonotonicInstant::<u32, 1, 1_000>::from_ticks(u32::MAX))
    );
    assert_eq!(
        unsorted.iter().min(),
        Some(&MonotonicInstant::<u32, 1, 1_000>::from_ticks(0))
    );

    // Earliest-deadline-first, the use case that motivated keeping `Ord`.
    let mut heap: BinaryHeap<Reverse<_>> = unsorted.iter().copied().map(Reverse).collect();
    let mut popped = Vec::new();
    while let Some(Reverse(next)) = heap.pop() {
        popped.push(next);
    }
    assert_eq!(popped, ascending);

    let set: BTreeSet<_> = unsorted.iter().copied().collect();
    assert_eq!(set.into_iter().collect::<Vec<_>>(), ascending);
}

#[test]
fn monotonic_instant_const_cmp_and_predicates_are_const_u32() {
    use core::cmp::Ordering;

    const I1: MonotonicInstant<u32, 1, 1_000> = MonotonicInstant::<u32, 1, 1_000>::from_ticks(1);
    const I2: MonotonicInstant<u32, 1, 1_000> = MonotonicInstant::<u32, 1, 1_000>::from_ticks(2);

    const CMP: Ordering = I1.const_cmp(I2);
    const BEFORE: bool = I1.is_before(I2);
    const AFTER: bool = I1.is_after(I2);

    assert_eq!(CMP, Ordering::Less);
    assert_eq!((BEFORE, AFTER), (true, false));
}

#[test]
fn monotonic_instant_const_cmp_and_predicates_are_const_u64() {
    use core::cmp::Ordering;

    const I1: MonotonicInstant<u64, 1, 1_000> = MonotonicInstant::<u64, 1, 1_000>::from_ticks(2);
    const I2: MonotonicInstant<u64, 1, 1_000> = MonotonicInstant::<u64, 1, 1_000>::from_ticks(1);

    const CMP: Ordering = I1.const_cmp(I2);
    const BEFORE: bool = I1.is_before(I2);
    const AFTER: bool = I1.is_after(I2);

    assert_eq!(CMP, Ordering::Greater);
    assert_eq!((BEFORE, AFTER), (false, true));
}

#[test]
fn monotonic_instant_predicates_agree_with_operators_u32() {
    for (a, b) in [
        (
            MonotonicInstant::<u32, 1, 1_000>::from_ticks(1),
            MonotonicInstant::<u32, 1, 1_000>::from_ticks(2),
        ),
        (
            MonotonicInstant::<u32, 1, 1_000>::from_ticks(2),
            MonotonicInstant::<u32, 1, 1_000>::from_ticks(1),
        ),
        (
            MonotonicInstant::<u32, 1, 1_000>::from_ticks(1),
            MonotonicInstant::<u32, 1, 1_000>::from_ticks(1),
        ),
        (
            MonotonicInstant::<u32, 1, 1_000>::from_ticks(0),
            MonotonicInstant::<u32, 1, 1_000>::from_ticks(u32::MAX),
        ),
        (
            MonotonicInstant::<u32, 1, 1_000>::from_ticks(u32::MAX),
            MonotonicInstant::<u32, 1, 1_000>::from_ticks(0),
        ),
        (
            MonotonicInstant::<u32, 1, 1_000>::from_ticks(u32::MAX - 1),
            MonotonicInstant::<u32, 1, 1_000>::from_ticks(u32::MAX),
        ),
    ] {
        assert_eq!(a.is_before(b), a < b);
        assert_eq!(a.is_after(b), a > b);
    }
}

#[test]
fn monotonic_instant_predicates_agree_with_operators_u64() {
    for (a, b) in [
        (
            MonotonicInstant::<u64, 1, 1_000>::from_ticks(1),
            MonotonicInstant::<u64, 1, 1_000>::from_ticks(2),
        ),
        (
            MonotonicInstant::<u64, 1, 1_000>::from_ticks(2),
            MonotonicInstant::<u64, 1, 1_000>::from_ticks(1),
        ),
        (
            MonotonicInstant::<u64, 1, 1_000>::from_ticks(1),
            MonotonicInstant::<u64, 1, 1_000>::from_ticks(1),
        ),
        (
            MonotonicInstant::<u64, 1, 1_000>::from_ticks(0),
            MonotonicInstant::<u64, 1, 1_000>::from_ticks(u64::MAX),
        ),
        (
            MonotonicInstant::<u64, 1, 1_000>::from_ticks(u64::MAX),
            MonotonicInstant::<u64, 1, 1_000>::from_ticks(0),
        ),
        (
            MonotonicInstant::<u64, 1, 1_000>::from_ticks(u64::MAX - 1),
            MonotonicInstant::<u64, 1, 1_000>::from_ticks(u64::MAX),
        ),
    ] {
        assert_eq!(a.is_before(b), a < b);
        assert_eq!(a.is_after(b), a > b);
    }
}

#[test]
fn monotonic_instant_checked_math_u32() {
    let one_tick = Duration::<u32, 1, 1_000>::from_ticks(1);

    assert_eq!(
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(10).checked_add_duration(one_tick),
        Some(MonotonicInstant::<u32, 1, 1_000>::from_ticks(11))
    );
    assert_eq!(
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(10).checked_sub_duration(one_tick),
        Some(MonotonicInstant::<u32, 1, 1_000>::from_ticks(9))
    );

    // Tick overflow in both directions, which is what distinguishes this from the wrapping kind.
    assert_eq!(
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(u32::MAX).checked_add_duration(one_tick),
        None
    );
    assert_eq!(
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(0).checked_sub_duration(one_tick),
        None
    );

    // Base-conversion overflow, the other cause of `None`.
    let too_big = Duration::<u32, 1, 500>::from_ticks(u32::MAX / 2 + 1);
    assert_eq!(
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(1).checked_add_duration(too_big),
        None
    );
    assert_eq!(
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(1).checked_sub_duration(too_big),
        None
    );

    // Cross-base arithmetic that does fit.
    assert_eq!(
        MonotonicInstant::<u32, 1, 10_000>::from_ticks(10).checked_add_duration(one_tick),
        Some(MonotonicInstant::<u32, 1, 10_000>::from_ticks(20))
    );
}

#[test]
fn monotonic_instant_checked_math_u64() {
    let one_tick = Duration::<u64, 1, 1_000>::from_ticks(1);

    assert_eq!(
        MonotonicInstant::<u64, 1, 1_000>::from_ticks(10).checked_add_duration(one_tick),
        Some(MonotonicInstant::<u64, 1, 1_000>::from_ticks(11))
    );
    assert_eq!(
        MonotonicInstant::<u64, 1, 1_000>::from_ticks(10).checked_sub_duration(one_tick),
        Some(MonotonicInstant::<u64, 1, 1_000>::from_ticks(9))
    );
    assert_eq!(
        MonotonicInstant::<u64, 1, 1_000>::from_ticks(u64::MAX).checked_add_duration(one_tick),
        None
    );
    assert_eq!(
        MonotonicInstant::<u64, 1, 1_000>::from_ticks(0).checked_sub_duration(one_tick),
        None
    );
}

#[test]
#[should_panic]
fn monotonic_instant_sub_later_instant_panics_u32() {
    // The explicit panic in `Sub<Instant>`, which fires in both profiles.
    let _ = MonotonicInstant::<u32, 1, 1_000>::from_ticks(1)
        - MonotonicInstant::<u32, 1, 1_000>::from_ticks(2);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn monotonic_instant_add_overflow_panics_in_debug_u32() {
    // Plain `+`, so this is an `overflow-checks` panic: debug only. The value side is covered
    // by `checked_add_duration` returning `None`, which holds in both profiles.
    let _ = MonotonicInstant::<u32, 1, 1_000>::from_ticks(u32::MAX)
        + Duration::<u32, 1, 1_000>::from_ticks(1);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn monotonic_instant_sub_underflow_panics_in_debug_u32() {
    let _ =
        MonotonicInstant::<u32, 1, 1_000>::from_ticks(0) - Duration::<u32, 1, 1_000>::from_ticks(1);
}

#[test]
fn monotonic_instant_duration_math_u32() {
    use crate::ExtU32;

    let diff: Duration<u32, 1, 1_000> = MonotonicInstant::<u32, 1, 1_000>::from_ticks(10)
        - MonotonicInstant::<u32, 1, 1_000>::from_ticks(1);
    assert_eq!(diff, Duration::<u32, 1, 1_000>::from_ticks(9));

    let sum: MonotonicInstant<u32, 1, 1_000> = MonotonicInstant::<u32, 1, 1_000>::from_ticks(10)
        + Duration::<u32, 1, 1_000>::from_ticks(1);
    assert_eq!(sum, MonotonicInstant::<u32, 1, 1_000>::from_ticks(11));

    let mut sum = MonotonicInstant::<u32, 1, 1_000>::from_ticks(10);
    sum += Duration::<u32, 1, 1_000>::from_ticks(1);
    assert_eq!(sum, MonotonicInstant::<u32, 1, 1_000>::from_ticks(11));

    let mut diff = MonotonicInstant::<u32, 1, 1_000>::from_ticks(10);
    diff -= Duration::<u32, 1, 1_000>::from_ticks(1);
    assert_eq!(diff, MonotonicInstant::<u32, 1, 1_000>::from_ticks(9));

    // Extension traits, which need the same NOM/DENOM on both sides.
    let sum: MonotonicInstant<u32, 1, 10_000> =
        MonotonicInstant::<u32, 1, 10_000>::from_ticks(10) + 1.millis();
    assert_eq!(sum, MonotonicInstant::<u32, 1, 10_000>::from_ticks(20));

    let diff: MonotonicInstant<u32, 1, 10_000> =
        MonotonicInstant::<u32, 1, 10_000>::from_ticks(10) - 1.millis();
    assert_eq!(diff, MonotonicInstant::<u32, 1, 10_000>::from_ticks(0));
}

#[test]
fn monotonic_instant_duration_math_u64_u32() {
    // The cross-storage operators, which resolve through the kind's own same-base operator.
    let sum: MonotonicInstant<u64, 1, 1_000> = MonotonicInstant::<u64, 1, 1_000>::from_ticks(10)
        + Duration::<u32, 1, 1_000>::from_ticks(1);
    assert_eq!(sum, MonotonicInstant::<u64, 1, 1_000>::from_ticks(11));

    let mut sum = MonotonicInstant::<u64, 1, 1_000>::from_ticks(10);
    sum += Duration::<u32, 1, 1_000>::from_ticks(1);
    assert_eq!(sum, MonotonicInstant::<u64, 1, 1_000>::from_ticks(11));

    let diff: MonotonicInstant<u64, 1, 1_000> = MonotonicInstant::<u64, 1, 1_000>::from_ticks(10)
        - Duration::<u32, 1, 1_000>::from_ticks(1);
    assert_eq!(diff, MonotonicInstant::<u64, 1, 1_000>::from_ticks(9));

    let mut diff = MonotonicInstant::<u64, 1, 1_000>::from_ticks(10);
    diff -= Duration::<u32, 1, 1_000>::from_ticks(1);
    assert_eq!(diff, MonotonicInstant::<u64, 1, 1_000>::from_ticks(9));
}

////////////////////////////////////////////////////////////////////////////////
//
// Across the kinds
//
////////////////////////////////////////////////////////////////////////////////

#[test]
fn debug_output_names_the_kind() {
    assert_eq!(
        format!("{:?}", WrappingInstant::<u32, 1, 1_000>::from_ticks(7)),
        "WrappingInstant { ticks: 7 }"
    );
    assert_eq!(
        format!("{:?}", MonotonicInstant::<u64, 1, 1_000>::from_ticks(7)),
        "MonotonicInstant { ticks: 7 }"
    );
}

#[cfg(feature = "serde")]
#[test]
fn both_kinds_implement_serde() {
    fn assert_serde<T: serde::Serialize + serde::de::DeserializeOwned>() {}

    assert_serde::<WrappingInstant<u32, 1, 1_000>>();
    assert_serde::<MonotonicInstant<u64, 1, 1_000>>();
}

#[cfg(feature = "postcard_max_size")]
#[test]
fn both_kinds_implement_max_size() {
    use postcard::experimental::max_size::MaxSize;

    assert_eq!(
        WrappingInstant::<u32, 1, 1_000>::POSTCARD_MAX_SIZE,
        u32::POSTCARD_MAX_SIZE
    );
    assert_eq!(
        MonotonicInstant::<u64, 1, 1_000>::POSTCARD_MAX_SIZE,
        u64::POSTCARD_MAX_SIZE
    );
}

#[test]
fn both_kinds_hash_equal_ticks_equally_u64() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn hash_of(value: impl Hash) -> u64 {
        let mut h = DefaultHasher::new();
        value.hash(&mut h);
        h.finish()
    }

    assert_eq!(
        hash_of(WrappingInstant::<u64, 1, 1_000>::from_ticks(7)),
        hash_of(MonotonicInstant::<u64, 1, 1_000>::from_ticks(7))
    );
    assert_ne!(
        hash_of(WrappingInstant::<u64, 1, 1_000>::from_ticks(7)),
        hash_of(MonotonicInstant::<u64, 1, 1_000>::from_ticks(8))
    );
}
