use super::Fraction;

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
pub struct Helpers<const L: Fraction, const R: Fraction>;

impl<const L: Fraction, const R: Fraction> Helpers<L, R> {
    /// Helper constants generated at compile time
    pub const DIVISOR: u64 =
        gcd::binary_u64(L.denom as u64 * R.num as u64, R.denom as u64 * L.num as u64);

    /// Helper constants generated at compile time
    pub const DIVISOR_2: u64 =
        gcd::binary_u64(L.num as u64 * R.num as u64, R.denom as u64 * L.denom as u64);

    /// Helper constants generated at compile time for Durations
    pub const RD_TIMES_LN: u64 = (R.denom as u64 * L.num as u64) / Self::DIVISOR;

    /// Helper constants generated at compile time
    pub const LD_TIMES_RN: u64 = (L.denom as u64 * R.num as u64) / Self::DIVISOR;

    /// Helper constants generated at compile time for Rates
    pub const LN_TIMES_RN: u64 = (L.num as u64 * R.num as u64) / Self::DIVISOR_2;

    /// Helper constants generated at compile time for Rates
    pub const RD_TIMES_LD: u64 = (R.denom as u64 * L.denom as u64) / Self::DIVISOR_2;

    /// Helper constants generated at compile time for Rates
    pub const RATE_TO_DURATION_NUMERATOR: u64 = Self::RD_TIMES_LD / Self::LN_TIMES_RN;

    /// Helper constants generated at compile time
    pub const SAME_BASE: bool = Self::LD_TIMES_RN == Self::RD_TIMES_LN;
}
