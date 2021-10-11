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
    pub const SAME_BASE: bool = Self::RH_CHECK == Self::LH_CHECK;

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
