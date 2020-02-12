# blockish-caca

video players in the terminal with [blockish](https://github.com/yazgoo/blockish) (unicode blocks) over libcaca with LD_PRELOAD magic

[![Example](images/sintel.gif)](https://www.youtube.com/watch?v=HminM4V40iI)

# building it

```shell
$ cargo build --release
```

# Using it as a crate

See [blockish-player](https://github.com/yazgoo/blockish-player)

# using it (rust)

```shell
$ cargo run mplayer path/to/your/video
```

```shell
$ cargo run mpv path/to/your/video
```

```shell
$ cargo run cvlc path/to/your/video
```

# using it (terminal)

```shell
$ CACA_DRIVER=raw LD_PRELOAD=target/release/libcaca_blockish.so mplayer -vo caca path/to/your/video
```

```shell
$ CACA_DRIVER=raw LD_PRELOAD=target/release/libcaca_blockish.so mplayer -vo caca path/to/your/video
```

```shell
$ DISPLAY="" CACA_DRIVER=raw LD_PRELOAD=target/release/libcaca_blockish.so:/usr/lib/x86_64-linux-gnu/libcaca.so cvlc --quiet -V caca /path/to/your/video
```

# using it (shell script)

```shell
$ ./mplayer path/to/your/video
```

```shell
$ ./mpv path/to/your/video
```

```shell
$ ./vlc path/to/your/video
```
