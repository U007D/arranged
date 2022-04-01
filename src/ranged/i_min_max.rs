use super::*;
use arith_traits::IMinMax;

impl<TRange> const IMinMax for Ranged<TRange>
where
    Self: PartialOrd,
    TRange: ~const IRangeFrom + ~const IRangeTo + ~const IRangeToInclusive,
{
    const MAX: Self = Self(TRange::end());
    const MIN: Self = Self(TRange::start());
}
