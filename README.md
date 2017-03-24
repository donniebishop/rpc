rpc
===

`rpc` is a terminal client for the Music Player Daemon (MPD), implemented in [Rust](https://rust-lang.org). `rpc` connects to an MPD and contrlols it via commands and arguments passed to it.

```
USAGE:
    rpc [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    consume    Toggle consume mode
    help       Prints this message or the help of the given subcommand(s)
    next       Plays next track
    pause      Pauses playing
    play       Starts playing
    prev       Plays previous track
    random     Toggle randomized track playback
    repeat     Toggle repeat mode
    shuffle    Shuffle playlist order
    single     Toggle single mode if state (on|off) is not specified
    stop       Stops playing
    toggle     Toggles between play and pause
```

Deep influence for rpc is taken from [`mpc`](https://github.com/MaxKellermann/mpc).
