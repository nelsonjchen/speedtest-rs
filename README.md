# speedtest-rs

[![Project Status: Concept - Minimal or no implementation has been done yet.](http://www.repostatus.org/badges/0.1.0/concept.svg)](http://www.repostatus.org/#concept)
[![Build Status](https://travis-ci.org/nelsonjchen/speedtest-rust.svg?branch=master)](https://travis-ci.org/nelsonjchen/speedtest-rust)

*This is not really working at the moment. Download and upload is kinda done. I'm just stumbling around. It's going to need major refactoring anyway.*

This is a learning exercise for me to learn:

* How some python stuff maps to Rust
* Rust
* HTTP Libraries in Rust
* XML Parsing in Rust
* Sockets in Rust
* Threads with Rust
* Some really, really dumb napkin distance calculations
* Idiomatic Rust
* what speedtest-cli do anyway?!

If I get this done, this should be a runtime-free tool (with musl especially) that can run against
speedtest.net. Cross-compile this and maybe it can go anywhere! I also hope that this is less demanding on resources than

This will be based heavily on:

https://github.com/sivel/speedtest-cli

For now, it would only support speedtest.net and not Speedtest Mini.

There is also a Go version which is different from the other CLI speedtest clients I've found:

https://github.com/traetox/speedtest

It seems different as it appears to just use TCP connections and some protocol. It's probably more suitable to high-speed connections.

# How this speedtest would work.

This is pretty much cribbed from the Python implementation.

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
  * A `latency.txt` is downloaded from the "directory" where the file of the `url` element is located. This is timed.
  * TCP version just uses some `PING` and `PONG`. (TODO)
1. Download and time GETs for `[350, 500, 750, 1000, 1500, 2000, 2500, 3000, 3500, 4000]` with `random(size)x(size).jpg`, like `/random350x350.jpg` from the fastest server.
  * All this is put into a queue with a capacity of 6 handled by two threads producing and consuming the queue. Each download is launched in its own thread. Smaller downloads must complete before the queue is advanced. If the test has already been running for more than 10 seconds, then the larger files downloads are not initiated. When all the downloads are complete, the resulting time is taken. *Trivia: Each dot in the original `speedtest-cli` is a file download that in the beginning has started and at the latter half is when the downloads are complete which is also when the trickling starts happening*
1. The Download speed is calculated from the sum of all the files and the time to took to download 6 files at a time in parallel from the list.
1. Upload and time POSTs for `[250000(25 times), 500000(25 times)]` where bytes of a rolling `0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ` cycled to the desired size is posted as a request with that `data`. This is timed. It is posted that `url` that is in the server configuration.
  * Similar operation with a queue and all that stuff to the downloads above. Also note the 10 second limit as well.
1. Same calculation as download but ... for upload!?! :scream:
1. There's some sharing stuff but this isn't bothered with for now. The `speedtest-cli`

Note that some of these operations are different on one particular [Go version of the speedtest](https://github.com/traetox/speedtest/blob/master/speedtestdotnet/actions.go). In that version, tests are done to find an amount of data that can run for a default of 3 seconds. In particular, `speedtest-cli` tests with what Ookla calls the ["HTTP Legacy Fallback"](http://www.ookla.com/support/a84541858) for hosts that cannot establish a direct TCP connection.

That's not done here for now so I guess maybe I'll modify the tool with an option to do both?
