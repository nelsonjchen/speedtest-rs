# Maintenance Message

I’m sorry for the pun, but I no longer have the bandwidth to maintain or develop this project. I also don’t have the bandwidth to search for or evaluate new maintainers—and, frankly, I’m not terribly interested in doing so after recent experiences in other open-source projects.

When I originally built this project, the goal was to port speedtest-cli (a Python tool) to Rust for use on an ARM9/ARM926EJ-S receipt printer. Ironically, although I now work for an ISP, my interest in further developing or supporting this project hasn’t increased. This tool technically violates Ookla’s current Terms of Service, and Ookla now provides its own binaries for speed tests. Besides, I’m also not on the ISP team that handles speed testing and/or uses iperf.

As of now, this project is officially mothballed. I will not be accepting pull requests for code changes or updating the crate, at least for the foreseeable future. However, I will accept pull requests related to the list of alternative projects below. You’re welcome to fork this project and give it a new name, which I’d be happy to add to this list.

Alternatives (ordered by GitHub stars at the time of PR):

* to be filled

# speedtest-rs

*a tool like `speedtest-cli`, but in Rust*

![Continuous integration](https://github.com/nelsonjchen/speedtest-rs/workflows/Continuous%20integration/badge.svg)
[![](https://img.shields.io/crates/v/speedtest-rs.svg)](https://crates.io/crates/speedtest-rs)

Status: This is usable for lower-end residential connections using ["HTTP Legacy Fallback"][http_legacy_fallback]

## Install from AUR

```sh
paru -S speedtest-rs
```

or

```sh
paru -S speedtest-rs-bin
```

## [HTTP Legacy Fallback][http_legacy_fallback]

This tool currently only supports [HTTP Legacy Fallback][http_legacy_fallback] for testing.

High bandwidth connections higher than ~200Mbps may return incorrect results!

The testing operations are different from socket versions of tools connecting to speedtest.net infrastructure. In the many FOSS Go versions, tests are done to find an amount of data that can run for a default of 3 seconds over some TCP connection. In particular, `speedtest-cli` and `speedtest-rs` tests with what Ookla calls the ["HTTP Legacy Fallback"][http_legacy_fallback] for hosts that cannot establish a direct TCP connection.

### Ookla speedtest now has their own non-FOSS CLI tool that's native and available for many platforms.

* TCP-based
* Higher Bandwidth capable.

https://www.speedtest.net/apps/cli

Please look here. Unfortunately, it is not FOSS. Still, it is supported by them and can be used for non-commercial purposes.

## Purpose

This is a learning exercise for me to learn Rust and keeping up with its ecosystem.

The [HTTP Legacy Fallback][http_legacy_fallback] is currently based on the popular Python implementation:

https://github.com/sivel/speedtest-cli @ 2.1.2

There are also other speedtest.net using tools using different approaches to be stolen from in the future. For example:

https://github.com/traetox/speedtest

This example seems different as it appears to just use TCP connections and some protocol. It's probably more suitable to high-speed connections. TODO: Add a default TCP-mode.

## Use as a Library

The API is very much not stable. Use at your own risk. Semver adherence definitely not guaranteed. Please lock to exact versions if you must.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

[http_legacy_fallback]: https://web.archive.org/web/20161109011118/http://www.ookla.com/support/a84541858
