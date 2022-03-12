# arranged

**A** **R**ust **ranged** type library.

Support for (machine-word-abstracting? TBD) ranged types as zero or low-runtime overhead abstractions using safe and
conventional arithmetic wrappers, written in Rust.

Currently experimental.

## Usage

```rust
// Create a `u8`-based ranged type limited to (`RangeInclusive`) `1..=100`, set to the value 42
let my_ranged_value = Ranged::<RiU8<1, 100>>::from(42);
```

## License

Licensed under either:

* MIT license (see LICENSE-MIT file)
* Apache License, Version 2.0 (see LICENSE-APACHE file)
  at your option.

## Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall
be dual licensed as above, without any additional terms or conditions.
