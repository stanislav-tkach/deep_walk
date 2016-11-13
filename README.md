# deep_walk

[![license-img][]] [license]
[![travis-img][]] [travis]
[![appveyor-img][]] [appveyor]
[![coveralls-img][]] [coveralls]
[![dependency-img][]] [dependency]
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
[license]: https://github.com/DarkEld3r/deep_walk/blob/master/LICENSE
[travis-img]: https://travis-ci.org/DarkEld3r/deep_walk.png?branch=master
[travis]: https://travis-ci.org/DarkEld3r/deep_walk
[appveyor-img]: https://ci.appveyor.com/api/projects/status/x95jhmdn6or16581?svg=true
[appveyor]: https://ci.appveyor.com/project/DarkEld3r/deep_walk
[coveralls-img]: https://coveralls.io/repos/github/DarkEld3r/deep_walk/badge.svg?branch=master
[coveralls]: https://coveralls.io/github/DarkEld3r/deep_walk?branch=master
[dependency-img]: https://dependencyci.com/github/DarkEld3r/deep_walk/badge
[dependency]: https://dependencyci.com/github/DarkEld3r/deep_walk
[clippy-img]: https://clippy.bashy.io/github/DarkEld3r/deep_walk/master/badge.svg
[clippy]: https://clippy.bashy.io/github/DarkEld3r/deep_walk/master/log
