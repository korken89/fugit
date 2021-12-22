# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Added

- `Instant::into` and `Instant::try_into` to convert between `Instant`s of different fractions (#13)

### Fixed

### Changed

- Underlying method of calculating the GCD of fractions at compile time changed to `gcd` crate (#13)

## [v0.3.2]

### Fixed

- `Duration::convert` did not do the right thing when getting close to maximum supported values.

## [v0.3.1]

### Added

- Added `CHANGELOG.md`

### Fixed

- Now supports a `defmt` version span (0.2 and 0.3 is supported)

[Unreleased]: https://github.com/korken89/fugit/compare/v0.3.2...HEAD
[v0.3.2]: https://github.com/korken89/fugit/compare/v0.3.1...v0.3.2
[v0.3.1]: https://github.com/korken89/fugit/compare/v0.3.0...v0.3.1
