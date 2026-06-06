# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- ## [Unreleased] -->

## [0.2.0] - 2026-06-06

### Changed

- Change source for [wordlist](https://people.sc.fsu.edu/~jburkardt/datasets/words/sowpods.txt).
- Make error message from setting `--min` higher than `--max` less vague.
- Make printed error messages cleaner.

### Fixed

- Disallow user from being able to start with zero lives.

### Removed

- Default minimum/maximum wordlength specification for the secret word. Words
  of any length are now allowed by default.

## [0.1.0] - 2026-06-04

### Added

- Basic functionality.
- Automatic caching for the online [wordlist](https://www.mit.edu/~ecprice/wordlist.10000).
- Option to adjust the minimum and maximum wordlength for the secret word.
- Option to adjust how many lives the player starts with.

[unreleased]: https://github.com/mattaroni/hangcrab/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/mattaroni/hangcrab/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/mattaroni/hangcrab/releases/tag/v0.1.0
