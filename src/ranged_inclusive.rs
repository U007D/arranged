use crate::{ErrIntPrimIntExt, Error, Result};

macro_rules! impl_ranged_inclusive {
    ($($SrcTL:ident $SrcTU:ident),+) => {
        $(
            #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct $SrcTU<const START: $SrcTL, const END: $SrcTL> {
                value: $SrcTL,
            }

            impl<const START: $SrcTL, const END: $SrcTL> $SrcTU<START, END> {
                // Suppress false positive 'associated function is never used: `new`'
                #[allow(dead_code)]
                pub const fn new(value: $SrcTL) -> Result<Self> {
                    match START <= value && END >= value {
                        true => Ok(Self { value }),
                        false => Err(Error::ValueOutOfInclusiveBounds(START.to_err_int(),
                            END.to_err_int(),
                            value.to_err_int())),
                    }
                }
            }
        )+
    }
}

impl_ranged_inclusive!(
    i8 RangedInclusiveI8, i16 RangedInclusiveI16, i32 RangedInclusiveI32,
    i64 RangedInclusiveI64, i128 RangedInclusiveI128, isize RangedInclusiveIsize,
    u8 RangedInclusiveU8, u16 RangedInclusiveU16, u32 RangedInclusiveU32,
    u64 RangedInclusiveU64, u128 RangedInclusiveU128, usize RangedInclusiveUsize
);
