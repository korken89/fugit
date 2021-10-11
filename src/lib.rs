/// Test PR

pub mod helpers;
pub mod duration;
pub mod instant;

pub struct Ratio {
    pub nom: u32,
    pub denom: u32,
}

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
    fn instant_compare() {
        todo!()
    }

    #[test]
    fn instant_duration_math() {
        todo!()
    }

    #[test]
    fn duration_duration_math() {
        todo!()
    }

    #[test]
    fn duration_ratio_conversion() {
        todo!()
    }

    #[test]
    fn instant_ratio_conversion() {
        todo!()
    }
}
