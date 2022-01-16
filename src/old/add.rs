// #[cfg(test)]
// mod unit_tests;

use super::*;
use crate::Ranged;
use std::ops::Add;

macro_rules! impl_ranged_inclusive_add {
    ($($ValueType:ident $RangeName:ident),+) => {
        $(
            // ∵ ranged bounds are always const, use of `unwrap()` in `const` context does not panic, but rather halts
            // compilation in the event of overflow.  This is exactly the behavior we want to ensure compile-time
            // verification of bounds and zero runtime overhead.  `checked_add()` is used to ensure compilation halts on
            // overflow regardless of build settings (may be unnecessary).  See
            // https://stackoverflow.com/a/66362584/1541330 for an explanation of why the `where` clause is necessary.
            #[allow(clippy::unwrap_used)]
            impl<const START_L: $ValueType, const END_L: $ValueType, const START_R: $ValueType, const END_R: $ValueType> Add<$RangedInclusive<$RangeName<START_R, END_R>>> for $RangedInclusive<$RangeName<START_L, END_L>>
                where
                    $RangeName<START_L, END_L>: IRangeInclusiveEnd<$ValueType> + IRangeFinite<$ValueType>,
                    $RangeName<START_R, END_R>: IRangeInclusiveEnd<$ValueType> + IRangeFinite<$ValueType>,
                    Ranged<$RangeName<{ START_L.checked_add(START_R).unwrap() }, { END_L.checked_add(END_R).unwrap() }>>: Sized,
            $RangeName<{ START_L.checked_add(START_R).unwrap() }, { END_L.checked_add(END_R).unwrap() }>:  IRangeInclusiveEnd<$ValueType> + IRangeFinite<$ValueType> {
                type Output =
                    Ranged<$RangeName<{ START_L.checked_add(START_R).unwrap() }, { END_L.checked_add(END_R).unwrap() }>>;

                fn add(self, rhs: Ranged<$RangeName<START_R, END_R>>) -> Self::Output {
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
    i8 RiI8, i16 RiI16, i32 RiI32, i64 RiI64, i128 RiI128, isize RiIsize,
    u8 RiU8, u16 RiU16, u32 RiU32, u64 RiU64, u128 RiU128, usize RiUsize
);
