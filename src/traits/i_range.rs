use std::ops::Sub;

use arith_traits::IWrappingOps;
use num_traits::{Num, SaturatingSub};

use crate::traits::ITyEq;

pub trait IRange {
    const INVARIANTS: () = ();
    type ValueType: Num + TryFrom<Self::WorkingValueType>;
    type WorkingValueType: From<Self::ValueType> + Sub + IWrappingOps;

    fn contains(value: &Self::ValueType) -> bool
    where
        Self: Sized;
}

pub trait IRangeFinite<TValue>: IRange + IRangeFrom + IRangeTo
where
    (TValue, <Self as IRange>::ValueType): ITyEq,
    TValue: SaturatingSub, {
    fn is_empty() -> bool;
    fn len() -> Option<usize>;
}

pub trait IRangeFrom: IRange {
    fn start() -> <Self as IRange>::ValueType;
}

pub trait IRangeIntoIterator: IRangeFrom + IntoIterator<Item = <Self as IRange>::ValueType> {
    type IntoIter: Iterator;
    fn into_iter() -> <Self as IRangeIntoIterator>::IntoIter;
}

pub trait IRangeTo: IRange {
    fn end() -> <Self as IRange>::ValueType;
}

pub trait IRangeToInclusive: IRangeTo {}
