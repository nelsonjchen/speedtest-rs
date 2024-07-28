# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2024-07-27

### Changed

- [Update dependencies in general by @dilawar](https://github.com/nelsonjchen/speedtest-rs/pull/165)

## [0.1.5] - 2024-02-11

### Added

- [feat: hide log behind a feature by @radiohertz](https://github.com/nelsonjchen/speedtest-rs/pull/144)

### Changed

- Update dependencies in general


## [0.1.4] - 2023-02-25

### Changed

- Update dependencies in general

## [0.1.3] - 2021-07-24
### Fixed
- [Don't log finish parsed configuration if it fails](https://github.com/nelsonjchen/speedtest-rs/pull/84)

## [0.1.2] - 2021-04-14
### Fixed
- [Check whether ignore_server str is empty.](https://github.com/nelsonjchen/speedtest-rs/pull/78) Thanks [@pcmid](https://github.com/pcmid)!

## [0.1.1] - 2020-07-26
### Added
- Add a plain `lib.rs` with no guarantees on stability

## [0.1.0] - 2020-07-23
### Changed
- [Major reimplementation of download and upload test implementation to be more accurate to speedtest-cli.](https://github.com/nelsonjchen/speedtest-rs/pull/74)
  That said, speedtest-cli isn't too accurate on high bandwidth connections.
  A future alternate or default accurate implementation will need to be like socket based speedtest tools.
- Replaced xml-rs implementation with roxmltree implementation.

### Added
- Upload tests no longer pre-allocate the test upload content. It is now iterator based and generates the uploaded bytes on the fly.
  Speedtest-cli takes ~300-400MB of RAM to preallocate upload request data on my connection to do its upload tests.
  Speedtest-rs now takes ~8MB.

## [0.0.15] - 2020-07-11
### Added
- [Mini server support with `--mini`](https://github.com/nelsonjchen/speedtest-rs/pull/72)

## [0.0.14] - 2020-03-05
### Added
- CSV output support in the form of `--csv` and `--csv-header`

## [0.0.13] - 2020-02-09
### Changed
- Swapped out MD5 crate to simpler version
- Replaced Error Chain with plain old error enums.

### Added
- `rustls-tls` feature to use `rustls-tls` in reqwest.

## [0.0.12] - 2019-10-13
### Fixed
- [Skip servers if the latency test failed.](https://github.com/nelsonjchen/speedtest-rs/pull/22)
### Changed
- Update dependencies

## [0.0.11] - 2019-02-04
### Changed
- Update dependencies and followed the API changes
- Updated to Rust 2018

## [0.0.10] - 2017-12-03
### Changed
- Update infrastructure and ensure things still build on beta and nightly as of
  release.
- Lay out initial foundation for a "error-chain" approach instead of unwraps
  everywhere. This may be replaced later with the "failure" crate. WIP.
- Update some internal formatting to modern Rust. WIP.

## [0.0.9] - 2016-12-22
### Changed
- Swap out usage of hyper directly with reqwest.

- `speedtest-rs` now uses the platform's native TLS implementation. Compile
  issues on Windows or Mac due to OpenSSL issues or sheanigans are no
  longer an issue.

## [0.0.8] - 2016-08-14

### Changed

- Updated dependencies. In particular, updated to the new `url` crate API.

## [0.0.7] - 2016-01-27

### Changed

- Update progress bar behavior to be more like `speedtest-cli` by displaying
  attempts to upload a file piece instead of completion.

## [0.0.6] - 2016-01-25

### Changed

- Correct issue with confusion on maths used to calculate bits and bytes. I
  should probably code when I'm awake and not when I'm tired, exhausted, and
  delirious. Fix was put in while I'm delirious so who knows if this works!
- Fixed issue where not using `--bytes` results in "Mbytes/s" output even
  though output is "Mbit/s".

## [0.0.5] - 2016-01-15

### Changed

- Also applied omitted 10 second test run limit to download.

## [0.0.4] - 2015-12-24

### Added

- Added `--share` to generate and provide an URL to the speedtest.net share
  results image.

## [0.0.3] - 2015-12-23

### Changed

- Server list URL changed to non-static version. The static version appears to
  have been taken down for good this time.


## [0.0.2] - 2015-12-04

### Added

- Add `--simple` flag which prints out results but not progress bars simular to
  `speedtest-cli`.
- Generate User Agent string from crate version

### Changed
- Made latency test determination a lot more like `speedtest-cli`'s weird
  metric for "averaging". Not sure if fix but they say it was intentional.


## [0.0.1] - 2015-11-18

### Added

- Progress indicators and "TUI" like `speedtest-cli`
- Test download speed like `speedtest-cli`
- Test upload speed like `speedtest-cli`
- Option to display values in bytes instead.... also like `speedtest-cli`.
