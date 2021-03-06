use std::panic::catch_unwind;

use super::*;
use crate::RiU64;

#[allow(clippy::items_after_statements)]
#[test]
const fn const_ranged_contains_in_bounds_value() {
    // Given
    const MIN_CPU_HZ: u64 = 350_000_000;
    const MAX_CPU_HZ: u64 = 1_400_000_000;

    type HzRange = RiU64<MIN_CPU_HZ, MAX_CPU_HZ>;

    const FREQ: u64 = 1_000_000_000;
    const EXPECTED_START: u64 = MIN_CPU_HZ;
    const EXPECTED_END: u64 = MAX_CPU_HZ;
    const EXPECTED_VALUE: u64 = FREQ;

    // When
    const RESULT: Ranged<HzRange> = Ranged::<HzRange>::from(FREQ);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT.start() == EXPECTED_START);
    std::assert!(RESULT.end() == EXPECTED_END);
    std::assert!(*RESULT.value() == EXPECTED_VALUE);
}

#[test]
fn const_ranged_does_not_contain_out_of_bounds_value() {
    // Given
    const MIN_CPU_HZ: u64 = 350_000_000;
    const MAX_CPU_HZ: u64 = 1_400_000_000;

    type HzRange = RiU64<MIN_CPU_HZ, MAX_CPU_HZ>;

    const FREQ: u64 = 2_000_000_000;

    // When
    let result = catch_unwind(|| Ranged::<HzRange>::from(FREQ));

    // Then
    std::assert!(result.is_err());
}
