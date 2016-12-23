## 0.0.9 (Dec 22, 2016)

Swap out usage of hyper directly with reqwest.

### Features

* `speedtest-rs` now uses the platform's native TLS implementation. Compile
  issues on Windows or Mac due to OpenSSL issues or sheanigans are no
  longer an issue.

## 0.0.8 (Aug 14, 2016)

Maintenance and Progress Release

### Fixes

* Updated dependencies. In particular, updated to the new `url` crate API.

## 0.0.7 (Jan 27, 2016)

### Fixes

* Update progress bar behavior to be more like `speedtest-cli` by displaying
  attempts to upload a file piece instead of completion.

## 0.0.6 (Jan 25, 2016)

### Fixes

* Correct issue with confusion on maths used to calculate bits and bytes. I
  should probably code when I'm awake and not when I'm tired, exhausted, and
  delirious. Fix was put in while I'm delirious so who knows if this works!
* Fixed issue where not using `--bytes` results in "Mbytes/s" output even
  though output is "Mbit/s".

## 0.0.5 (Jan 15, 2016)

### Fixes

* Also applied omitted 10 second test run limit to download.

## 0.0.4 (Dec 24, 2015)

### Features

* Added `--share` to generate and provide an URL to the speedtest.net share
  results image.

## 0.0.3 (Dec 23, 2015)

### Fixes

* Server list URL changed to non-static version. The static version appears to
  have been taken down for good this time.


## 0.0.2 (Dec 4, 2015)

Maintenance and Progress Release

### Features

* Add `--simple` flag which prints out results but not progress bars simular to
  `speedtest-cli`.

### Fixes

* Generate User Agent string from crate version
* Made latency test determination a lot more like `speedtest-cli`'s weird
  metric for "averaging". Not sure if fix but they say it was intentional.


## 0.0.1 (Nov 18, 2015)

Initial Release.

### Features

* Progress indicators and "TUI" like `speedtest-cli`
* Test download speed like `speedtest-cli`
* Test upload speed like `speedtest-cli`
* Option to display values in bytes instead.... also like `speedtest-cli`.
