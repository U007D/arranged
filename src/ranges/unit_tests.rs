#![allow(clippy::equatable_if_let, clippy::unwrap_used)]

use std::ops::RangeInclusive;
use assert2::assert;
use crate::ranges::RangeInclusiveU64;
use super::*;

#[test]
fn from_const_succeeds_with_in_bounds_value() {
    // Given
    const MIN_CPU_HZ: u64 = 350_000_000;
    const MAX_CPU_HZ: u64 = 1_400_000_000;

    type CpuHz = RangeInclusiveU64<MIN_CPU_HZ, MAX_CPU_HZ>;

    const FREQ: u64 = 210_000;
    let expected = RiU64::<CpuHz>(FREQ);
    let sut = RiU64::<CpuHz>::from_const;

    // When
    let result = sut(FREQ);

    // Then
    assert!(result == expected);
}

// #[test]
// fn from_value_succeeds_with_in_bounds_value() {
//     // Given
//     struct ValidCpuFrequencies;
//
//     impl IRangeInclusive for ValidCpuFrequencies {
//         type Ty = u64;
//         const START: Self::Ty = 100_000;
//         const END: Self::Ty = 250_000;
//     }
//
//     let value = 210_000;
//     let expected = Ok(Riu64::<ValidCpuFrequencies>(value));
//     let sut = RangedInclusive::<ValidCpuFrequencies>::from_value;
//
//     // When
//     let result = sut(value);
//
//     // Then
//     assert!(result == expected);
// }
