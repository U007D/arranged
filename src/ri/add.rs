#[cfg(test)]
mod unit_tests;

use super::*;
use std::ops::Add;

macro_rules! impl_ranged_inclusive_add {
    ($($SrcTL:ident $SrcTU:ident),+) => {
        $(
            // ∵ range bounds are always const, use of `unwrap()` in `const` context does not panic, but rather halts
            // compilation in the event of overflow.  This is exactly the behavior we want to ensure compile-time
            // verification of bounds and zero runtime overhead.  `checked_add()` is used to ensure compilation halts on
            // overflow regardless of build settings (may be unnecessary).  See
            // https://stackoverflow.com/a/66362584/1541330 for an explanation of why the `where` clause is necessary.
            #[allow(clippy::unwrap_used)]
            impl<const START_LHS: $SrcTL, const END_LHS: $SrcTL, const START_RHS: $SrcTL, const END_RHS: $SrcTL>
                Add<$SrcTU<START_RHS, END_RHS>> for $SrcTU<START_LHS, END_LHS>
            where
                $SrcTU<{ START_LHS.checked_add(START_RHS).unwrap() }, { END_LHS.checked_add(END_RHS).unwrap() }>: Sized,
            {
                type Output =
                    $SrcTU<{ START_LHS.checked_add(START_RHS).unwrap() }, { END_LHS.checked_add(END_RHS).unwrap() }>;

                fn add(self, rhs: $SrcTU<START_RHS, END_RHS>) -> Self::Output {
                    // Given `value` is range-checked for both lhs & rhs, ∵ the sum of the bounds does not overflow,
                    // then the sum of the `value`s cannot overflow either.  Elide bounds checking.
                    #[allow(unsafe_code)]
                    Self::Output { value: unsafe { self.value().unchecked_add(rhs.value()) } }
                }
            }

        )+
    }
}

impl_ranged_inclusive_add!(
    i8 RiI8, i16 RiI16, i32 RiI32, i64 RiI64, i128 RiI128, isize RiIsize,
    u8 RiU8, u16 RiU16, u32 RiU32, u64 RiU64, u128 RiU128, usize RiUsize
);
