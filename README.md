# cosmic-applet-net-speed

A [COSMIC desktop](https://github.com/pop-os/cosmic-epoch) panel applet that shows live network download/upload speed.

It reads interface statistics directly from `/sys/class/net`, picking the active
non-loopback/non-virtual interface with the most traffic, and displays the
current throughput in the panel.

## Building

Requires a recent Rust toolchain.

```sh
cargo build --release
```

## Installing

```sh
sudo install -Dm755 target/release/cosmic-applet-net-speed /usr/bin/cosmic-applet-net-speed
sudo install -Dm644 data/com.github.Balayogi.CosmicNetworkSpeed.desktop \
    /usr/share/applications/com.github.Balayogi.CosmicNetworkSpeed.desktop
```

Then add "Network Speed" from the COSMIC panel applet list.

## Logging

Set `COSMIC_NET_SPEED_LOG` (default `warn`) to control log verbosity, e.g.:

```sh
COSMIC_NET_SPEED_LOG=debug cosmic-applet-net-speed
```

## License

GPL-3.0-only
