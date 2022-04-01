use super::*;
use assert2::assert;

type Range<const START: i128, const END: i128> = RiI128<START, END>;

#[test]
const fn const_range_contains_in_bounds_value() {
    // Given
    const ELECTRON_CHARGE_IN_INVERSE_COULOMBS: i128 = -16_020_000_000_000_000_000;
    const PROTON_CHARGE_IN_INVERSE_COULOMBS: i128 = 16_020_000_000_000_000_000;

    type Sut = Range<ELECTRON_CHARGE_IN_INVERSE_COULOMBS, PROTON_CHARGE_IN_INVERSE_COULOMBS>;

    const NEUTRON: i128 = 0;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&NEUTRON);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_contains_min_bounds_value() {
    // Given
    const ELECTRON_CHARGE_IN_INVERSE_COULOMBS: i128 = -16_020_000_000_000_000_000;
    const PROTON_CHARGE_IN_INVERSE_COULOMBS: i128 = 16_020_000_000_000_000_000;

    type Sut = Range<ELECTRON_CHARGE_IN_INVERSE_COULOMBS, PROTON_CHARGE_IN_INVERSE_COULOMBS>;

    const ELECTRON: i128 = -16_020_000_000_000_000_000;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&ELECTRON);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_contains_max_bounds_value() {
    // Given
    const ELECTRON_CHARGE_IN_INVERSE_COULOMBS: i128 = -16_020_000_000_000_000_000;
    const PROTON_CHARGE_IN_INVERSE_COULOMBS: i128 = 16_020_000_000_000_000_000;

    type Sut = Range<ELECTRON_CHARGE_IN_INVERSE_COULOMBS, PROTON_CHARGE_IN_INVERSE_COULOMBS>;

    const PROTON: i128 = 16_020_000_000_000_000_000;
    const EXPECTED: bool = true;

    // When
    const RESULT: bool = Sut::contains(&PROTON);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_does_not_contain_low_out_of_bounds_value() {
    // Given
    const ELECTRON_CHARGE_IN_INVERSE_COULOMBS: i128 = -16_020_000_000_000_000_000;
    const PROTON_CHARGE_IN_INVERSE_COULOMBS: i128 = 16_020_000_000_000_000_000;

    type Sut = Range<ELECTRON_CHARGE_IN_INVERSE_COULOMBS, PROTON_CHARGE_IN_INVERSE_COULOMBS>;

    const SUPER_NEGATIVE_CHARGE: i128 = -32_040_000_000_000_000_000;
    const EXPECTED: bool = false;

    // When
    const RESULT: bool = Sut::contains(&SUPER_NEGATIVE_CHARGE);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
const fn const_range_does_not_contain_high_out_of_bounds_value() {
    // Given
    const ELECTRON_CHARGE_IN_INVERSE_COULOMBS: i128 = -16_020_000_000_000_000_000;
    const PROTON_CHARGE_IN_INVERSE_COULOMBS: i128 = 16_020_000_000_000_000_000;

    type Sut = Range<ELECTRON_CHARGE_IN_INVERSE_COULOMBS, PROTON_CHARGE_IN_INVERSE_COULOMBS>;

    const SUPER_POSITIVE_CHARGE: i128 = 32_040_000_000_000_000_000;
    const EXPECTED: bool = false;

    // When
    const RESULT: bool = Sut::contains(&SUPER_POSITIVE_CHARGE);

    // Then
    // Uncomment below and set `assert` to failing condition to observe compile-time impact
    // const _ASSERT: () = std::assert!(RESULT == EXPECTED);
    std::assert!(RESULT == EXPECTED);
}

#[test]
fn range_contains_in_bounds_value() {
    // Given
    const ELECTRON_CHARGE_IN_INVERSE_COULOMBS: i128 = -16_020_000_000_000_000_000;
    const PROTON_CHARGE_IN_INVERSE_COULOMBS: i128 = 16_020_000_000_000_000_000;

    type Sut = Range<ELECTRON_CHARGE_IN_INVERSE_COULOMBS, PROTON_CHARGE_IN_INVERSE_COULOMBS>;

    let neutron: i128 = 0;
    let expected = true;

    // When
    let result = Sut::contains(&neutron);

    // Then
    assert!(result == expected);
}

#[test]
fn range_contains_min_bounds_value() {
    // Given
    const ELECTRON_CHARGE_IN_INVERSE_COULOMBS: i128 = -16_020_000_000_000_000_000;
    const PROTON_CHARGE_IN_INVERSE_COULOMBS: i128 = 16_020_000_000_000_000_000;

    type Sut = Range<ELECTRON_CHARGE_IN_INVERSE_COULOMBS, PROTON_CHARGE_IN_INVERSE_COULOMBS>;

    let electron: i128 = -16_020_000_000_000_000_000;
    let expected = true;

    // When
    let result = Sut::contains(&electron);

    // Then
    assert!(result == expected);
}

#[test]
fn range_contains_max_bounds_value() {
    // Given
    const ELECTRON_CHARGE_IN_INVERSE_COULOMBS: i128 = -16_020_000_000_000_000_000;
    const PROTON_CHARGE_IN_INVERSE_COULOMBS: i128 = 16_020_000_000_000_000_000;

    type Sut = Range<ELECTRON_CHARGE_IN_INVERSE_COULOMBS, PROTON_CHARGE_IN_INVERSE_COULOMBS>;

    let proton: i128 = 16_020_000_000_000_000_000;
    let expected = true;

    // When
    let result = Sut::contains(&proton);

    // Then
    assert!(result == expected);
}

#[test]
fn range_does_not_contain_low_out_of_bounds_value() {
    // Given
    const ELECTRON_CHARGE_IN_INVERSE_COULOMBS: i128 = -16_020_000_000_000_000_000;
    const PROTON_CHARGE_IN_INVERSE_COULOMBS: i128 = 16_020_000_000_000_000_000;

    type Sut = Range<ELECTRON_CHARGE_IN_INVERSE_COULOMBS, PROTON_CHARGE_IN_INVERSE_COULOMBS>;

    let super_negative_charge: i128 = -32_040_000_000_000_000_000;
    let expected = false;

    // When
    let result = Sut::contains(&super_negative_charge);

    // Then
    assert!(result == expected);
}

#[test]
fn range_does_not_contain_high_out_of_bounds_value() {
    // Given
    const ELECTRON_CHARGE_IN_INVERSE_COULOMBS: i128 = -16_020_000_000_000_000_000;
    const PROTON_CHARGE_IN_INVERSE_COULOMBS: i128 = 16_020_000_000_000_000_000;

    type Sut = Range<ELECTRON_CHARGE_IN_INVERSE_COULOMBS, PROTON_CHARGE_IN_INVERSE_COULOMBS>;

    let super_positive_charge: i128 = 32_040_000_000_000_000_000;
    let expected = false;

    // When
    let result = Sut::contains(&super_positive_charge);

    // Then
    assert!(result == expected);
}
