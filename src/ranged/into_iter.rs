macro_rules! impl_range_inclusive_into_iter {
    ($($InnerType:ident $RangeInclusive:ident $IntoIter:ident),+) => {
        $(
            #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct $IntoIter {
                current: $InnerType,
                end:     $InnerType,
            }

            impl $IntoIter {
                #[must_use]
                pub const fn new(start: $InnerType, end: $InnerType) -> Self { Self { current: start, end } }
            }

            impl Iterator for $IntoIter {
                type Item = $InnerType;

                fn next(&mut self) -> Option<Self::Item> {
                    // Since `match` expression assures `current` < `end` and both are of the same time, `current + 1`
                    // cannot overflow.
                    #[allow(clippy::integer_arithmetic)]
                    match self.current < self.end {
                        true => {
                            let tmp = self.current;
                            self.current += 1;
                            Some(tmp)
                        },
                        false => None,
                    }
                }
            }
        )+
    }
}

impl_range_inclusive_into_iter!(
    i8    RangeInclusiveI8    IntoIterI8,
    i16   RangeInclusiveI16   IntoIterI16,
    i32   RangeInclusiveI32   IntoIterI32,
    i64   RangeInclusiveI64   IntoIterI64,
    i128  RangeInclusiveI128  IntoIterI128,
    isize RangeInclusiveIsize IntoIterIsize,
    u8    RangeInclusiveU8    IntoIterU8,
    u16   RangeInclusiveU16   IntoIterU16,
    u32   RangeInclusiveU32   IntoIterU32,
    u64   RangeInclusiveU64   IntoIterU64,
    u128  RangeInclusiveU128  IntoIterU128,
    usize RangeInclusiveUsize IntoIterUsize
);
