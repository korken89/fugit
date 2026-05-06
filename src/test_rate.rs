////////////////////////////////////////////////////////////////////////////////
//
// Rate tests
//
////////////////////////////////////////////////////////////////////////////////

use crate::{Duration, Rate};
use crate::{
    Hertz, HertzU32, HertzU64, Kilohertz, KilohertzU32, KilohertzU64, Megahertz, MegahertzU32,
    MegahertzU64, TimerRate, TimerRateU32, TimerRateU64,
};

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
fn rate_rem_u32() {
    // Same base - Rate % Rate
    let r1 = Rate::<u32, 1_000, 1>::from_raw(10);
    let r2 = Rate::<u32, 1_000, 1>::from_raw(3);
    assert_eq!(r1 % r2, Rate::<u32, 1_000, 1>::from_raw(1));

    // RemAssign Rate
    let mut r1 = Rate::<u32, 1_000, 1>::from_raw(10);
    r1 %= Rate::<u32, 1_000, 1>::from_raw(3);
    assert_eq!(r1, Rate::<u32, 1_000, 1>::from_raw(1));

    // Test checking if a frequency is a multiple of another
    let r1 = Rate::<u32, 1, 1>::Hz(100);
    let r2 = Rate::<u32, 1, 1>::Hz(25);
    assert_eq!(r1 % r2, Rate::<u32, 1, 1>::Hz(0)); // 100 Hz is a multiple of 25 Hz

    let r1 = Rate::<u32, 1, 1>::Hz(100);
    let r2 = Rate::<u32, 1, 1>::Hz(30);
    assert_eq!(r1 % r2, Rate::<u32, 1, 1>::Hz(10)); // 100 Hz mod 30 Hz = 10 Hz
}

#[test]
fn rate_rem_u64() {
    // Same base - Rate % Rate
    let r1 = Rate::<u64, 1_000, 1>::from_raw(10);
    let r2 = Rate::<u64, 1_000, 1>::from_raw(3);
    assert_eq!(r1 % r2, Rate::<u64, 1_000, 1>::from_raw(1));

    // RemAssign Rate
    let mut r1 = Rate::<u64, 1_000, 1>::from_raw(10);
    r1 %= Rate::<u64, 1_000, 1>::from_raw(3);
    assert_eq!(r1, Rate::<u64, 1_000, 1>::from_raw(1));
}

#[test]
fn rate_checked_rem() {
    // Successful checked_rem
    let r1 = Rate::<u32, 1_000, 1>::from_raw(10);
    let r2 = Rate::<u32, 1_000, 1>::from_raw(3);
    assert_eq!(r1.checked_rem(r2), Some(Rate::<u32, 1_000, 1>::from_raw(1)));

    // Division by zero should return None
    let r1 = Rate::<u32, 1_000, 1>::from_raw(10);
    let r2 = Rate::<u32, 1_000, 1>::from_raw(0);
    assert_eq!(r1.checked_rem(r2), None);

    // Different base
    let r1 = Rate::<u32, 1_000, 1>::from_raw(350); // 350 kHz = 350_000 Hz
    let r2 = Rate::<u32, 10_000, 1>::from_raw(10); // 100 kHz = 100_000 Hz
    assert_eq!(
        r1.checked_rem(r2),
        Some(Rate::<u32, 1_000, 1>::from_raw(50)) // 350 kHz % 100 kHz = 50 kHz
    );
}

