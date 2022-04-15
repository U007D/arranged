use std::mem::{size_of, transmute};

use arith_traits::IMinMax;
use num::BigInt;

use into_iter_ri::IntoIterRi;

use crate::traits::{IRange, IRangeFinite, IRangeFrom, IRangeIntoIterator, IRangeTo, IRangeToInclusive, ITyEq};

mod into_iter_ri;
#[cfg(test)]
mod unit_tests;

macro_rules! impl_range_inclusive {
    ($($ValueType:ident $UnsignedValueType:ident $WorkingValueType:ident $RangeName:ident,)+) => {
        $(
            #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct $RangeName<const START: $ValueType, const END: $ValueType>;

            /// Rust will evaluate (at compile-time) `consts` which it determines are in an execution path.  Even though
            /// `drop()` is not guaranteed to be run (e.g. if `$RangeName` is used only in a static context) the
            /// existence of this `Drop` impl is enough to trigger compile-time evaluation of the invariant, which has
            /// the effect of enforcing its contract at compile-time.
            // TODO: When this invariant is violated, the compiler will report that it *has* been broken, but will not
            //       reveal *where* the violation occurred.  Look for alternative invariant enforcement schemes with
            //       better debug experiences (note: the `Ri*` types are designed to be useful without any methods being
            //       called).
            impl<const START: $ValueType, const END: $ValueType> Drop for $RangeName<START, END> {
                #[allow(clippy::let_unit_value)]
                fn drop(&mut self) { let _invariants = Self::INVARIANTS; }
            }

            impl<const START: $ValueType, const END: $ValueType> IntoIterator for $RangeName<START, END> {
                type IntoIter = IntoIterRi<$ValueType>;
                type Item = $ValueType;

                fn into_iter(self) -> IntoIterRi<$ValueType> { IntoIterRi::<$ValueType> { range: START..=END } }
            }

            impl<const START: $ValueType, const END: $ValueType> const IMinMax<$ValueType> for $RangeName<START, END> {
                const MAX: $ValueType = END;
                const MIN: $ValueType = START;
            }


            impl<const START: $ValueType, const END: $ValueType> const IRange for $RangeName<START, END> {
                const INVARIANTS: () = assert!(START <= END);
                type ValueType = $ValueType;
                type WorkingValueType = $WorkingValueType;

                fn contains(value: &Self::ValueType) -> bool { *value >= START && *value <= END }
            }

            #[allow(clippy::useless_transmute)]
            impl<const START: $ValueType, const END: $ValueType> const IRangeFinite<$ValueType> for $RangeName<START, END>
            where
                ($ValueType, Self::ValueType): ITyEq,
            {
                // Inclusive range where `START <= END` can never be empty
                #[allow(clippy::inline_always)]
                #[inline(always)]
                fn is_empty() -> bool { false }

                /// Compute magnitude of `START..=END` span.  Note that this difference may overflow if `START`, `END`
                /// are signed types (and thus typically the computed difference is also signed).  In such a case, the
                /// difference could overflow `SignedType` (e.g. `SignedType::MAX` - `SignedType::MIN` will overflow
                /// `SignedType`).
                ///
                /// To avoid this situation, this algorithm leverages (`unsafe` aka "manually-verified-to-be-correct")
                /// twos-complement arithmetic to perform a signed difference computation yielding an *unsigned* result,
                /// which can no longer overflow.
                ///
                /// Then the result is increased by one (since `Ri*` is inclusive of `END`), which *could* overflow the
                /// return type when `size_of_val(&START)` or `size_of_val(&END)` `>=` `size_of::<usize>()` and the span
                /// is very large.  This function returns `Option` where a value of `None` represents this overflow.
                #[allow(unsafe_code)]
                fn len() -> Option<usize> {
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

            impl<const START: $ValueType, const END: $ValueType> const IRangeFrom for $RangeName<START, END> {
                fn start() -> <Self as IRange>::ValueType { START }
            }

            impl<const START: $ValueType, const END: $ValueType> IRangeIntoIterator for $RangeName<START, END> {
                type IntoIter = IntoIterRi<$ValueType>;

                fn into_iter() -> <Self as IRangeIntoIterator>::IntoIter {
                    IntoIterator::into_iter(Self)
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
    i8    u8    i16             RiI8,
    i16   u16   i32             RiI16,
    i32   u32   i64             RiI32,
    i64   u64   i128            RiI64,
    i128  u128  BigInt          RiI128,
    isize usize BigInt          RiIsize,
    u8    u8    i16             RiU8,
    u16   u16   i32             RiU16,
    u32   u32   i64             RiU32,
    u64   u64   i128            RiU64,
    u128  u128  BigInt          RiU128,
    usize usize BigInt          RiUsize,
);
