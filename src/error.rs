mod err_int;

use crate::consts::*;
pub use err_int::{ErrInt, ErrIntPrimIntExt};
use std::fmt::Debug;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}: {}..={}.", msg::ERR_INVALID_RANGE_INCLUSIVE_BOUNDS, 0, 1)]
    InvalidRangeInclusiveBounds(ErrInt, ErrInt),
    #[error("{}: {}..={} {} {:?}.", msg::ERR_VALUE_OUT_OF_INCLUSIVE_BOUNDS, 0, 1, msg::DOES_NOT_CONTAIN, 2)]
    ValueOutOfInclusiveBounds(ErrInt, ErrInt, ErrInt),
}