#[test]
fn rate_conversion_rounding() {
    // Issue #50: Rate conversion should use half-up rounding instead of truncating.

    // Test case 1: Conversion that should round up.
    // Rate<u32, 2, 1> raw=4 means 4 * 2/1 = 8 Hz
    // Converting to Rate<u32, 3, 1>: 8 Hz = x * 3/1 => x = 8/3 = 2.666...
    // With half-up rounding: (8 + 1.5) / 3 = 9.5 / 3 = 3 (rounds up)
    // Without rounding: 8 / 3 = 2 (truncates)
    let rate1 = Rate::<u32, 2, 1>::from_raw(4);
    let rate2: Rate<u32, 3, 1> = rate1.const_try_into().unwrap();
    assert_eq!(rate2.to_raw(), 3, "8 Hz should round to 3 in base 3/1");

    // Test case 2: Conversion that should round down.
    // Rate<u32, 2, 1> raw=2 means 2 * 2/1 = 4 Hz
    // Converting to Rate<u32, 3, 1>: 4 Hz = x * 3/1 => x = 4/3 = 1.333...
    // With half-up rounding: (4 + 1.5) / 3 = 5.5 / 3 = 1 (rounds down)
    // Without rounding: 4 / 3 = 1 (truncates)
    let rate3 = Rate::<u32, 2, 1>::from_raw(2);
    let rate4: Rate<u32, 3, 1> = rate3.const_try_into().unwrap();
    assert_eq!(rate4.to_raw(), 1, "4 Hz should round to 1 in base 3/1");

    // Test case 3: Another rounding up case.
    // Rate<u32, 5, 1> raw=3 means 3 * 5/1 = 15 Hz
    // Converting to Rate<u32, 7, 1>: 15 Hz = x * 7/1 => x = 15/7 = 2.142...
    // With half-up rounding: (15 + 3.5) / 7 = 18.5 / 7 = 2 (rounds down)
    let rate5 = Rate::<u32, 5, 1>::from_raw(3);
    let rate6: Rate<u32, 7, 1> = rate5.const_try_into().unwrap();
    assert_eq!(rate6.to_raw(), 2, "15 Hz should round to 2 in base 7/1");

    // Test case 4: Conversion that rounds up at exactly 0.5.
    // Rate<u32, 1, 1> raw=5 means 5 * 1/1 = 5 Hz
    // Converting to Rate<u32, 2, 1>: 5 Hz = x * 2/1 => x = 5/2 = 2.5
    // With half-up rounding: (5 + 1) / 2 = 6 / 2 = 3 (rounds up)
    // Without rounding: 5 / 2 = 2 (truncates)
    let rate7 = Rate::<u32, 1, 1>::from_raw(5);
    let rate8: Rate<u32, 2, 1> = rate7.const_try_into().unwrap();
    assert_eq!(
        rate8.to_raw(),
        3,
        "5 Hz should round up to 3 in base 2/1 (half-up rounding)"
    );

    // Test case 5: Exact conversion (no rounding needed).
    // Rate<u32, 1, 1> raw=6 means 6 * 1/1 = 6 Hz
    // Converting to Rate<u32, 2, 1>: 6 Hz = x * 2/1 => x = 6/2 = 3 (exact)
    let rate9 = Rate::<u32, 1, 1>::from_raw(6);
    let rate10: Rate<u32, 2, 1> = rate9.const_try_into().unwrap();
    assert_eq!(
        rate10.to_raw(),
        3,
        "6 Hz should convert exactly to 3 in base 2/1"
    );
}

#[test]
fn rate_const_try_from_no_rounding_overflow_u64() {
    // Regression: previously the rounding step `lh + LD_TIMES_RN/2` overflowed
    // u64 silently when raw was close to u64::MAX, producing Some(garbage).

    // u64::MAX of (1/1000) base into (1/1) base. RD_TIMES_LN = 1, LD_TIMES_RN
    // = 1000, lh = u64::MAX, remainder 615 so the answer rounds up.
    let r1 = Rate::<u64, 1, 1_000>::from_raw(u64::MAX);
    let r2: Option<Rate<u64, 1, 1>> = r1.const_try_into();
    assert_eq!(r2.unwrap().to_raw(), u64::MAX / 1000 + 1);
}

#[test]
#[should_panic(expected = "Rate::to_Hz: multiplication overflowed storage type")]
fn rate_to_hz_panics_on_runtime_overflow_u32() {
    // Regression: 5 GHz expressed as Rate<u32, 1e9, 1>::from_raw(5) used to
    // silently wrap to 705_032_704 in release. Now it panics.
    let r = Rate::<u32, 1_000_000_000, 1>::from_raw(5);
    let _ = r.to_Hz();
}

#[test]
fn rate_to_hz_works_for_valid_u32() {
    // Make sure the const_checked path doesn't break valid uses.
    let r = Rate::<u32, 1, 1>::from_raw(1_234);
    assert_eq!(r.to_Hz(), 1_234);

    let r = Rate::<u32, 1_000, 1>::from_raw(2);
    assert_eq!(r.to_Hz(), 2_000);
}

#[test]
#[should_panic(expected = "Rate::Hz: multiplication overflowed storage type")]
fn rate_hz_constructor_panics_on_runtime_overflow_u32() {
    // Try to construct 5 GHz in a Rate type whose period (1 raw unit) is 1 Hz:
    // Rate<u32, 1, 1>::Hz(5_000_000_000) - val itself overflows u32 but use a
    // different combination that overflows the multiplication.
    // Rate<u32, 1, 5>::Hz(val): RD_TIMES_LN = 5, so val * 5 must fit u32.
    // For val = u32::MAX, 5*u32::MAX overflows.
    let _ = Rate::<u32, 1, 5>::Hz(u32::MAX);
}
