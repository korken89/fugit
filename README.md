# fugit

[![crates.io](https://img.shields.io/crates/v/fugit.svg)](https://crates.io/crates/fugit)
[![docs.rs](https://docs.rs/fugit/badge.svg)](https://docs.rs/fugit)
[![Build](https://github.com/korken89/fugit/workflows/Build/badge.svg)](https://github.com/korken89/fugit/actions)

A `no_std` time library for embedded systems that does as much as possible at compile time. Heavily inspired by C++ `std::chrono`.

## Overview

`fugit` provides `Duration`, `Instant`, and `Rate` types for handling time in embedded applications. The library is designed around concrete types rather than traits, which allows extensive use of `const fn` and compile-time optimization.

When you add or compare durations with different time bases, the library generates all the necessary conversion constants at compile time. This means operations like adding milliseconds to microseconds can often happen without any runtime division, just a simple addition with a compile-time calculated multiplier.

The library supports both `u32` and `u64` backing storage with careful attention to code generation on embedded targets. On ARM Cortex-M3 and newer, most operations avoid pulling in soft implementations for division. Comparisons between different time bases use multiplication rather than division, and the constants are all calculated at compile time.

Extension traits provide convenient shorthand methods. Instead of manually creating `Duration::<u32, 1, 1000>::from_ticks(100)`, you can just write `100.millis()`. The methods `.millis()`, `.secs()`, `.Hz()`, `.kHz()`, and others make the code cleaner while maintaining the same performance characteristics.

## Use Cases

The library is particularly well-suited for embedded HAL implementations, RTIC applications, and any embedded system where you need to work with timeouts, delays, or periodic operations. The compile-time optimization means you get readable code without sacrificing the tight instruction counts needed on microcontrollers.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
