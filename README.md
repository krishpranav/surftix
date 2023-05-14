# Surftix

# About:
- Surftix is an open-source search-engine.

# Features:
- Easy-To-Use 😊
- Secure 🔒
- Fast 🚀
- 1000% FREE ✅
- NO ADS 🚫

# Installation:

## Pre-Requirements:
- macOS/Win/Linux
- 4GiB+
- 10GiB Interal Storage+
- latest [rust](https://www.rust-lang.org/tools/install) version
- redis-server

## Building:
```
$ git clone https://github.com/krishpranav/surftix.git
$ cd surftix
$ cargo build --release
$ redis-server --port 8082 & ./target/release/surftix
```

## API:
- check out the [API.md](https://github.com/krishpranav/surftix/blob/master/docs/API.md) for more details on how the backend works for this engine.

## Troubleshooting:
- most of the times when you try running the builded binary it throws an error which shows you to run the project with ```RUST_BACKTRACE=1```, to avoid that try doing ```cargo run```

- if the search-request doesn't work try running the binary with ```redis-server --port 8082 ``` infront of the ```./target/release/surftix```

# Contributing:
## Guide:
- surftix is an open-source projects and contributions are always welcomed!
- look at the [CONTRIBUTING.md](https://github.com/krishpranav/surftix/blob/master/CONTRIBUTING.md) file for more guide regarding the contributions.

# License:
- surftix is licensed under MIT-License