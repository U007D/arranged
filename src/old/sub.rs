#[cfg(test)]
mod unit_tests;

use super::*;
use crate::ranges::{
    RangeInclusiveI128, RangeInclusiveI16, RangeInclusiveI32, RangeInclusiveI64, RangeInclusiveI8, RangeInclusiveIsize,
    RangeInclusiveU128, RangeInclusiveU16, RangeInclusiveU32, RangeInclusiveU64, RangeInclusiveU8, RangeInclusiveUsize,
};
use std::ops::Sub;

macro_rules! impl_ranged_inclusive_sub {
    ($($SrcTL:ident $SrcTU:ident $RangeInclusive:ident),+) => {
        $(
            // ∵ ranged bounds are always const, use of `unwrap()` in `const` context does not panic, but rather halts
            // compilation in the event of overflow.  This is exactly the behavior we want to ensure compile-time
            // verification of bounds and zero runtime overhead.  `checked_add()` is used to ensure compilation halts on
            // overflow regardless of build settings (may be unnecessary).  See
            // https://stackoverflow.com/a/66362584/1541330 for an explanation of why the `where` clause is necessary.
            #[allow(clippy::unwrap_used)]
            impl<TRiLhs, TRiRhs> Sub<TRiRhs> for $SrcTU<TRiLhs>
            where
                $SrcTU<$RangeInclusive<{ TRiLhs::START.checked_sub(TRiRhs::END).unwrap() }, { TRiLhs::END.checked_sub(TRiRhs::START).unwrap() }>>: Sized,
            {
                type Output =
                    $SrcTU<$RangeInclusive<{ TRiLhs::START.checked_sub(TRiRhs::END).unwrap() }, { TRiLhs::END.checked_sub(TRiRhs::START).unwrap() }>>;

                fn sub(self, rhs: $SrcTU<TRiRhs>) -> Self::Output {
                    // Given `value` is ranged-checked for both lhs & rhs, ∵ the sum of the bounds does not overflow,
                    // then the sum of the `value`s cannot overflow either.  Elide bounds checking.
                    #[allow(unsafe_code)]
                    Self::Output { value: unsafe { self.value().unchecked_sub(rhs.value()) } }
                }
            }

        )+
    }
}

impl_ranged_inclusive_sub!(
    i8 RiI8 RangeInclusiveI8, i16 RiI16 RangeInclusiveI16, i32 RiI32 RangeInclusiveI32, i64 RiI64 RangeInclusiveI64, i128 RiI128 RangeInclusiveI128, isize RiIsize RangeInclusiveIsize,
    u8 RiU8 RangeInclusiveU8, u16 RiU16 RangeInclusiveU16, u32 RiU32 RangeInclusiveU32, u64 RiU64 RangeInclusiveU64, u128 RiU128 RangeInclusiveU128, usize RiUsize RangeInclusiveUsize
);
