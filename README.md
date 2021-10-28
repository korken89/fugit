# `fugit`

This library is a heavily inspired Rust port of `std::chrono`'s `Duration` from C++ which does all it can at compile time.

## Aims

* `no_std` library with goals of user-friendliness and performance first
  * All methods are `const fn` that can be (i.e. non-trait methods)
  * Use no traits, concrete types all the way for maximum `const`-ification
  * Operations are supported between different bases and backing storages instead of implementing custom traits
  * All constants needed for comparing or changing timebase are guaranteed compile time generated
* Support for both `u32` and `u64` backing storage with efficient instruction lowering on MCUs
  * On Cortex-M3 and up: no soft-impls pulled in for both `u32` and `u64` except when changing base on `u64`
  * Comparisons on `u32` and `u64` do not use division, only changing base with all constants calculated at compile time
* Selection of base happens at compile time
  * A common problem is that run time changing of base robs us of a lot of optimization opportunities, but since there are no traits and short-hands select the correct base at compile time.

### Some features

```rust
fn foo(d: Duration<u32, 1, 1_000>) {
    // ...
}

foo(200.millis()) // <-- Compile time move of base
foo(Duration::<u32, 1, 1_000_000>::from_ticks(1_000_000).convert()) // <-- Compile time move of base


// -----------------------


let d = Duration::<u32, 1, 1_000>::from_ticks(111);

let sum1 = d + 300.millis();
//             ^^^ Compile time move of base, only a sum is needed and no change of base

let sum2 = d + Duration::<u32, 1, 1_000_000>::from_ticks(1_000_000);
//             ^^^ Compile time move of base, only a sum is needed and no change of base


// -----------------------


fn bar(d1: Duration<u32, 1, 1_000>, d2: Duration<u32, 1, 1_000_000>) {
    let sum = d1 + d2;
    //        ^^^^^^^ Run time move of base, will use a `mul` and `div` instruction (Cortex-M3+) to
    //                perform the move of base

    let ops = d1 > d2;
    //        ^^^^^^^ Run time comparison of different base, will use 2 `mul` instructions
    //                (Cortex-M3+) to perform the comparison
}

fn baz(d1: Duration<u64, 1, 1_000>, d2: Duration<u64, 1, 1_000_000>) {
    let sum = d1 + d2;
    //        ^^^^^^^ Run time move of base, will use a `mul` insruction and `div`
    //                soft-impl (Cortex-M3+) to perform the move of base

    let ops = d1 > d2;
    //        ^^^^^^^ Run time comparison of different base, will use 4 `mul` instructions
    //                (Cortex-M3+) to perform the comparison
}
```
