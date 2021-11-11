use crate::ErrInt;

pub trait ErrIntPrimIntExt {
    fn to_err_int(self) -> ErrInt;
}
