use crate::{ErrIntPrimIntExt, Error, Result};

macro_rules! impl_ranged_inclusive {
    ($($SrcTL:ident $SrcTU:ident),+) => {
        $(
            #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct $SrcTU<const START: $SrcTL, const END: $SrcTL> {
                value: $SrcTL,
            }

            impl<const START: $SrcTL, const END: $SrcTL> $SrcTU<START, END> {
                const _INVARIANT_START_LE_END: () = assert!(START <= END);

                // Suppress false positive 'associated function is never used: `new`'
                #[allow(dead_code)]
                pub const fn new(value: $SrcTL) -> Result<Self> {
                    match START <= value && END >= value {
                        true => Ok(Self { value }),
                        false => Err(Error::ValueOutOfInclusiveBounds(START.to_err_int(),
                            END.to_err_int(), value.to_err_int())),
                    }
                }
            }
        )+
    }
}

impl_ranged_inclusive!(
    i8 RiI8, i16 RiI16, i32 RiI32, i64 RiI64, i128 RiI128, isize RiIsize,
    u8 RiU8, u16 RiU16, u32 RiU32, u64 RiU64, u128 RiU128, usize RiUsize
);
