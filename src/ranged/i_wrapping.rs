#[cfg(test)]
mod unit_tests;

use super::*;
use arith_traits::{IMinMax, IWrapping};
use num::traits::{NumOps, One, Zero};
use std::ops::Neg;

impl<TRange> IWrapping for Ranged<TRange>
where
    Self: PartialOrd,
    TRange: IRangeFrom + IRangeToInclusive,
    TRange::ValueType: PartialOrd + IWrapping,
    <TRange as IRange>::WorkingValueType: IWrapping<Output=<TRange as IRange>::WorkingValueType>
    + Clone
    + NumOps<<TRange as IRange>::WorkingValueType, <TRange as IRange>::WorkingValueType>
    + One
    + PartialOrd
    + Zero,
{
    type Output = Self;

    fn wrapping_abs(self) -> Self::Output {
        let (start, end) = (
            <TRange as IRange>::WorkingValueType::from(<TRange as IRangeFrom>::start()),
            <TRange as IRange>::WorkingValueType::from(<TRange as IRangeTo>::end()),
        );

        // `WorkingValueType::from(self.0).abs()` cannot wrap, so `wrapping_abs()` is being used as
        // "plain `abs`".  Note there is no convenient "plain `abs`" for `ValueType` as (`num::abs<T>(T)` is only
        // defined on `T: Signed` (i.e. `f32`, `f64` and `Rational`).
        // TODO: Use `abs` or `wrapping_abs`. Attempts to use `WorkingValueType.wrapping_abs()` seem to enter infinite
        //       recursive loops (resolving `IWrapping`?).
        let abs_value = {
            let tmp_value = <TRange as IRange>::WorkingValueType::from(self.0);
            match tmp_value >= <TRange as IRange>::WorkingValueType::zero() {
                true => tmp_value,
                false => <TRange as IRange>::WorkingValueType::zero() - tmp_value,
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
                    let range_len = end.clone() - start.clone() + <TRange as IRange>::WorkingValueType::one();

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

    fn wrapping_add(self, rhs: Self) -> Self::Output { todo!() }

    fn wrapping_div(self, rhs: Self) -> Self::Output { todo!() }

    fn wrapping_div_euclid(self, rhs: Self) -> Self::Output { todo!() }

    fn wrapping_mul(self, rhs: Self) -> Self::Output { todo!() }

    fn wrapping_neg(self) -> Self::Output { todo!() }

    fn wrapping_pow(self, rhs: u32) -> Self::Output { todo!() }

    fn wrapping_rem(self, rhs: Self) -> Self::Output { todo!() }

    fn wrapping_rem_euclid(self, rhs: Self) -> Self::Output { todo!() }

    fn wrapping_shl(self, rhs: u32) -> Self::Output { todo!() }

    fn wrapping_shr(self, rhs: u32) -> Self::Output { todo!() }

    fn wrapping_sub(self, rhs: Self) -> Self::Output { todo!() }
}

impl<TRange> const IMinMax for Ranged<TRange>
    where
        Self: PartialOrd,
        TRange: ~ const IRangeFrom + ~ const IRangeTo + ~ const IRangeToInclusive,
{
    const MAX: Self = Self(TRange::end());
    const MIN: Self = Self(TRange::start());
}
