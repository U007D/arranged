# arranged

**A** **R**ust **ranged**-type library.

[`arranged`](https://crates.io/crates/arranged) is ranged type crate for Rust.  Note: This crate is currently pre-alpha and is still under construction.

## Goals
`arranged` is a pathfinder implementation to safe default arithmetic manipulation of values in Rust--no arithmetic operation should ever fail (panic) due to overflow at runtime 
(unless the user explicitly requests a panic)--while supporting ergonomics via conventional arithmetic operators (`+`, `-`, `*`, `/`, `%`, etc.).

## Design Philosophy
The Ranged type is designed to be zero-runtime overhead where possible  and minimal runtime overhead otherwise:
* all bounds-checking on arithmetic operations between two unwrapped Ranged types occurs at compile-time
* `mem::size_of::<Ranged<T>>() == mem::size_of::<T>()`  (i.e. ranges are zero-sized).  Performance & cache-friendly, esp. when 
operating on large quantities of values

## Behavior
* `Ranged<T>` {op} `Ranged<U>` yields `Range<V>` and is compile-time bounds checked (where {op} represents an arithmetic 
operation, and `T`, `U` and `V` represent Ranges of values of the same machine word type).  Any possible overflow fails to compile.
* `Ranged<T>` {op} `*Wrapper<Ranged<U>>` yields `Ranged<T>`, is runtime bounds-checked and handles overflow according to`*` policy, where `*` is one of Checked, Overflowing, Panicking, Saturating or Wrapping overflow policies.
* `Ranged<T>` {op} {scalar} does not compile because policy is unspecified
* `Ranged<T>` {op} `*Wrapper<{scalar}>` yields `Ranged<T>`, is runtime bounds-checked and handles overflow according to`*` policy
* {scalar} {op} `Ranged<T>` does not compile because policy is unspecified
* {scalar} {op} `*Wrapper<{scalar}>` yields `*Wrapper<{scalar}>`, is runtime bounds-checked and handles overflow according to `*` policy
* {scalar} {op} `*Wrapper<Ranged<T>>` yields `*Wrapper<Ranged<T>>`, is runtime bounds-checked and handles overflow according to `*` policy
* `*Wrapper<Ranged<T>>` {op} `Ranged<U>` yields `*Wrapper<Ranged<V>>`, is compile-time bounds checked and handles overflow according to `*` policy
* `*Wrapper<Ranged<T>>` {op} `†Wrapper<Ranged<U>>` yields `*Wrapper<Ranged<T>>`, is runtime bounds checked and handles overflow according to `*` policy.  `†` policy is ignored.
* `*Wrapper<Ranged<T>` {op} {scalar} yields `*Wrapper<"Ranged<T>>`, is runtime bounds checked and handles overflow according to `*` policy.

## Implementation plan
- [x] ~~unify existing Rust arithmetic overflow policies under arithmetic traits + add `Panicking` policy ([arith_traits](https://crates.io/crates/arith_traits))~~
- [x] ~~establish initial implementation of one arithmetic *Wrapper type ([arith_wrappers](https://crates.io/crates/arith_wrappers)) to support rapid PoC~~
- [x] ~~design and implement one initial Range statically bounds verifiable, zero-sized integer type, addressing concerns [1](https://ridiculousfish.com/blog/posts/least-favorite-rust-type.html), [2](https://kaylynn.gay/blog/post/rust_ranges_and_suffering) as a rapid PoC~~
- [x] ~~design and implement Ranged type to bring together the concept of Ranges and values into a single type~~
- [ ] implement one Ranged<T> arithmetic operation covering all variations listed above, without and with one wrapper type as PoC.
- [ ] verify implementation is zero runtime overhead or low runtime overhead in release mode
- [ ] extend pathfinder implementation to support f32 and f64

— !! pathfinder complete !! — if successful:
- [ ] implement remaining arith_traits
- [ ] implement remaining arith_wrappers
- [ ] implement remaining arithmetic operators for `Ranged<Ri*>` (where `Ri` is `arranged`’s range-inclusive type and `*` represents any int and float machine word types or `BigInt` or `BigUint`)
- [ ] implement R (half-open range) type
- [ ] implement remaining arithmetic operators for `Ranged<R*>` (where `R` is `arranged`’s (half-open) range type and `*` represents any int and float machine word types or `BigInt` or `BigUint`)
- [ ] consider priority of implementing additional range types (JIT/as-needed basis?)

## What problem is `arranged` addressing?
Rust has made terrific progress moving the safety and correctness needle without sacrificing performance.  And yet we are still saddled with C’s arithmetic model from the early 1970's which does a very poor job of providing predictable behavior in important real-world safety scenarios--[Ariane 5 initial launch](https://www.youtube.com/watch?v=PK_yguLapgA), [analysis](https://youtu.be/Xsqe3utT6rs?t=238).

As software eats more and more of the world, the importance of performant, ergonomic, ‘safe by default’ systems increases.

## Beyond arranged
Explore leveraging arranged as part of a more comprehensive [“semantic types”](https://stackoverflow.com/a/39417034/1541330) implementation in Rust, hopefully further addressing safety concerns such as [NASA losing a $300M+ mission](https://www.youtube.com/watch?v=u4r0yrF_Wa0&t=969s)

## Usage

```rust
// Create a `u8`-based `RangeInclusive`-style type limited to `1..=100`, set to the value 42
let my_ranged_value = Ranged::<RiU8<1, 100>>::from(42);
```
For more examples, see `ranged::unit_tests`.

## License

Licensed under either:

* MIT license (see LICENSE-MIT file)
* Apache License, Version 2.0 (see LICENSE-APACHE file)
  at your option.

## Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall
be dual licensed as above, without any additional terms or conditions.
