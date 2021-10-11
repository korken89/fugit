/// Test PR

pub mod helpers {
    /// Needed due to not being allowed to call const-fn in `PartialEq` fo some reasion
    /// get the error:
    ///
    /// ```console
    /// error[E0401]: can't use generic parameters from outer function
    ///   --> src/main.rs:25:47
    ///    |
    /// 21 | impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
    ///    |                                                                    ------- const parameter from outer function
    /// ...
    /// 25 |         const TEST: u32 = gcd_binary_u32(L_DENOM, R_DENOM);
    ///    |                                                   ^^^^^^^ use of generic parameter from outer function
    ///
    /// For more information about this error, try `rustc --explain E0401`
    /// ```
    pub struct Helpers<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>;

    impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
        Helpers<L_NOM, L_DENOM, R_NOM, R_DENOM>
    {
        pub const DIVISOR: u32 = gcd_binary_u32(L_DENOM * R_NOM, R_DENOM * L_NOM);
        pub const RH_CHECK: u32 = (R_DENOM * L_NOM) / Self::DIVISOR;
        pub const LH_CHECK: u32 = (L_DENOM * R_NOM) / Self::DIVISOR;

        // TODO: Add asserting method for giving compile time errors
    }

    pub const fn gcd_binary_u32(mut u: u32, mut v: u32) -> u32 {
        if u == 0 {
            return v;
        }

        if v == 0 {
            return u;
        }

        let shift = (u | v).trailing_zeros();
        u >>= shift;
        v >>= shift;
        u >>= u.trailing_zeros();

        loop {
            v >>= v.trailing_zeros();

            if u > v {
                let t = u;
                u = v;
                v = t;
            }

            v -= u; // here v >= u

            if v == 0 {
                break;
            }
        }

        u << shift
    }
}

pub struct Ratio {
    pub nom: u32,
    pub denom: u32,
}

pub mod duration {
    use super::{helpers::Helpers, Ratio};
    use core::cmp::Ordering;

    pub struct Duration<const NOM: u32, const DENOM: u32> {
        pub ticks: u32,
    }

    impl<const NOM: u32, const DENOM: u32> Duration<NOM, DENOM> {
        pub const fn ratio() -> Ratio {
            Ratio {
                nom: NOM,
                denom: DENOM,
            }
        }

        pub const fn new(ticks: u32) -> Self {
            Duration { ticks }
        }
    }

    impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
        PartialOrd<Duration<L_NOM, L_DENOM>> for Duration<R_NOM, R_DENOM>
    {
        fn partial_cmp(&self, other: &Duration<L_NOM, L_DENOM>) -> Option<Ordering> {
            Some(
                self.ticks
                    .checked_mul(Helpers::<L_NOM, L_DENOM, R_NOM, R_DENOM>::LH_CHECK)?
                    .cmp(
                        &other
                            .ticks
                            .checked_mul(Helpers::<L_NOM, L_DENOM, R_NOM, R_DENOM>::RH_CHECK)?,
                    ),
            )
        }
    }

    impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
        PartialEq<Duration<L_NOM, L_DENOM>> for Duration<R_NOM, R_DENOM>
    {
        fn eq(&self, other: &Duration<L_NOM, L_DENOM>) -> bool {
            let lh = self
                .ticks
                .checked_mul(Helpers::<L_NOM, L_DENOM, R_NOM, R_DENOM>::LH_CHECK);
            let rh = other
                .ticks
                .checked_mul(Helpers::<L_NOM, L_DENOM, R_NOM, R_DENOM>::RH_CHECK);

            if let (Some(lh), Some(rh)) = (lh, rh) {
                lh == rh
            } else {
                false
            }
        }
    }
}

pub mod instant {
    pub struct Instant<const NOM: u32, const DENOM: u32> {
        pub ticks: u32,
    }
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
