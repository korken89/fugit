# fugit

[![crates.io](https://img.shields.io/crates/v/fugit.svg)](https://crates.io/crates/fugit)
[![docs.rs](https://docs.rs/fugit/badge.svg)](https://docs.rs/fugit)
[![Build](https://github.com/korken89/fugit/workflows/Build/badge.svg)](https://github.com/korken89/fugit/actions)

A `no_std` time library for embedded systems that does as much as possible at compile time. Heavily inspired by C++ `std::chrono`.

## Overview

`fugit` provides `Duration`, `Instant`, and `Rate` types for handling time in embedded applications. The library is designed around concrete types rather than traits, which allows extensive use of `const fn` and compile-time optimization.

When you add or compare durations with different time bases, the library generates all the necessary conversion constants at compile time. This means operations like adding milliseconds to microseconds can often happen without any runtime division, just a simple addition with a compile-time calculated multiplier.

The library supports both `u32` and `u64` backing storage with careful attention to code generation on embedded targets. With `u64` you can reach picosecond-level precision (`1/1_000_000_000_000` seconds), useful for high-resolution timers. On ARM Cortex-M3 and newer, most operations avoid pulling in soft implementations for division. Comparisons between different time bases use multiplication rather than division, and the constants are all calculated at compile time.

Extension traits provide convenient shorthand methods. Instead of manually creating `Duration::<u32, 1, 1000>::from_ticks(100)`, you can just write `100.millis()`. Methods like `.picos()`, `.nanos()`, `.millis()`, `.secs()`, `.Hz()`, and `.kHz()` cover the common cases, and `_at_least` variants (e.g. `.millis_at_least()`) ceil-round when the source value isn't exactly representable, which is what you usually want for "wait at least X" patterns.

Conversion to and from `core::time::Duration` is also provided, so values can cross the boundary between `fugit` and the standard library without manual reconstruction.

## Use Cases

The library is particularly well-suited for embedded HAL implementations, RTIC applications, and any embedded system where you need to work with timeouts, delays, or periodic operations. The compile-time optimization means you get readable code without sacrificing the tight instruction counts needed on microcontrollers.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
