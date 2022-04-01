use super::*;
use assert2::assert;

type Range<const START: i32, const END: i32> = RiI32<START, END>;

#[test]
#[allow(clippy::assertions_on_constants)]
const fn returns_expected_value() {
    // Given
    const MIN_CPU_HZ: i32 = 350_000_000;
    const MAX_CPU_HZ: i32 = 1_400_000_000;

    type Sut = Range<MIN_CPU_HZ, MAX_CPU_HZ>;

    const EXPECTED: i32 = 350_000_000;

    // When
    const RESULT: i32 = Sut::start();

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}
