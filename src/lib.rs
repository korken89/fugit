pub mod duration;
pub mod helpers;
pub mod instant;

#[cfg(test)]
mod test {
    use crate::duration::Duration;

    #[test]
    fn duration_compare() {
        // Same fraction
        assert!(Duration::<1, 1_000>::new(2) > Duration::<1, 1_000>::new(1));
        assert!(Duration::<1, 1_000>::new(2) >= Duration::<1, 1_000>::new(1));
        assert!(Duration::<1, 1_000>::new(1) >= Duration::<1, 1_000>::new(1));
        assert!(Duration::<1, 1_000>::new(1) < Duration::<1, 1_000>::new(2));
        assert!(Duration::<1, 1_000>::new(1) <= Duration::<1, 1_000>::new(1));
        assert!(Duration::<1, 1_000>::new(1) <= Duration::<1, 1_000>::new(2));
        assert!(Duration::<1, 1_000>::new(1) == Duration::<1, 1_000>::new(1));
        assert!(Duration::<1, 1_000>::new(1) != Duration::<1, 1_000>::new(2));

        // Different fraction
        assert!(Duration::<1, 10_000>::new(11) > Duration::<1, 1_000>::new(1));
        assert!(Duration::<1, 10_000>::new(11) >= Duration::<1, 1_000>::new(1));
        assert!(Duration::<1, 10_000>::new(10) >= Duration::<1, 1_000>::new(1));
        assert!(Duration::<1, 10_000>::new(11) < Duration::<1, 1_000>::new(2));
        assert!(Duration::<1, 10_000>::new(1) <= Duration::<1, 1_000>::new(1));
        assert!(Duration::<1, 10_000>::new(10) <= Duration::<1, 1_000>::new(1));
        assert!(Duration::<1, 10_000>::new(10) == Duration::<1, 1_000>::new(1));
        assert!(Duration::<1, 10_000>::new(9) != Duration::<1, 1_000>::new(2));
    }

    #[test]
    fn duration_duration_math() {
        // Same base
        let sum: Duration<1, 1_000> = Duration::<1, 1_000>::new(10) + Duration::<1, 1_000>::new(1);
        assert_eq!(sum, Duration::<1, 1_000>::new(11));

        let diff: Duration<1, 1_000> = Duration::<1, 1_000>::new(10) - Duration::<1, 1_000>::new(1);
        assert_eq!(diff, Duration::<1, 1_000>::new(9));

        // Different base
        let sum: Duration<1, 10_000> =
            Duration::<1, 10_000>::new(10) + Duration::<1, 1_000>::new(1);
        assert_eq!(sum, Duration::<1, 1_000>::new(2));

        let diff: Duration<1, 10_000> =
            Duration::<1, 10_000>::new(10) - Duration::<1, 1_000>::new(1);
        assert_eq!(diff, Duration::<1, 1_000>::new(0));
    }

    #[test]
    fn duration_shorthands() {
        use crate::duration::ExtU32;

        let d: Duration<1, 10_000> = 100_000.micros();
        assert_eq!(d.ticks(), 1_000);

        let d: Duration<1, 10_000> = 1.millis();
        assert_eq!(d.ticks(), 10);

        let d: Duration<1, 10_000> = 1.secs();
        assert_eq!(d.ticks(), 10_000);

        let d: Duration<1, 10_000> = 1.minutes();
        assert_eq!(d.ticks(), 600_000);

        let d: Duration<1, 10_000> = 1.hours();
        assert_eq!(d.ticks(), 36_000_000);
    }

    #[test]
    fn instant_compare() {
        todo!()
    }

    #[test]
    fn instant_duration_math() {
        todo!()
    }
}
