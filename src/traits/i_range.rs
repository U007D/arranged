pub trait IRange {
    type ValueType: PartialOrd;
    fn contains(value: Self::ValueType) -> bool;
}

pub trait IRangeFrom: IRange {
    type IntoIter: IntoIterator<Item=<Self as IRange>::ValueType>;

    fn into_iter(self) -> Self::IntoIter;
    fn start(&self) -> &<Self as IRange>::ValueType;
}

pub trait IRangeTo: IRange {
    fn end(&self) -> &<Self as IRange>::ValueType;
}

pub trait IRangeFinite: IRange + IRangeFrom + IRangeTo {
    fn len(&self) -> <Self as IRange>::ValueType;
    fn is_empty(&self) -> bool;
}

pub trait IRangeToInclusive: IRangeFinite {}
