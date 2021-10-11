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
        pub const LH_CHECK: u32 = (R_DENOM * L_NOM) / Self::DIVISOR;
        pub const RH_CHECK: u32 = (L_DENOM * R_NOM) / Self::DIVISOR;

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
    use core::cmp::Ordering;
    use super::Ratio;

    pub struct Duration<const NOM: u32, const DENOM: u32> {
        pub value: u32,
    }

    impl<const NOM: u32, const DENOM: u32> Duration<NOM, DENOM> {
        pub const fn ratio() -> Ratio {
            Ratio {
                nom: NOM,
                denom: DENOM,
            }
        }
    }

    impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
        PartialOrd<Duration<L_NOM, L_DENOM>> for Duration<R_NOM, R_DENOM>
    {
        fn partial_cmp(&self, other: &Duration<L_NOM, L_DENOM>) -> Option<Ordering> {
            // Lol this works
            // let test = Helpers::<L_NOM, L_DENOM, R_NOM, R_DENOM>::LH_CHECK;

            // This not
            // const TEST: u32 = gcd_binary_u32(L_DENOM * R_NOM, R_DENOM * L_NOM);

            todo!()
        }
    }

    impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
        PartialEq<Duration<L_NOM, L_DENOM>> for Duration<R_NOM, R_DENOM>
    {
        fn eq(&self, other: &Duration<L_NOM, L_DENOM>) -> bool {
            todo!()
        }
    }
}

pub mod instant {
    pub struct Instant<const NOM: u32, const DENOM: u32> {
        pub value: u32,
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn duration_compare() {
        todo!()
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
