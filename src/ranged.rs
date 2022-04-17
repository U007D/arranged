mod i_min_max;
#[cfg(test)]
mod unit_tests;

use std::{
    fmt::Debug,
    ops::{Add, Div, Sub},
};

use arith_traits::{IMinMax, IUnaryWrappingOps, IWrappingOps};
use num_traits::{NumOps, One, Zero};

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
    /// with a runtime value.  In such a case, it will panic if the provided value is out of bounds.  When writing
    /// panic-free code, use the `try_from()` constructor instead.
    ///
    /// # Returns
    /// `Self`, if `value` is within `TRange`'s range bounds.
    ///
    /// # Panics
    /// If `value` is not within `TRange`'s range bounds:
    ///     * fails to compile if `value` is `const` (or a literal), or
    ///     * panics at runtime if `value` is not `const` (prefer `try_from()` constructor instead).
    #[must_use]
    #[allow(clippy::let_unit_value, clippy::no_effect_underscore_binding)]
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
    /// # Returns
    /// Returns `Some(Self)` when `value` is within bounds or `None` otherwise.
    #[allow(clippy::let_unit_value, clippy::no_effect_underscore_binding)]
    pub const fn try_from(value: TRange::ValueType) -> Result<Self>
    where
        TRange: ~const IRange + ~const IRangeFrom + ~const IRangeTo, {
        let _invariants = TRange::INVARIANTS;

        match TRange::contains(&value) {
            true => Ok(Self(value)),
            false => Err(Error::ValueOutOfInclusiveBounds(
                <TRange as IRangeFrom>::start().into(),
                <TRange as IRangeTo>::end().into(),
                value.into(),
            )),
        }
    }

    /// Constructor
    /// For orthogonality with this type's other constructors, this constructor may be called from `const` context.
    ///
    /// # Returns
    /// `Self`, unconditionally.
    ///
    /// # Safety
    /// This constructor is `unsafe` because the value being passed in is not verified to be within the bounds of
    /// `TRange`.  It is the caller's responsibility to ensure that this contract is upheld--violating it is Undefined
    /// Behavior.
    #[allow(unsafe_code)]
    #[must_use]
    pub const unsafe fn unchecked_from(value: TRange::ValueType) -> Self { Self(value) }
}

// TODO: Separate `value()` from `start()` and `end()` methods, since `IRangeFrom` and `IRangeTo(Inclusive)` are not
//       required for `value()`, but are for `start()` and `end()`, respectively.
// TODO: Separate `start()` and `end()` methods to accommodate `RangeTo` and `RangeFrom` types, respectively?
// TODO: Reconcile `start()` and `end()` methods with `IMinMax` trait to eliminate redundancy
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

impl<TRangeLhs, TRangeRhs> IWrappingOps<Ranged<TRangeRhs>> for Ranged<TRangeLhs>
where
    ErrInt: From<TRangeLhs::ValueType>,
    Self: PartialOrd,
    TRangeLhs: Clone + IMinMax<TRangeLhs::ValueType> + IRangeFrom + IRangeToInclusive + PartialOrd,
    TRangeLhs::ValueType: Debug + PartialOrd + IWrappingOps,
    TRangeLhs::WidenedValueType: Add<TRangeRhs::WidenedValueType, Output = TRangeLhs::WidenedValueType>
        + Clone
        + Debug
        + Div<Output = TRangeLhs::WidenedValueType>
        + IWrappingOps<Output = <TRangeLhs as IRange>::WidenedValueType>
        + NumOps<<TRangeLhs as IRange>::WidenedValueType, <TRangeLhs as IRange>::WidenedValueType>
        + One
        + PartialOrd
        + Sub<TRangeRhs::WidenedValueType, Output = TRangeLhs::WidenedValueType>
        + Zero,
    TRangeRhs: IRangeFrom + IRangeToInclusive,
{
    // TODO: Determine why arithmetic below compiles without having to `#[allow(clippy::integer_arithmetic)]` (!)
    fn wrapping_add(self, rhs: Ranged<TRangeRhs>) -> Self::Output {
        let sum = TRangeLhs::WidenedValueType::from(self.0) + TRangeRhs::WidenedValueType::from(rhs.0)
            - TRangeLhs::WidenedValueType::from(TRangeLhs::MIN);
        Self::from(wrapping_total::<TRangeLhs>(TRangeLhs::MIN, TRangeLhs::MAX, sum))
    }

    fn wrapping_div(self, _rhs: Ranged<TRangeRhs>) -> Self::Output { todo!() }

    fn wrapping_div_euclid(self, _rhs: Ranged<TRangeRhs>) -> Self::Output { todo!() }

    fn wrapping_mul(self, _rhs: Ranged<TRangeRhs>) -> Self::Output { todo!() }

    fn wrapping_rem(self, _rhs: Ranged<TRangeRhs>) -> Self::Output { todo!() }

    fn wrapping_rem_euclid(self, _rhs: Ranged<TRangeRhs>) -> Self::Output { todo!() }

    fn wrapping_sub(self, rhs: Ranged<TRangeRhs>) -> Self::Output {
        let diff = TRangeLhs::WidenedValueType::from(self.0)
            - TRangeRhs::WidenedValueType::from(rhs.0)
            - TRangeLhs::WidenedValueType::from(TRangeLhs::MIN);
        Self::from(wrapping_total::<TRangeLhs>(TRangeLhs::MIN, TRangeLhs::MAX, diff))
    }
}

