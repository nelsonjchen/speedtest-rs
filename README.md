# speedtest-rs

*a tool like `speedtest-cli`, but in Rust and lacking a few things*

![Continuous integration](https://github.com/nelsonjchen/speedtest-rs/workflows/Continuous%20integration/badge.svg)
[![](https://img.shields.io/crates/v/speedtest-rs.svg)](https://crates.io/crates/speedtest-rs)

Status: This is working at the moment. Download and upload testing is done.

## Ookla speedtest now has their own CLI tool that's native and available for many platforms.

https://www.speedtest.net/apps/cli

Please look here. Unfortunately, it is not FOSS. Still, it is supported by them and can be used for non-commercial purposes. 

## Purpose

This is a learning exercise for me to learn:

* How some python stuff maps to Rust
* Rust
* HTTP Libraries in Rust
* XML Parsing in Rust
* Sockets in Rust
* Threads with Rust
* CLI apps with nice GUIs with Rust
* Some really, really dumb napkin distance calculations
* Idiomatic Rust
* what speedtest-cli do anyway?!
* Can I make a version that's more efficient in space and time than the Python or Go versions?

Plans may include a runtime-free tool (with musl especially) that can run against
speedtest.net. Cross-compile this and maybe it can go anywhere! I also hope that this is less demanding on resources than the Python or Go versions.

This is currently based heavily on the popular Python implementation:

https://github.com/sivel/speedtest-cli @ 0.3.4

For now, it would only support speedtest.net and not Speedtest Mini.

There is also a Go version which is different from the other CLI speedtest clients I've found:

https://github.com/traetox/speedtest

It seems different as it appears to just use TCP connections and some protocol. It's probably more suitable to high-speed connections. I'll probably offer it as an alternative option that is also built-in.

## How this speedtest works like `speedtest-cli` and a bit about speedtest.net in general and about `speedtest-go`

1. "http://www.speedtest.net/speedtest-config.php" is downloaded.
1. The `client` attributes are read.
  * `client`'s attributes
    * `ip` - Your IP address.
    * `lat` - GeoIP'd Latitude of your location
    * `lon` - GeoIP'd Longitude of your location
    * `isp` - Your ISP name. e.g. "Time Warner Cable"
    * etc. They are not used.

  In the original Python implementation, `times`, `download`, and `upload`
  elements' attributes are also read.

  * `times` - Saved in the Python implementation but it's not used throughout
    the script.
  * `download` - Saved in the Python implementation. Not used either.
  * `upload` - Ditto.

Deleting these unused elements (`times`, `download`, and `upload`) still lets
the Python implementation work. As far as I know, they are unused. Judging
from the history of the original Python implementation, they were introduced
in the first commit but never removed and stayed in as further refactoring
was done.

1. The server list is downloaded. http://www.speedtest.net/speedtest-servers-static.php
1. Each `server` attributes are read.
  * `lat` - GeoIP'd Latitude of your location
  * `lon` - GeoIP'd Longitude of your location
  * `url` - This is used as the endpoint that is used to calculate speeds.
  * `name` - Name of the server used.
  * `sponsor` - Sponsor of the server
  * etc. and some more like `host`, `id`, and `country` but they are not used. `host` is used in the TCP connection test (as in the Go version) which isn't supported yet here.
1. The five closest servers are found by the latitude and longitude with the
   original `client` attribute from the configuration.
1. These five servers are tested for latency and the best server is selected.
  * A `latency.txt` is downloaded from the "directory" where the file of the `url` element is located. This is timed and 3 samples are made and then averaged by dividing by 6 (???, ["It was purposeful."](https://github.com/sivel/speedtest-cli/pull/199)). Samples that aren't `200` or `test=test` will have a sample of "1 hour" recorded. I think this is just a sentinel value to say "don't use this server".
  * TCP version just uses some `PING` and `PONG`. (TODO)
1. Download and time GETs for `[350, 500, 750, 1000, 1500, 2000, 2500, 3000, 3500, 4000]` with `random(size)x(size).jpg`, like `/random350x350.jpg` from the fastest server.
  * All this is each put into a queue four times with a capacity of 6 handled by two threads producing and consuming the queue. Each download is launched in its own thread. Smaller downloads must complete before the queue is advanced. If the test has already been running for more than 10 seconds, then the larger files downloads are not initiated. When all the downloads are complete, the resulting time is taken. *Trivia: Each dot in the original `speedtest-cli` is a file download that in the beginning has started and at the latter half is when the downloads are complete which is also when the trickling starts happening*
1. The Download speed is calculated from the sum of all the files and the time to took to download 6 files at a time in parallel from the list.
1. Upload and time POSTs for `[250000(25 times), 500000(25 times)]` where bytes of a rolling `0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ` cycled to the desired size is posted as a request with that `data`. This is timed. It is posted that `url` that is in the server configuration.
  * Similar operation with a queue and all that stuff to the downloads above. Also note the 10 second limit as well.
1. Same calculation as download but ... for upload!?! :scream:
1. There's some sharing stuff but that hasn't been investigated yet. It it probably pretty simply.

Note that some of these operations are different on one particular [Go version of the speedtest](https://github.com/traetox/speedtest/blob/master/speedtestdotnet/actions.go). In that version, tests are done to find an amount of data that can run for a default of 3 seconds. In particular, `speedtest-cli` tests with what Ookla calls the ["HTTP Legacy Fallback"](http://www.ookla.com/support/a84541858) for hosts that cannot establish a direct TCP connection.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
