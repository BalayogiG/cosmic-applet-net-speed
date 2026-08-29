# cosmic-ext-applet-net-speed

A [COSMIC desktop](https://github.com/pop-os/cosmic-epoch) panel applet that shows live network download/upload speed.

It reads interface statistics directly from `/sys/class/net`, picking the active
non-loopback/non-virtual interface with the most traffic, and displays the
current throughput in the panel.

| Panel | Popup |
| --- | --- |
| ![Panel showing live download/upload speed](data/applet_screenshot_1.png) | ![Popup showing interface and speeds](data/applet_screenshot_2.png) |

## Building

Requires a recent Rust toolchain.

```sh
cargo build --release
```

## Installing

```sh
just install
```

or manually:

```sh
sudo install -Dm755 target/release/cosmic-ext-applet-net-speed /usr/bin/cosmic-ext-applet-net-speed
sudo install -Dm644 data/io.github.balayogig.cosmic-ext-applet-net-speed.desktop \
    /usr/share/applications/io.github.balayogig.cosmic-ext-applet-net-speed.desktop
sudo install -Dm644 data/io.github.balayogig.cosmic-ext-applet-net-speed.metainfo.xml \
    /usr/share/metainfo/io.github.balayogig.cosmic-ext-applet-net-speed.metainfo.xml
sudo install -Dm644 data/io.github.balayogig.cosmic-ext-applet-net-speed-symbolic.svg \
    /usr/share/icons/hicolor/scalable/apps/io.github.balayogig.cosmic-ext-applet-net-speed-symbolic.svg
```

Then add "Network Speed" from the COSMIC panel applet list.

## Logging

Set `COSMIC_NET_SPEED_LOG` (default `warn`) to control log verbosity, e.g.:

```sh
COSMIC_NET_SPEED_LOG=debug cosmic-ext-applet-net-speed
```

## Flatpak

The applet is packaged for Flatpak distribution via
[`io.github.balayogig.cosmic-ext-applet-net-speed.json`](io.github.balayogig.cosmic-ext-applet-net-speed.json),
which is how it's submitted to community COSMIC applet repositories such as
[cosmic-flatpak](https://github.com/pop-os/cosmic-flatpak).

To build and test locally:

```sh
./flatpak/generate-cargo-sources.sh   # regenerate cargo-sources.json after Cargo.lock changes
flatpak-builder --user --install --force-clean build-dir \
    io.github.balayogig.cosmic-ext-applet-net-speed.json
flatpak run io.github.balayogig.cosmic-ext-applet-net-speed
```

AppStream metadata can be checked with:

```sh
appstreamcli validate data/io.github.balayogig.cosmic-ext-applet-net-speed.metainfo.xml
```

## License

GPL-3.0-only, see [LICENSE](LICENSE).
