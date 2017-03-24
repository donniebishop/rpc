rpc
===

`rpc` is a terminal client for the Music Player Daemon (MPD), implemented in [Rust](https://rust-lang.org). `rpc` connects to an MPD and contrlols it via commands and arguments passed to it.

```
USAGE:
    rpc [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
        --help       Prints help information
    -q, --quiet      Silences output of mpd status on completion of commands
    -V, --version    Prints version information

OPTIONS:
    -h, --host <HOST>    The host to connect to
    -p, --port <PORT>    The port to connect to

SUBCOMMANDS:
    clear      Empties playlist
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
    version    Reports the version of MPD
```

Deep influence for rpc is taken from [`mpc`](https://github.com/MaxKellermann/mpc).
