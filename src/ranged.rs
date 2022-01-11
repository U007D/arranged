mod into_iter;

use crate::{
    consts::msg,
    traits::{IRange, IRangeFrom, IRangeTo, IRangeToInclusive},
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Ranged<TRange>(TRange::ValueType) where TRange: IRange;

impl < const START: $ ValueType, const END: $ ValueType> $ RangedInclusive<START, END> {
/// Constructor
/// Returns `Some(Self)` when `value` is within bounds or `None` otherwise.
// Suppress false positive 'associated function is never used'
# [allow(dead_code)]
# [must_use]
pub const fn from_value(value: $ ValueType) -> Option < Self > {
match value > = Self::START & & value < = Self::END {
true => Some( Self (value)),
false => None,
}
}

/// This constructor is intended to be called from `const` context.  This way, if the value passed in is
/// out of bounds, the compilation will fail.  If the code runs, it means the value passed in was within
/// bounds and therefore the function signature is infallible (at runtime).  NOTE: Rust does not provide
/// a way to *only* provide a (`const`) method at compile-time, thus, this function can also be called
/// with a runtime value.  In such a case, it will panic if the provided value is out of bounds.
///
/// Returns `Self` when `value` is within bounds, otherwise:
///     * fails to compile if `value` is `const` (or a literal), or
///     * panics at runtime if `value` is not `const` (prefer `from_value()` constructor instead).
// Suppress false positive 'associated function is never used'
# [allow(dead_code)]
# [must_use]
pub const fn from_const(value: $ ValueType) -> Self {
assert ! (value > = Self::START && value < = Self::END, "{}", msg::ERR_VALUE_OUT_OF_INCLUSIVE_BOUNDS);
Self { value }
}

# [must_use]
pub const fn value( &self ) -> $ ValueType {
self.value
}
}

impl < const START: $ ValueType, const END: $ ValueType> IRange for $ RangedInclusive<START, END> {
const fn contains(value: < Self as IRange>::ValueType) -> bool {
value > = Self::START & & value <= Self::END
}
}
