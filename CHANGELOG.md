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
