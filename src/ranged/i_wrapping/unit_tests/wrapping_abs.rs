#[allow(unused_imports)]
use super::*;
use crate::RiU64;
use assert2::assert;
use backtrace::Backtrace;
use std::panic::{set_hook, take_hook};

#[allow(clippy::items_after_statements)]
#[test]
fn range_with_nonnegative_start_and_end_returns_original_value() {
    // Given
    type OneToOneHundred = RiU64<1, 100>;
    let orig_hook = take_hook();
    set_hook(Box::new(move |panic_info| {
        println!("{:?}", Backtrace::new());
        orig_hook(panic_info);
    }));
    let value = 42;
    let sut = Ranged::<OneToOneHundred>::from(value);
    let expected = Ranged::<OneToOneHundred>::from(value);

    // When
    let result = sut.wrapping_abs();

    // Then
    assert!(result == expected);
}

// #[test]
// fn range_with_nonnegative_start_and_end_with_minimum_value_returns_original_value() {
//     // Given
//     type ZeroToNineNinetyNine = RiU64::<0, 999>;
//     let value = 0;
//     let sut = Ranged::<ZeroToNineNinetyNine>::from(value);
//     let expected = Ranged::<ZeroToNineNinetyNine>::from(value);
//
//     // When
//     let result = sut.wrapping_abs();
//
//     // Then
//     assert!(result == expected);
// }
