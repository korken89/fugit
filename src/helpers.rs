/// Needed due to not being allowed to call const-fn in `PartialEq` for some reason
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
pub struct Helpers<const L_NOM: u64, const L_DENOM: u64, const R_NOM: u64, const R_DENOM: u64>;

impl<const L_NOM: u64, const L_DENOM: u64, const R_NOM: u64, const R_DENOM: u64>
    Helpers<L_NOM, L_DENOM, R_NOM, R_DENOM>
{
    /// Helper constants generated at compile time (intermediate u128 calculation)
    const DIVISOR_U128: u128 = gcd::binary_u128(
        L_DENOM as u128 * R_NOM as u128,
        R_DENOM as u128 * L_NOM as u128,
    );

    /// Helper constants generated at compile time (intermediate u128 calculation)
    const DIVISOR_2_U128: u128 = gcd::binary_u128(
        L_NOM as u128 * R_NOM as u128,
        R_DENOM as u128 * L_DENOM as u128,
    );

    /// Helper constants generated at compile time for Durations (intermediate u128 calculation)
    const RD_TIMES_LN_U128: u128 = (R_DENOM as u128 * L_NOM as u128) / Self::DIVISOR_U128;

    /// Helper constants generated at compile time (intermediate u128 calculation)
    const LD_TIMES_RN_U128: u128 = (L_DENOM as u128 * R_NOM as u128) / Self::DIVISOR_U128;

    /// Helper constants generated at compile time for Rates (intermediate u128 calculation)
    const LN_TIMES_RN_U128: u128 = (L_NOM as u128 * R_NOM as u128) / Self::DIVISOR_2_U128;

    /// Helper constants generated at compile time for Rates (intermediate u128 calculation)
    const RD_TIMES_LD_U128: u128 = (R_DENOM as u128 * L_DENOM as u128) / Self::DIVISOR_2_U128;

    /// Helper constants generated at compile time for Rates (intermediate u128 calculation)
    const RATE_TO_DURATION_NUMERATOR_U128: u128 = Self::RD_TIMES_LD_U128 / Self::LN_TIMES_RN_U128;

    /// Helper constants generated at compile time for Durations
    pub const RD_TIMES_LN: u64 = {
        assert!(
            Self::RD_TIMES_LN_U128 <= u64::MAX as u128,
            "RD_TIMES_LN overflows u64 - NOM/DENOM values too large"
        );
        Self::RD_TIMES_LN_U128 as u64
    };

    /// Helper constants generated at compile time
    pub const LD_TIMES_RN: u64 = {
        assert!(
            Self::LD_TIMES_RN_U128 <= u64::MAX as u128,
            "LD_TIMES_RN overflows u64 - NOM/DENOM values too large"
        );
        Self::LD_TIMES_RN_U128 as u64
    };

    /// Helper constants generated at compile time for Rates
    pub const RATE_TO_DURATION_NUMERATOR: u64 = {
        assert!(
            Self::RATE_TO_DURATION_NUMERATOR_U128 <= u64::MAX as u128,
            "RATE_TO_DURATION_NUMERATOR overflows u64 - NOM/DENOM values too large"
        );
        Self::RATE_TO_DURATION_NUMERATOR_U128 as u64
    };

    /// Helper constants generated at compile time
    pub const SAME_BASE: bool = Self::LD_TIMES_RN == Self::RD_TIMES_LN;
}

#[allow(dead_code)]
#[allow(path_statements)]
pub(crate) const fn greater_than_0<const N: u64>() {
    Assert::<N, 0>::GREATER;
}

#[allow(dead_code)]
/// Const assert hack
pub struct Assert<const L: u64, const R: u64>;

#[allow(dead_code)]
impl<const L: u64, const R: u64> Assert<L, R> {
    /// Const assert hack
    pub const GREATER_EQ: () = assert!(L >= R);

    /// Const assert hack
    pub const LESS_EQ: () = assert!(L <= R);

    /// Const assert hack
    pub const NOT_EQ: () = assert!(L != R);

    /// Const assert hack
    pub const EQ: () = assert!(L == R);

    /// Const assert hack
    pub const GREATER: () = assert!(L > R);

    /// Const assert hack
    pub const LESS: () = assert!(L < R);

    /// Const assert hack
    pub const POWER_OF_TWO: () = assert!(L.is_power_of_two());
}
