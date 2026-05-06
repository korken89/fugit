# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.4.0] - 2026-05-06

### Added

- `Duration::ZERO` and `Duration::MAX` constants
- `Duration::checked_mul`, `checked_div`, `div_ceil` methods
- `Duration::saturating_add`, `saturating_sub`, `saturating_mul` methods
- `Duration::from_secs_f32` and `from_secs_f64` constructors with rounding
- `Duration::as_secs_f32` and `as_secs_f64` conversions
- Support for picosecond-level precision (enabled by u64 const generics)
- `picos()` shorthand method to `ExtU32`, `ExtU64`, `ExtU32Ceil`, and `ExtU64Ceil` traits
- `Duration::from_picos`, `as_picos`, `from_picos_at_least` methods
- `PicosDuration<T>`, `PicosDurationU32`, `PicosDurationU64` aliases
- `Gigahertz<T>`, `GigahertzU32`, `GigahertzU64` aliases (the `defmt`/`Display` impls already formatted this base as `GHz`)
- `core::ops::Rem` and `core::ops::RemAssign` trait implementations for `Rate` and `Duration` (fixes #41)
- `core::ops::SubAssign` for `Rate` (was missing; asymmetric with `Add`/`AddAssign`/`Sub`)
- Conversion to/from `core::time::Duration`

### Fixed

- Issue #50: Rate conversions now use half-up rounding instead of truncating for better accuracy
- Issue #53: Overflow detection in shorthand conversions (e.g., `NanosDurationU32.minutes()`)
- Added `#[track_caller]` to all panicking functions and operator impls (including `*Assign` wrappers, `Mul`/`Div`/`Rem` on integers, and the cross-type `u32`<->`u64` ops) so panic locations point at the user's call site.
- Documentation clarifications across `Instant::checked_*_duration`, `Instant::checked_duration_since`, `Duration::checked_*` / `Rate::checked_*`, the `try_from_*`/`try_to_*` conversions, and the `const_partial_cmp`/`const_eq` methods. Several methods previously claimed to "check for overflow" or "check for divide-by-zero" while the actual contract was different (e.g. wrapping ticks, returning `None` on cross-base conversion overflow).
- `Duration::const_try_from`/`const_try_into`/`convert` (and the same on `Rate`) no longer overflow `u64` silently when ticks are close to `u64::MAX`. The rounding step `(lh + d/2) / d` was replaced with an overflow-free divmod-based form that produces the same rounded result without ever wrapping.
- `Duration::Hz`/`kHz`/`MHz`, `Rate::Hz`/`kHz`/`MHz`, `Rate::to_Hz`/`to_kHz`/`to_MHz`, `Duration::try_from_rate`, and `Rate::try_from_duration` now produce a compile-time error when the conversion constants don't fit the storage type, instead of silently truncating `u64` constants on a `u32` cast.
- The `Duration` shorthand methods (`as_*`, `from_*`, `from_*_at_least`) and the `Rate::Hz`/`kHz`/`MHz`/`to_Hz`/`to_kHz`/`to_MHz` family now panic on multiplication overflow instead of silently wrapping in release builds. All gained `#[track_caller]` so the panic points at the user's call site.

### Changed

- Modernized CI
- Updated `gcd` dependency to 2.3
- Updated `defmt` dependency to 1.0

### Breaking Changes

- **BREAKING**: `Duration::const_try_from` now rounds-to-nearest, before it was truncating
- **BREAKING**: Standardized API naming conventions
  - Constructors now use `from_*` prefix (e.g., `from_ticks`, `from_rate`)
  - Getters now use `to_*` or `as_*` prefix (e.g., `to_rate`, `as_ticks`)
- **BREAKING**: Changed `NOM` and `DENOM` const generic parameters from `u32` to `u64` across all types (`Duration`, `Rate`, `Instant`)
  - This enables support for higher precision time units (e.g., picoseconds with denominator 1_000_000_000_000)
  - Type aliases with `FREQ_HZ` parameters now use `u64` instead of `u32`
  - Compile-time calculations use u128 intermediates with overflow checks, but results are guaranteed to fit in u64 for embedded compatibility

## [v0.3.9]

### Added

### Fixed

- No default feature from `serde`

### Changed

## [v0.3.8] - YANKED

### Added

- Optional `serde` support
- Optional `postcard` max size support

### Fixed

### Changed

## [v0.3.7]

### Added

- `Hz`, `kHz`, `MHz` shorthands for `Duration` and `nanos`, `micros`, `millis` for `Rate`
- `from_rate`, `from_duration` in addition to `into_duration`, `into_rate`;
- `const_try_from` in addition to `const_try_into`
- `<unit>_at_least` for ceil-rounded `Duration` convert
- Add `is_zero()` methods to `Duration<u32, NOM, DENOM>` and `Duration<u64, NOM, DENOM>` types to compare with zero without type conversion

### Fixed

### Changed

- Updated documentation of `const_cmp`

## [v0.3.6]

### Fixed

- Fixed error in conversion between large durations.
- Fixed `TimerRateU32` and `TimerRateU64` to use `Rate` instead of `Duration`

## [v0.3.5]

### Added

### Fixed

- Fixed `to_X` rates.

### Changed

## [v0.3.4] -- YANKED

### Added

- Add `Div` implementation for `Duration` and `Rate`
- Add `nanos()` methods and `NanosDuration` aliases alongside other units.
- Implement AddAssign and SubAssign for Instant and Duration, and
  MulAssign and DivAssign for Duration.
- Add `to_nanos()`, `to_micros()`, `to_millis()`, `to_secs()`, `to_minutes()`,
  and `to_hours()` methods to `Duration<u32, NOM, DENOM>` and
  `Duration<u64, NOM, DENOM>` types to easily convert to integer time units.
- Support for `Rate` (Hertz, Kilohertz, ...)

### Fixed

### Changed

## [v0.3.3]

### Changed

- Underlying const gcd implementation switched to the `gcd` crate.
- `Duration::convert` now `const`.

## [v0.3.2]

### Fixed

- `Duration::convert` did not do the right thing when getting close to maximum supported values.

## [v0.3.1]

### Added

- Added `CHANGELOG.md`

### Fixed

- Now supports a `defmt` version span (0.2 and 0.3 is supported)

[Unreleased]: https://github.com/korken89/fugit/compare/v0.4.0...HEAD
[v0.4.0]: https://github.com/korken89/fugit/compare/v0.3.9...v0.4.0
[v0.3.9]: https://github.com/korken89/fugit/compare/v0.3.8...v0.3.9
[v0.3.8]: https://github.com/korken89/fugit/compare/v0.3.7...v0.3.8
[v0.3.7]: https://github.com/korken89/fugit/compare/v0.3.6...v0.3.7
[v0.3.6]: https://github.com/korken89/fugit/compare/v0.3.5...v0.3.6
[v0.3.5]: https://github.com/korken89/fugit/compare/v0.3.4...v0.3.5
[v0.3.4]: https://github.com/korken89/fugit/compare/v0.3.3...v0.3.4
[v0.3.3]: https://github.com/korken89/fugit/compare/v0.3.2...v0.3.3
[v0.3.2]: https://github.com/korken89/fugit/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/korken89/fugit/compare/v0.3.0...v0.3.1
