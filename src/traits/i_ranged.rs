use crate::traits::{IRange, IRangeFrom, IRangeToInclusive};

pub trait IRanged<TRange>
where
    TRange: IRange + IRangeFrom + IRangeToInclusive, {
    fn end(&self) -> TRange::ValueType;
    fn start(&self) -> TRange::ValueType;
    fn value(&self) -> &TRange::ValueType;
}
