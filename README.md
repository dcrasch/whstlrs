# WHSTLRS

Whstlrs is a cross-platform midi visualizer, build in Rust,
It helps people to practice song on a MIDI wind controller.

## Supported operating systems

* Linux Raspberry PI
* Macos
* Windows 10

## Requirements

* midi wind controller
* graphics card supported by WGPU.

## Screenshot

<img src="https://github.com/dcrasch/whstlrs/assets/2651634/a0e136b1-2fb0-4f4a-aead-e461b0cc6914" width="800"/>

## Development

* https://www.rust-lang.org/
* https://lilypond.org/ for sheet music

## Show log

Bash:
```
RUST_LOG=info ./cargo run
```

Powershell:
```
$Env:RUST_LOG = "info"
cargo run
```

## tools

* abc2ly -> lilypond format -> notes, midi, svg

## Graphics libraries

* https://wgpu.rs/
* https://github.com/RazrFalcon/resvg
* https://github.com/nical/lyon/

## Music libraries

* https://github.com/kovaxis/midly
* https://github.com/Boddlnagg/midir

## Other graphics tools

* https://inkscape.org/

## Other music tools

* https://abcnotation.com/
* https://lilypond.org/
* https://en.wikipedia.org/wiki/Tin_whistle
* https://www.frescobaldi.org/
* https://michaeleskin.com/abctools/abctools.html

## Inspiration

* https://charleslabs.fr/en/project-Irish+Whistle+Tab+-+Android+app
* https://github.com/PolyMeilex/Neothesia/

## Songs

* https://thesession.org/

## MIDI wind controlers

* https://warbl.xyz
* https://github.com/amowry/warbl

# TODO

- [ ] hover notes show fingering
- [ ] import of tune from the session
- [ ] UI with settings and load song

# License

This code is distributed under a GPL v3 style license, see the LICENSE file for complete information.

# Copyright

Whstlrs Copyright (c) 2024 David Rasch

