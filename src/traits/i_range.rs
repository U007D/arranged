use crate::traits::ITyEq;
use num_traits::SaturatingSub;

pub trait IRange {
    type ValueType: PartialOrd;
    fn contains(value: Self::ValueType) -> bool;
}

pub trait IRangeFrom: IRange + IntoIterator<Item=<Self as IRange>::ValueType> {
    fn start(&self) -> &<Self as IRange>::ValueType;
}

pub trait IRangeTo: IRange {
    fn end(&self) -> &<Self as IRange>::ValueType;
}

pub trait IRangeFinite: IRange + IRangeFrom + IRangeTo {}

pub trait IRangeToInclusive: IRangeFinite {}

pub trait IRangeLen<TValue>: IRange + IRangeFrom + IRangeTo
    where
        (TValue, <Self as IRange>::ValueType): ITyEq,
        TValue: SaturatingSub, {
    fn is_empty(&self) -> bool;
    fn len(&self) -> Option<usize>;
}
