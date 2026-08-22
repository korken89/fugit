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

## Two kinds of instant

`Instant` names the kind of timeline it sits on, because a raw counter and an extended one need different operations to be correct. `WrappingInstant` wraps and compares wrap-aware, which is only meaningful within half the tick range so comparison goes through the `InstantOrd` trait or `is_before`/`is_after`. `MonotonicInstant` compares as a plain integer, and implements `Ord`.

Nothing verifies that a monotonic timeline really does not wrap. `from_ticks` accepts any value, so feeding it a raw counter that has wrapped just produces an instant that is placed in the past. Use `WrappingInstant` for raw hardware counters and `MonotonicInstant` where the producer guarantees the monotonic timeline.

### Migrating `Instant` from v0.4

`Duration` and `Rate` gain no kind parameter.

| v0.4                      | >v0.5, wrapping                           | >v0.5, monotonic                      |
|---------------------------|-------------------------------------------|---------------------------------------|
| `Instant<T, NOM, DENOM>`  | `WrappingInstant<T, NOM, DENOM>`          | `MonotonicInstant<T, NOM, DENOM>`     |
| `TimerInstantU32<FREQ>`   | `WrappingTimerInstantU32<FREQ>`           | `MonotonicTimerInstantU32<FREQ>`      |
| `TimerInstantU64<FREQ>`   | `WrappingTimerInstantU64<FREQ>`           | `MonotonicTimerInstantU64<FREQ>`      |
| `checked_add_duration`    | `convert_add_duration`                    | same name, now checking the ticks too |
| `checked_sub_duration`    | `convert_sub_duration`                    | same name, now checking the ticks too |
| `const_cmp` -> `Ordering` | `const_partial_cmp` -> `Option<Ordering>` | unchanged                             |
| `duration_since_epoch`    | `Duration::from_ticks(i.as_ticks())`      | unchanged                             |
| `Ord`, `PartialOrd`, `<`  | removed, use `InstantOrd` / `is_before`   | unchanged                             |

Watch the `checked_*_duration` rows: the name still compiles on a monotonic instant but now fails on tick overflow as well as on the base conversion.

`Instant` with three generic arguments no longer names a type, so pick a kind or one of the aliases.

## Use Cases

The library is particularly well-suited for embedded HAL implementations, RTIC/embassy applications, and any embedded system where you need to work with timeouts, delays, or periodic operations. The compile-time optimization means you get readable code without sacrificing performance, especially on microcontrollers.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
