use super::*;
use assert2::assert;

type ValueType = i32;
type Range<const START: ValueType, const END: ValueType> = RiI32<START, END>;

#[test]
#[allow(clippy::assertions_on_constants)]
const fn returns_expected_value() {
    // Given
    const MIN_CPU_HZ: ValueType = 350_000_000;
    const MAX_CPU_HZ: ValueType = 1_400_000_000;

    type Sut = Range<MIN_CPU_HZ, MAX_CPU_HZ>;

    const EXPECTED: ValueType = 350_000_000;

    // When
    const RESULT: ValueType = Sut::start();

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}
