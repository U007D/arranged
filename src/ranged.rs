#[cfg(test)]
mod unit_tests;

use crate::{
    consts::msg,
    traits::{IRange, IRangeFrom, IRangeTo, IRangeToInclusive, IRanged},
    ErrInt, Error, Result,
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Ranged<TRange>(TRange::ValueType)
where
    TRange: IRange;

impl<TRange> Ranged<TRange>
where
    TRange: IRange + IRangeFrom + IRangeToInclusive,
    ErrInt: From<TRange::ValueType>,
{
    /// Constructor
    /// This constructor is intended to be called from `const` context.  This way, if the value passed in is
    /// out of bounds, the compilation will fail.  If the code runs, it means the value passed in was within
    /// bounds and therefore the function signature is infallible (at runtime).  NOTE: Rust does not provide
    /// a way to *only* provide a (`const`) method at compile-time, thus, this function can also be called
    /// with a runtime value.  In such a case, it will panic if the provided value is out of bounds.
    ///
    /// # Panics
    /// Returns `Self` when `value` is within bounds, otherwise:
    ///     * fails to compile if `value` is `const` (or a literal), or
    ///     * panics at runtime if `value` is not `const` (prefer `try_from()` constructor instead).
    #[must_use]
    pub const fn from(value: TRange::ValueType) -> Self
    where
        TRange: ~const IRange + ~const IRangeFrom + ~const IRangeTo, {
        #[allow(clippy::match_wild_err_arm)]
        // TODO: Replace with `const` `expect()` once it exists
        match Self::try_from(value) {
            Ok(instance) => instance,
            // TODO: Note loss of `err` context; restore when `const` `format!` or other solution is found
            Err(_err) => panic!("{}", msg::ERR_VALUE_OUT_OF_INCLUSIVE_BOUNDS),
        }
    }

    /// Constructor
    /// Returns `Some(Self)` when `value` is within bounds or `None` otherwise.
    pub const fn try_from(value: TRange::ValueType) -> Result<Self>
    where
        TRange: ~const IRange + ~const IRangeFrom + ~const IRangeTo, {
        match TRange::contains(&value) {
            true => Ok(Self(value)),
            false => Err(Error::ValueOutOfInclusiveBounds(
                <TRange as IRangeFrom>::start().into(),
                <TRange as IRangeTo>::end().into(),
                value.into(),
            )),
        }
    }
}

impl<TRange> const IRanged<TRange> for Ranged<TRange>
where
    TRange: ~const IRange + ~const IRangeFrom + ~const IRangeTo + ~const IRangeToInclusive,
{
    #[must_use]
    fn end(&self) -> TRange::ValueType { TRange::end() }

    #[must_use]
    fn start(&self) -> TRange::ValueType { TRange::start() }

    #[must_use]
    fn value(&self) -> &TRange::ValueType { &self.0 }
}
