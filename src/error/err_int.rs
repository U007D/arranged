mod prim_int_ext;
mod signed;
mod unsigned;

pub use prim_int_ext::ErrIntPrimIntExt;
pub use signed::Signed;
pub use unsigned::Unsigned;

// This type allows any primitive integer type to be included as payload with `Error` types carrying integer values,
// without having to make the `Error` type generic (which has viral ergonomic issues) or with `Box`ing (which requires
// heap allocation and may not be compatible with `no_std` environments.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrInt {
    Signed(Signed),
    Unsigned(Unsigned),
}

macro_rules! impl_from_for_err_int {
    ($($Signed:ident: $($SrcTL:ident $SrcTU:ident),+);+) => {
        $(
            $(
                impl const ErrIntPrimIntExt for $SrcTL {
                    fn to_err_int(self) -> ErrInt {
                        ErrInt::$Signed($Signed::$SrcTU(self))
                    }
                }

                impl const From<$SrcTL> for ErrInt {
                    fn from(src: $SrcTL) -> Self {
                        src.to_err_int()
                    }
                }
            )+
        )+
    }
}

impl_from_for_err_int!(
    Signed:   i8 I8, i16 I16, i32 I32, i64 I64, i128 I128, isize Isize;
    Unsigned: u8 U8, u16 U16, u32 U32, u64 U64, u128 U128, usize Usize
);
