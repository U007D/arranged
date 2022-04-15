use std::{
    fmt::Debug,
    ops::{Add, Div, Sub},
};

use arith_traits::{IMinMax, IWrappingNonGenericOps, IWrappingOps};
use num_traits::{NumOps, One, Zero};

use crate::{
    consts::msg,
    ErrInt,
    Error, Result, traits::{IRange, IRanged, IRangeFrom, IRangeTo, IRangeToInclusive},
};

mod i_min_max;
#[cfg(test)]
mod unit_tests;

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
            TRange: ~ const IRange + ~ const IRangeFrom + ~ const IRangeTo, {
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
            TRange: ~ const IRange + ~ const IRangeFrom + ~ const IRangeTo, {
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
//       required for the former.
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
        TRangeLhs: IMinMax<TRangeLhs::ValueType> + IRangeFrom + IRangeToInclusive + PartialOrd,
        TRangeLhs::ValueType: Debug + PartialOrd + IWrappingOps,
        TRangeLhs::WorkingValueType: Add<TRangeRhs::WorkingValueType, Output=TRangeLhs::WorkingValueType>
        + Debug
        + Div<Output=TRangeLhs::WorkingValueType>,
        for<'a> &'a TRangeLhs::WorkingValueType: Add<&'a TRangeLhs::WorkingValueType, Output=TRangeLhs::WorkingValueType>
        + Sub<Output=TRangeLhs::WorkingValueType>,
        <TRangeLhs as IRange>::WorkingValueType: IWrappingOps<Output=<TRangeLhs as IRange>::WorkingValueType>
        + Clone
        + NumOps<<TRangeLhs as IRange>::WorkingValueType, <TRangeLhs as IRange>::WorkingValueType>
        + One
        + PartialOrd
        + Zero,
        TRangeRhs: IRangeFrom + IRangeToInclusive,
{
    fn wrapping_add(self, rhs: Ranged<TRangeRhs>) -> Self::Output {
        let (start, end) =
            (TRangeLhs::WorkingValueType::from(TRangeLhs::MIN), TRangeLhs::WorkingValueType::from(TRangeLhs::MAX));
        let span = &(&end - &start) + &<TRangeLhs as IRange>::WorkingValueType::one();
        let sum = &(TRangeLhs::WorkingValueType::from(self.0) + TRangeRhs::WorkingValueType::from(rhs.0)) - &start;
        let offset = sum % span;
        let wrapped_sum = (offset + start).try_into().unwrap_or_else(|_err| unreachable!());
        Self::from(wrapped_sum)
    }

    fn wrapping_div(self, _rhs: Ranged<TRangeRhs>) -> Self::Output { todo!() }

    fn wrapping_div_euclid(self, _rhs: Ranged<TRangeRhs>) -> Self::Output { todo!() }

    fn wrapping_mul(self, _rhs: Ranged<TRangeRhs>) -> Self::Output { todo!() }

    fn wrapping_rem(self, _rhs: Ranged<TRangeRhs>) -> Self::Output { todo!() }

    fn wrapping_rem_euclid(self, _rhs: Ranged<TRangeRhs>) -> Self::Output { todo!() }

    fn wrapping_sub(self, _rhs: Ranged<TRangeRhs>) -> Self::Output { todo!() }
}

impl<TRangeLhs> IWrappingNonGenericOps for Ranged<TRangeLhs>
    where
        Self: PartialOrd,
        TRangeLhs: IMinMax<TRangeLhs::ValueType> + IRangeFrom + IRangeToInclusive,
        TRangeLhs::ValueType: PartialOrd + IWrappingOps,
        <TRangeLhs as IRange>::WorkingValueType: IWrappingOps<Output=<TRangeLhs as IRange>::WorkingValueType>
        + Clone
        + NumOps<<TRangeLhs as IRange>::WorkingValueType, <TRangeLhs as IRange>::WorkingValueType>
        + One
        + PartialOrd
        + Zero,
{
    type Output = Self;

    fn wrapping_abs(self) -> Self::Output {
        let (start, end) = (
            <TRangeLhs as IRange>::WorkingValueType::from(<TRangeLhs as IRangeFrom>::start()),
            <TRangeLhs as IRange>::WorkingValueType::from(<TRangeLhs as IRangeTo>::end()),
        );

        // `WorkingValueType::from(self.0).abs()` cannot wrap, so `wrapping_abs()` is being used as
        // "plain `abs`".  Note there is no convenient "plain `abs`" for `ValueType` as (`num::abs<T>(T)` is only
        // defined on `T: Signed` (i.e. `f32`, `f64` and `Rational`).
        // TODO: Use `abs` or `wrapping_abs`. Attempts to use `WorkingValueType.wrapping_abs()` seem to enter infinite
        //       recursive loops (resolving `IWrapping`?).
        let abs_value = {
            let tmp_value = <TRangeLhs as IRange>::WorkingValueType::from(self.0);
            match tmp_value >= <TRangeLhs as IRange>::WorkingValueType::zero() {
                true => tmp_value,
                false => <TRangeLhs as IRange>::WorkingValueType::zero() - tmp_value,
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
                // let alone `WorkingValueType`, whose size is at least `2 * size_of::<ValueType>()`.
                // `end - start` + 1 (required for inclusive `Range`) _could_ overflow, so `ValueType` has been promoted
                // to `WorkingValueType` to ensure overflow cannot happen in this case either.
                #[allow(clippy::integer_arithmetic)]
                let range_len = end.clone() - start.clone() + <TRangeLhs as IRange>::WorkingValueType::one();

                // `abs_value > end` per `match` arm ensures that `abs_value - end` cannot overflow
                #[allow(clippy::integer_arithmetic)]
                let overflow_magnitude = abs_value - end;

                let range_offset = overflow_magnitude % range_len;
                start + range_offset
            },
        };

        // `range_offset` calculation above guarantees `working_value` is within `TRange::start()..=TRange::end()`.
        // TODO: Since `TryFrom` does not require the returned `Result`'s `E` to be `Error` or even `Debug`, determine
        //       how to output `err` to aid debuggability
        Self(working_value.try_into().unwrap_or_else(|_err| unreachable!()))
    }

    fn wrapping_neg(self) -> Self::Output { todo!() }

    fn wrapping_pow(self, _rhs: u32) -> Self::Output { todo!() }

    fn wrapping_shl(self, _rhs: u32) -> Self::Output { todo!() }

    fn wrapping_shr(self, _rhs: u32) -> Self::Output { todo!() }
}
