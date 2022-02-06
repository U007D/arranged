#[cfg(test)]
mod unit_tests;

use std::{iter::Step, ops::RangeInclusive};

#[derive(Debug)]
pub struct IntoIterRi<TValue>
where
    TValue: PartialOrd, {
    pub(super) range: RangeInclusive<TValue>,
}

impl<TValue> Iterator for IntoIterRi<TValue>
where
    TValue: PartialOrd + Step,
{
    type Item = TValue;

    fn next(&mut self) -> Option<Self::Item> { self.range.next() }
}
