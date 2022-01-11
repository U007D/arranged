mod add;
// mod sub;
#[cfg(test)]
mod unit_tests;

use crate::{
    traits::{IRange, IRangeTo, IRangeFinite, IRangeFrom, IRangeToInclusive},
};

macro_rules! impl_range_inclusive {
    ($($ValueType:ident $RangeName:ident,)+) => {
        $(
            #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct $RangeName<const START: $ValueType, const END: $ValueType>;

            impl<const START: $ValueType, const END: $ValueType> IRangeFrom for $RangeName<START, END> {
                type IntoIter = IntoIter<$ValueType>;

                fn into_iter() -> Self::IntoIter {
                    Self::IntoIter { iter: Self::START..=Self::END }
                }

                fn start() -> <Self as IRange>::ValueType {
                    Self::START
                }
            }

            impl<const START: $ValueType, const END: $ValueType> IRangeTo for $RangeName<START, END> {
                fn end() -> <Self as IRange>::ValueType {
                    Self::END
                }
            }

            impl<const START: $ValueType, const END: $ValueType> IRangeToInclusive for $RangeName<START, END> {}
        )+
    }
}

impl_range_inclusive!(
    i8    RiI8,
    i16   RiI16,
    i32   RiI32,
    i64   RiI64,
    i128  RiI128,
    isize RiIsize,
    u8    RiU8,
    u16   RiU16,
    u32   RiU32,
    u64   RiU64,
    u128  RiU128,
    usize RiUsize,
);
