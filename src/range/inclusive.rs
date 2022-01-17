mod into_iter_ri;
#[cfg(test)]
mod unit_tests;

use crate::traits::{IRange, IRangeFinite, IRangeFrom, IRangeTo, IRangeToInclusive, ITyEq};
use into_iter_ri::IntoIterRi;
use std::mem::{size_of, transmute};

macro_rules! impl_range_inclusive {
    ($($ValueType:ident $UnsignedValueType:ident $RangeName:ident,)+) => {
        $(
            #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct $RangeName<const START: $ValueType, const END: $ValueType>;

            impl<const START: $ValueType, const END: $ValueType> $RangeName<START, END> {
                const INVARIANT_START_LE_END: () = assert!(START <= END);
            }

            /// Rust will evaluate (at compile-time) `consts` which it determines are in an execution path.  Even though
            /// `drop()` is not guaranteed to be run (e.g. if `$RangeName` is used only in a static context) the
            /// existence of this `Drop` impl is enough to trigger compile-time evaluation of the invariant, which has
            /// the effect of enforcing its contract at compile-time.
            impl<const START: $ValueType, const END: $ValueType> Drop for $RangeName<START, END> {
                #[allow(clippy::let_unit_value)]
                fn drop(&mut self) { let () = Self::INVARIANT_START_LE_END; }
            }

            impl<const START: $ValueType, const END: $ValueType> IntoIterator for $RangeName<START, END> {
                type IntoIter = IntoIterRi<$ValueType>;
                type Item = $ValueType;

                fn into_iter(self) -> IntoIterRi<$ValueType> { IntoIterRi::<$ValueType> { range: START..=END } }
            }

            impl<const START: $ValueType, const END: $ValueType> const IRange for $RangeName<START, END> {
                type ValueType = $ValueType;

                fn contains(value: &Self::ValueType) -> bool { *value >= START && *value <= END }
            }

            impl<const START: $ValueType, const END: $ValueType> const IRangeFrom for $RangeName<START, END> {
                fn start() -> <Self as IRange>::ValueType { START }
            }

            #[allow(clippy::useless_transmute)]
            impl<const START: $ValueType, const END: $ValueType> const IRangeFinite<$ValueType> for $RangeName<START, END>
            where
                ($ValueType, Self::ValueType): ITyEq,
            {
                // Inclusive range where `START <= END` can never be empty
                #[allow(clippy::inline_always)]
                #[inline(always)]
                fn is_empty(&self) -> bool { false }

                /// Compute magnitude of `START..=END` span.  Note that this difference may overflow when `START`,
                /// `END` are signed and thus typically the computed difference is also a signed value.  In such a case,
                /// `SignedType::MAX` - `SignedType::MIN` will overflow `SignedType`.
                ///
                /// To avoid this situation, this algorithm leverages twos-complement arithmetic to perform a signed
                /// difference computation yielding an *unsigned* result, which can no longer overflow.
                ///
                /// The magnitude is then increased by one (since this range is inclusive of `END`), which *could*
                /// overflow the return type when
                /// `size_of_val(&START)` or `size_of_val(&END)` `>=` `size_of::<usize>()` and the span is very large.
                /// This is why this function returns `Option` where the value of `None` represents overflow.
                #[allow(unsafe_code)]
                fn len(&self) -> Option<usize> {
                    let offset = unsafe { transmute::<$ValueType, $UnsignedValueType>($ValueType::MIN) };
                    let u_start = unsafe { transmute::<$ValueType, $UnsignedValueType>(START) }.wrapping_add(offset);
                    let u_end = unsafe { transmute::<$ValueType, $UnsignedValueType>(END) }.wrapping_add(offset);

                    // Compute the span (cannot overflow, so `integer_arithmetic` is used), and if not already at
                    // `usize::MAX`, `checked_add(1)` to include end range value (this may overflow).
                    // Note: using closures e.g. `.unwrap_or_else(|| unreachable!())` gives error compiling in `const`
                    //       context, so using imperative `match` which is compatible with execution in `const` context.
                    // TODO: Update to use `const` `map()`, `unwrap_or_else()`/`expect()`/`unwrap()` once available
                    #[allow(clippy::integer_arithmetic, clippy::cast_possible_truncation, clippy::checked_conversions)]
                    match (size_of::<$UnsignedValueType>() < size_of::<usize>(), u_end - u_start) {
                        (true, diff) => (diff as usize).checked_add(1),
                        (false, diff) => match diff < usize::MAX as $UnsignedValueType {
                            true => (diff as usize).checked_add(1),
                            false => None,
                        }
                    }
                }
            }

            impl<const START: $ValueType, const END: $ValueType> const IRangeTo for $RangeName<START, END> {
                fn end() -> <Self as IRange>::ValueType { END }
            }

            impl<const START: $ValueType, const END: $ValueType> IRangeToInclusive for $RangeName<START, END> {}
        )+
    }
}

impl_range_inclusive!(
    i8    u8    RiI8,
    i16   u16   RiI16,
    i32   u32   RiI32,
    i64   u64   RiI64,
    i128  u128  RiI128,
    isize usize RiIsize,
    u8    u8    RiU8,
    u16   u16   RiU16,
    u32   u32   RiU32,
    u64   u64   RiU64,
    u128  u128  RiU128,
    usize usize RiUsize,
);
