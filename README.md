# LilyPond MIDI Input

Enter LilyPond notes like "c", "d", "gis" etc. using a MIDI controller.

## Features

- Supports two octaves below and above middle C.
- Toggle between sharp and flat note entries using an assignable MIDI key.
- Toggle note durations with assignable MIDI keys.

## Building

With a Rust toolchain installed, run:

```
cargo build --release
```

On Linux, you may have to install libxdo-dev.

### Ubuntu
```
apt install libxdo-dev
```

### Arch
```
pacman -S xdotool
```

## Running

```
lilypond-midi-input --port KeyboardName
```
