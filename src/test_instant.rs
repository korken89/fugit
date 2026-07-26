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
fn instant_compare_half_range_u32() {
    use core::cmp::Ordering;

    const HALF: u32 = 1 << 31;
    let i = Instant::<u32, 1, 1_000>::from_ticks;

    // The largest unambiguous difference. Used to report `Equal` while the ticks differ.
    assert_eq!(i(HALF).const_partial_cmp(i(1)), Some(Ordering::Greater));
    assert_eq!(i(1).const_partial_cmp(i(HALF)), Some(Ordering::Less));
    assert!(i(HALF) != i(1));

    // Exactly half the range apart: `a - b` and `b - a` are both `HALF`, so neither
    // comes first and no comparison holds.
    let (a, b) = (i(1), i(1 + HALF));
    assert_eq!(a.const_partial_cmp(b), None);
    assert_eq!(b.const_partial_cmp(a), None);
    for holds in [a < b, a > b, a <= b, a >= b, a == b] {
        assert!(!holds);
    }

    // Just past half the range, the comparison flips.
    assert_eq!(i(HALF + 1).const_partial_cmp(i(0)), Some(Ordering::Less));
    assert_eq!(i(0).const_partial_cmp(i(HALF + 1)), Some(Ordering::Greater));

    // `checked_duration_since` is unaffected by the fix.
    assert_eq!(
        i(HALF).checked_duration_since(i(1)),
        Some(Duration::<u32, 1, 1_000>::from_ticks(HALF - 1))
    );
    assert_eq!(a.checked_duration_since(b), None);
    assert_eq!(b.checked_duration_since(a), None);
}

#[test]
fn instant_compare_half_range_u64() {
    use core::cmp::Ordering;

    const HALF: u64 = 1 << 63;
    let i = Instant::<u64, 1, 1_000>::from_ticks;

    assert_eq!(i(HALF).const_partial_cmp(i(1)), Some(Ordering::Greater));
    assert_eq!(i(1).const_partial_cmp(i(HALF)), Some(Ordering::Less));
    assert!(i(HALF) != i(1));

    let (a, b) = (i(1), i(1 + HALF));
    assert_eq!(a.const_partial_cmp(b), None);
    assert_eq!(b.const_partial_cmp(a), None);
    for holds in [a < b, a > b, a <= b, a >= b, a == b] {
        assert!(!holds);
    }

    assert_eq!(i(HALF + 1).const_partial_cmp(i(0)), Some(Ordering::Less));
    assert_eq!(i(0).const_partial_cmp(i(HALF + 1)), Some(Ordering::Greater));

    assert_eq!(
        i(HALF).checked_duration_since(i(1)),
        Some(Duration::<u64, 1, 1_000>::from_ticks(HALF - 1))
    );
    assert_eq!(a.checked_duration_since(b), None);
    assert_eq!(b.checked_duration_since(a), None);
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
        let a = Instant::<u32, 1, 1_000>::from_ticks(base);
        let b = Instant::<u32, 1, 1_000>::from_ticks(base.wrapping_add(offset));

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
    let a = Instant::<u32, 1, 1_000>::from_ticks(0);
    let b = Instant::<u32, 1, 1_000>::from_ticks(0x6000_0000);
    let c = Instant::<u32, 1, 1_000>::from_ticks(0xC000_0000);

    assert!(a < b);
    assert!(b < c);
    assert!(a > c);
}

#[test]
fn instant_compare_is_not_transitive_u64() {
    let a = Instant::<u64, 1, 1_000>::from_ticks(0);
    let b = Instant::<u64, 1, 1_000>::from_ticks(0x6000_0000_0000_0000);
    let c = Instant::<u64, 1, 1_000>::from_ticks(0xC000_0000_0000_0000);

    assert!(a < b);
    assert!(b < c);
    assert!(a > c);
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
