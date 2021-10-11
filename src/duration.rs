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
        //
        // We want to check:
        //
        // n_lh / d_lh * lh_ticks {cmp} n_rh / d_rh * rh_ticks
        //
        // simplify to
        //
        // n_lh * d_rh * lh_ticks {cmp} n_rh * d_lh * rh_ticks
        //
        // find gdc(n_lh * d_rh, n_rh * d_lh) and use that to make the constants minimal (done
        // with the `helpers::Helpers` struct)
        //
        // then perform the comparison in a comparable basis
        //

        if Helpers::<L_NOM, L_DENOM, R_NOM, R_DENOM>::SAME_BASE {
            // If we are in the same base, comparison in trivial
            Some(self.ticks.cmp(&other.ticks))
        } else {
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
}

impl<const NOM: u32, const DENOM: u32> Ord for Duration<NOM, DENOM> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ticks.cmp(&other.ticks)
    }
}

impl<const L_NOM: u32, const L_DENOM: u32, const R_NOM: u32, const R_DENOM: u32>
    PartialEq<Duration<L_NOM, L_DENOM>> for Duration<R_NOM, R_DENOM>
{
    fn eq(&self, other: &Duration<L_NOM, L_DENOM>) -> bool {
        if Helpers::<L_NOM, L_DENOM, R_NOM, R_DENOM>::SAME_BASE {
            // If we are in the same base, comparison in trivial
            self.ticks.eq(&other.ticks)
        } else {
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

impl<const NOM: u32, const DENOM: u32> Eq for Duration<NOM, DENOM> {}
