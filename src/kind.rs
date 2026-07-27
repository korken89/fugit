//! Markers that say how an [`Instant`](crate::Instant)'s timeline is read.

mod sealed {
    pub trait Sealed {}
}

/// The set of timelines an [`Instant`](crate::Instant) can be on.
///
/// Sealed: the kinds are [`Monotonic`] and [`Wrapping`].
pub trait Kind: Copy + sealed::Sealed {
    /// The name shown for instants of this kind in `Debug` output.
    const DEBUG_NAME: &'static str;
}

/// Marker for an instant on a timeline that does not wrap.
///
/// Total order, so [`Ord`]. Arithmetic is plain `+` and `-`, panicking on overflow in debug and
/// wrapping in release. Nothing checks that the timeline really does not wrap, so use
/// [`Wrapping`] for raw hardware counters.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Monotonic {}

impl sealed::Sealed for Monotonic {}
impl Kind for Monotonic {
    const DEBUG_NAME: &'static str = "MonotonicInstant";
}

/// Marker for an instant on a circular counter, such as a hardware timer.
///
/// Arithmetic wraps, and comparison reads `self - other` as a signed offset, so it is only
/// meaningful within half the tick range and not transitive. [`PartialOrd`] formally requires
/// transitivity too; implementing it anyway is a conscious decision to keep `<` and `>` usable,
/// and the ordering holds whenever the compared instants lie within half the tick range of each
/// other. [`Ord`] is not implemented:
///
/// ```compile_fail
/// use fugit::WrappingInstant;
///
/// fn assert_ord<T: Ord>() {}
/// assert_ord::<WrappingInstant<u32, 1, 1_000>>();
/// ```
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Wrapping {}

impl sealed::Sealed for Wrapping {}
impl Kind for Wrapping {
    const DEBUG_NAME: &'static str = "WrappingInstant";
}
