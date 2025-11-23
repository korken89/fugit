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

    // Rate % integer
    let r1 = Rate::<u32, 1_000, 1>::from_raw(10);
    assert_eq!(r1 % 3, Rate::<u32, 1_000, 1>::from_raw(1));

    // RemAssign Rate
    let mut r1 = Rate::<u32, 1_000, 1>::from_raw(10);
    r1 %= Rate::<u32, 1_000, 1>::from_raw(3);
    assert_eq!(r1, Rate::<u32, 1_000, 1>::from_raw(1));

    // RemAssign integer
    let mut r1 = Rate::<u32, 1_000, 1>::from_raw(10);
    r1 %= 3;
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

    // Rate % integer
    let r1 = Rate::<u64, 1_000, 1>::from_raw(10);
    assert_eq!(r1 % 3, Rate::<u64, 1_000, 1>::from_raw(1));

    // RemAssign Rate
    let mut r1 = Rate::<u64, 1_000, 1>::from_raw(10);
    r1 %= Rate::<u64, 1_000, 1>::from_raw(3);
    assert_eq!(r1, Rate::<u64, 1_000, 1>::from_raw(1));

    // RemAssign integer
    let mut r1 = Rate::<u64, 1_000, 1>::from_raw(10);
    r1 %= 3;
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
