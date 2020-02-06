# blockish-caca

video players in the terminal with [blockish](https://github.com/yazgoo/blockish) (utf-8 blocks) over libcaca with LD_PRELOAD magic

[![Alt text](https://img.youtube.com/vi/HminM4V40iI/0.jpg)](https://www.youtube.com/watch?v=HminM4V40iI)

# building it

```shell
$ cargo build --release
```

# using it (rust)

```shell
$ cargo run mplayer path/to/your/video
```

```shell
$ cargo run mpv path/to/your/video
```

# using it (shell)

```shell
$ ./mplayer path/to/your/video
```

```shell
$ ./mpv path/to/your/video
```