impl<TRangeLhs> IUnaryWrappingOps for Ranged<TRangeLhs>
where
    Self: PartialOrd,
    TRangeLhs: Clone + IMinMax<TRangeLhs::ValueType> + IRangeFrom + IRangeToInclusive,
    TRangeLhs::ValueType: PartialOrd + IWrappingOps,
    <TRangeLhs as IRange>::WidenedValueType: IWrappingOps<Output = <TRangeLhs as IRange>::WidenedValueType>
        + Clone
        + NumOps<<TRangeLhs as IRange>::WidenedValueType, <TRangeLhs as IRange>::WidenedValueType>
        + One
        + PartialOrd
        + Zero,
{
    type Output = Self;

    fn wrapping_abs(self) -> Self::Output {
        let (start, end) = (
            <TRangeLhs as IRange>::WidenedValueType::from(<TRangeLhs as IRangeFrom>::start()),
            <TRangeLhs as IRange>::WidenedValueType::from(<TRangeLhs as IRangeTo>::end()),
        );

        // `WidenedValueType::from(self.0).abs()` cannot wrap, so `wrapping_abs()` is being used as
        // "plain `abs`".  Note there is no convenient "plain `abs`" for `ValueType` as (`num::abs<T>(T)` is only
        // defined on `T: Signed` (i.e. `f32`, `f64` and `Rational`).
        // TODO: Use `abs` or `wrapping_abs`. Attempts to use `WidenedValueType.wrapping_abs()` seem to enter infinite
        //       recursive loops (resolving `IWrapping`?).
        let abs_value = {
            let tmp_value = <TRangeLhs as IRange>::WidenedValueType::from(self.0);
            match tmp_value >= <TRangeLhs as IRange>::WidenedValueType::zero() {
                true => tmp_value,
                false => <TRangeLhs as IRange>::WidenedValueType::zero() - tmp_value,
            }
        };

        // If `self.contains(&abs_value)` then return `Ranged(abs_value)`, otherwise
        // compute `abs_value`'s wrapped offset into the range and return `Range(wrapped_offset)`.
        let working_value = match abs_value <= end {
            true => abs_value,
            false => {
                // Implementation notes and sample walkthrough:
                // 0..=255; n(∈) = 256_u8 (boom!)
                // 0..=255; Option<n(∈) - 1> = None..=Some(255_u8)
                // -128_i8..=127_i8; n(∈) = 256_i8 (boom!)

                // -30_i8..=-20_i8; n(∈) = 11_i16 ✅
                // |-25| = 25_i16 ✅
                // 25_i16 - -20_i16 = 45_i16 ✅
                // 45_i16 % 11_i16 = 1_i8 ✅
                // -30_i8 + 1_i8 = -29_i8 ✅

                // Arithmetic below is safe because span of bounds of `ValueType`
                // (`<TRange as IRangeTo>::end() - <TRange as IRangeTo>::start()`) cannot overflow `ValueType`,
                // let alone `WidenedValueType`, whose size is at least `2 * size_of::<ValueType>()`.
                // `end - start` + 1 (required for inclusive `Range`) _could_ overflow, so `ValueType` has been promoted
                // to `WidenedValueType` to ensure overflow cannot happen in this case either.
                #[allow(clippy::integer_arithmetic)]
                let range_len = end.clone() - start.clone() + <TRangeLhs as IRange>::WidenedValueType::one();

                // `abs_value > end` per `match` arm ensures that `abs_value - end` cannot overflow
                #[allow(clippy::integer_arithmetic)]
                let overflow_magnitude = abs_value - end;

                let range_offset = overflow_magnitude % range_len;
                start + range_offset
            },
        };

        // `range_offset` calculation above guarantees `working_value` is within `TRange::start()..=TRange::end()`.
        // TODO: Since `TryFrom/TryInto` do not require the returned `Result`'s `E` to be `Error` or even `Debug`,
        //       determine how to output `err` to aid debuggability
        Self(working_value.try_into().unwrap_or_else(|_err| unreachable!()))
    }

    fn wrapping_neg(self) -> Self::Output { todo!() }

    fn wrapping_pow(self, _rhs: u32) -> Self::Output { todo!() }

    fn wrapping_shl(self, _rhs: u32) -> Self::Output { todo!() }

    fn wrapping_shr(self, _rhs: u32) -> Self::Output { todo!() }
}

#[allow(clippy::inline_always)]
#[inline(always)]
fn wrapping_total<TRange>(
    start: TRange::ValueType,
    end: TRange::ValueType,
    total: TRange::WidenedValueType,
) -> TRange::ValueType
where
    TRange: IRange,
    TRange::WidenedValueType: Add<TRange::WidenedValueType, Output = TRange::WidenedValueType>
        + Clone
        + IUnaryWrappingOps<Output = TRange::WidenedValueType>
        + One
        + Sub<TRange::WidenedValueType, Output = TRange::WidenedValueType>, {
    let (start, end) = (TRange::WidenedValueType::from(start), TRange::WidenedValueType::from(end));
    // For `Copy` types, `clone()` is `memcpy::copy_*()`; for non-`Copy` types (uncommon), `Clone` bound permits
    //     ranged type participation
    let span = end - start.clone() + <TRange as IRange>::WidenedValueType::one();
    let offset = total.wrapping_rem_euclid(span);
    // TODO: Since `TryFrom/TryInto` do not require the returned `Result`'s `E` to be `Error` or even `Debug`, determine
    //       how to output `err` to aid debuggability
    (start + offset).try_into().unwrap_or_else(|_err| unreachable!())
}
