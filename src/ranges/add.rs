#[cfg(test)]
mod unit_tests;

use super::*;
use crate::ranges::{
    RangeInclusiveI128, RangeInclusiveI16, RangeInclusiveI32, RangeInclusiveI64, RangeInclusiveI8, RangeInclusiveIsize,
    RangeInclusiveU128, RangeInclusiveU16, RangeInclusiveU32, RangeInclusiveU64, RangeInclusiveU8, RangeInclusiveUsize,
};
use std::ops::Add;

macro_rules! impl_ranged_inclusive_add {
    ($($InnerType:ident $RangedInclusive:ident $RangeInclusive:ident),+) => {
        $(
            // ∵ ranged bounds are always const, use of `unwrap()` in `const` context does not panic, but rather halts
            // compilation in the event of overflow.  This is exactly the behavior we want to ensure compile-time
            // verification of bounds and zero runtime overhead.  `checked_add()` is used to ensure compilation halts on
            // overflow regardless of build settings (may be unnecessary).  See
            // https://stackoverflow.com/a/66362584/1541330 for an explanation of why the `where` clause is necessary.
            #[allow(clippy::unwrap_used)]
            impl<const START_L: $InnerType, const END_L: $InnerType, const START_R: $InnerType, const END_R: $InnerType> Add<$RangedInclusive<$RangeInclusive<START_R, END_R>>> for $RangedInclusive<$RangeInclusive<START_L, END_L>>
                where
                    $RangeInclusive<START_L, END_L>: IRangeInclusiveEnd<$InnerType> + IRangeFinite<$InnerType>,
                    $RangeInclusive<START_R, END_R>: IRangeInclusiveEnd<$InnerType> + IRangeFinite<$InnerType>,
                    $RangedInclusive<$RangeInclusive<{ START_L.checked_add(START_R).unwrap() }, { END_L.checked_add(END_R).unwrap() }>>: Sized,
            $RangeInclusive<{ START_L.checked_add(START_R).unwrap() }, { END_L.checked_add(END_R).unwrap() }>:  IRangeInclusiveEnd<$InnerType> + IRangeFinite<$InnerType> {
                type Output =
                    $RangedInclusive<$RangeInclusive<{ START_L.checked_add(START_R).unwrap() }, { END_L.checked_add(END_R).unwrap() }>>;

                fn add(self, rhs: $RangedInclusive<$RangeInclusive<START_R, END_R>>) -> Self::Output {
                    // Given `value` is ranged-checked for both lhs & rhs, ∵ the sum of the bounds does not overflow,
                    // then the sum of the `value`s cannot overflow either.  Elide bounds checking.
                    #[allow(unsafe_code)]
                    Self::Output::from_const(unsafe { self.value().unchecked_add(rhs.value()) })
                }
            }
        )+
    }
}

impl_ranged_inclusive_add!(
    i8 RiI8 RangeInclusiveI8, i16 RiI16 RangeInclusiveI16, i32 RiI32 RangeInclusiveI32, i64 RiI64 RangeInclusiveI64, i128 RiI128 RangeInclusiveI128, isize RiIsize RangeInclusiveIsize,
    u8 RiU8 RangeInclusiveU8, u16 RiU16 RangeInclusiveU16, u32 RiU32 RangeInclusiveU32, u64 RiU64 RangeInclusiveU64, u128 RiU128 RangeInclusiveU128, usize RiUsize RangeInclusiveUsize
);
