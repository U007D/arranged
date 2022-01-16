use std::{iter::Step, ops::RangeInclusive};

#[derive(Debug)]
pub struct IntoIterRi<TValueType>
where
    TValueType: PartialOrd, {
    pub(super) range: RangeInclusive<TValueType>,
}

impl<TValueType> Iterator for IntoIterRi<TValueType>
where
    TValueType: PartialOrd + Step,
{
    type Item = TValueType;

    fn next(&mut self) -> Option<Self::Item> { self.range.next() }
}
