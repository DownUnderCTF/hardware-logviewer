Hardware Logs Viewer
===

A log viewer for the hardware challenges, with real-time replay.

```
Usage: hardware-logviewer [OPTIONS] <FILE_NAME>

Arguments:
  <FILE_NAME>  

Options:
  -t <STREAMS>              streams to replay. defaults to all streams. 0 for user input and 1 for cluster output
  -r                        replay logs in real time
  -s <REAL_TIME_SPEED>      speed to replay logs at [default: 1]
  -h, --help                Print help
```

## Installation
1. Follow the instructions to set up rust if you don't already have rust. A good installer is [rustup](https://rustup.rs).
2. Run `cargo build --release`. The viewer will be available at `target/release/hardware-logviewer`.