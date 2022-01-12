#![feature(
const_refs_to_cell,
const_fn_trait_bound,
const_num_from_num,
const_option,
const_trait_impl,
const_type_id,
generic_associated_types,
step_trait,
unchecked_math
)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
// Safety-critical application lints
#![deny(
    clippy::pedantic,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used
)]
#![allow(
    clippy::implicit_return,
    clippy::iter_nth_zero,
    clippy::match_bool,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
clippy::wildcard_imports
)]
// To use the `unsafe` keyword, do not remove the `unsafe_code` attribute entirely.
// Instead, change it to `#![allow(unsafe_code)]` or preferably `#![deny(unsafe_code)]` + opt-in
// with local `#[allow(unsafe_code)]`'s on a case-by-case basis, if practical.
#![deny(unsafe_code)]
#![forbid(bare_trait_objects)]
// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing
// license files and more
// #![allow(clippy::blanket_clippy_restriction_lints)]
// #![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
// #![allow(clippy::implicit_return)]

mod consts;
mod error;
mod ranges;
mod traits;
// mod ranged;

pub use error::{ErrInt, ErrIntPrimIntExt, Error, Result};
// pub use ranges::{RiI128, RiI16, RiI32, RiI64, RiI8, RiU128, RiU16, RiU32, RiU64, RiU8};
