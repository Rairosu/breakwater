# breakwater
breakwater is a very fast [Pixelflut](https://wiki.cccgoe.de/wiki/Pixelflut) server written in Rust. It is heavily inspired by [Shoreline](https://github.com/TobleMiner/shoreline).

It claims to be **the fastest Pixelflut server in existence** - at least at the time of writing 02/2022.

# Features
1. Accepts Pixelflut commands
2. Provides VNC server so that everybody to watch
3. Exposes Prometheus metrics
4. IPv6 and legacy IP support

# Available Pixelflut commands
Commands must be sent newline-separated, for more details see [Pixelflut](https://wiki.cccgoe.de/wiki/Pixelflut)
* `HELP`: Prints a help text with the available commands.
* `PX x y rrggbb`: Color the pixel (x,y) with the given hexadecimal color, e.g. `PX 10 10 ff0000`
* `PX x y rrggbbaa`: Color the pixel (x,y) with the given hexadecimal color rrggbb (alpha channel is ignored for now), e.g. `PX 10 10 ff0000ff`
* `PX x y`: Get the color value of the pixel (x,y), e.g. `PX 10 10`
* `SIZE`: Get the size of the drawing surface

# Usage
The easiest way is to continue with the provided [Ready to use Docker setup](#ready-to-use-docker-setup) below.

If you prefer the manual way (the best performance) you need to have [Rust installed](https://www.rust-lang.org/tools/install).
Then you can directly run the server with
```
cargo run --release
```
The default settings should provide you with a ready-to-use server.

| Port | Description                 |
|------|-----------------------------|
| 1234 | Pixelflut server            |
| 5900 | VNC server                  |
| 9090 | Prometheus metrics exporter |

The get a list of options try
```
cargo run --release -- --help
    Finished release [optimized] target(s) in 0.03s
     Running `target/release/breakwater --help`
breakwater 0.0.1

USAGE:
    breakwater [OPTIONS]

OPTIONS:
    -f, --fps <FPS>
            Frames per second the VNC server should aim for [default: 30]

    -h, --height <HEIGHT>
            Height of the drawing surface [default: 720]

        --help
            Print help information

    -l, --listen-address <LISTEN_ADDRESS>
            Listen address to bind to. The default value will listen on all interfaces for IPv4 and
            v6 packets [default: [::]:1234]

    -p, --prometheus-listen-address <PROMETHEUS_LISTEN_ADDRESS>
            Listen address zhe prometheus exporter should listen om. The default value will listen
            on all interfaces for IPv4 and v6 packets [default: [::]:9090]

    -t, --text <TEXT>
            Text to display on the screen. The text will be followed by "on <listen_address>"
            [default: "Breakwater Pixelflut server"]

    -V, --version
            Print version information

    -w, --width <WIDTH>
            Width of the drawing surface [default: 1280]

```
You can also build the binary with `cargo build --release`. The binary will be placed at `target/release/breakwater`.

# Performance
My Laptop has a `Intel(R) Core(TM) i7-8850H CPU @ 2.60GHz` (6 Cores/12 Threads) and 2 DDR4 RAM modules with 16 GB each and 2667 MT/s.
The Pixelflut-server and Pixelflut-client [Sturmflut](https://github.com/TobleMiner/sturmflut) both run on my Laptop using 24 connections.
These are the results of different Pixelflut servers:

| Server                                                                  | Language | Traffic during first 30s | When thermal throttling |
|-------------------------------------------------------------------------|----------|--------------------------|-------------------------|
| [Pixelnuke](https://github.com/defnull/pixelflut/tree/master/pixelnuke) | C        | 1.1 Gbit/s               | 1 Gbit/s                |
| [Pixelwar](https://github.com/defnull/pixelflut/tree/master/pixelwar)   | Java     | 2.1 Gbit/s               | 1.6 Gbit/s              |
| [Shoreline](https://github.com/TobleMiner/shoreline)                    | C        | 15 Gbit/s                | 12 Gbit/s               |
| [Breakwater](https://github.com/sbernauer/breakwater)                   | Rust     | 30 Gbit/s                | 22 Gbit/s               |

Test with a real server only running the server (not also the client) will follow.

# Ready to use Docker setup
Will follow shorty ;)
Will contain [VNCmux](https://github.com/TobleMiner/vncmux), Prometheus and Grafana similar to https://github.com/sbernauer/pixelflut-infrastructure 

# TODOs
* Implement proper ring buffer or at least complete parsing the current buffer.
Currently, if the buffer will be filled with all - 128,000 bytes - the last 21 bytes are skipped so that we don't need to check for buffer boundaries
This is done for performance reasons. This will cause 1 of about 6,100 commands to be dropped (like 0,016%).
Ideally we would save us the remaining few bytes and add them to beginning of the next processing loop.
* Implement Alpha channel feature. For performance reasons there should be a compile-time switch (similar to `#ifdef`).
Actually haven't checked if Rust supports this ;)
* Finish Docker compose setup with VncMux, Prometheus and Grafana.
