# deep_walk

[![license-img][]] [license]
[![travis-img][]] [travis]
[![appveyor-img][]] [appveyor]
[![coveralls-img][]] [coveralls]
[![clippy-img][]] [clippy]

## Overview

Recursive directory/archive walk.

## Documentation

TODO: docs.rs

## Usage

To use this crate, add deep_walk as a dependency to your project's `Cargo.toml`:

```
[dependencies]
deep_walk = "0.0.1"
```

## Example

```rust,no_run
use deep_walk::Walk;

for entry in Walk::new("path") {
    // TODO: FIXME.
}
```

## License

Deep_walk is licensed under the MIT license (see the `LICENSE` file).

[license-img]: http://img.shields.io/badge/license-MIT-blue.svg
[license]: https://github.com/DarkEld3r/rrthozopsi/blob/master/LICENSE
[travis-img]: https://travis-ci.org/DarkEld3r/rrthozopsi.png?branch=master
[travis]: https://travis-ci.org/DarkEld3r/rrthozopsi
[appveyor-img]: https://ci.appveyor.com/api/projects/status/tso08ghhxgvt8fv0/branch/master?svg=true
[appveyor]: https://ci.appveyor.com/project/DarkEld3r/rrthozopsi
[coveralls-img]: https://coveralls.io/repos/DarkEld3r/rrthozopsi/badge.svg?branch=master&service=github
[coveralls]: https://coveralls.io/github/DarkEld3r/rrthozopsi?branch=master
[clippy-img]: http://clippy.bashy.io/github/DarkEld3r/rrthozopsi/master/badge.svg
[clippy]: http://clippy.bashy.io/github/DarkEld3r/rrthozopsi/master/log
