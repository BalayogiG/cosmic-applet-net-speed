default: build

build:
	cargo build --release

export NAME := 'cosmic-ext-applet-net-speed'
export APPID := 'io.github.balayogig.cosmic-ext-applet-net-speed'

cargo-target-dir := env('CARGO_TARGET_DIR', 'target')
bin-src := cargo-target-dir / 'release' / NAME

rootdir := ''
prefix := '/usr'

base-dir := absolute_path(clean(rootdir / prefix))
share-dst := base-dir / 'share'

bin-dst := base-dir / 'bin' / NAME
desktop-dst := share-dst / 'applications' / APPID + '.desktop'
metainfo-dst := share-dst / 'metainfo' / APPID + '.metainfo.xml'
icon-dst := share-dst / 'icons/hicolor/scalable/apps' / APPID + '-symbolic.svg'

install:
	install -Dm0755 {{ bin-src }} {{ bin-dst }}
	install -Dm0644 data/{{ APPID }}.desktop {{ desktop-dst }}
	install -Dm0644 data/{{ APPID }}.metainfo.xml {{ metainfo-dst }}
	install -Dm0644 data/{{ APPID }}-symbolic.svg {{ icon-dst }}

uninstall:
	rm {{ bin-dst }}
	rm {{ desktop-dst }}
	rm {{ metainfo-dst }}
	rm {{ icon-dst }}
