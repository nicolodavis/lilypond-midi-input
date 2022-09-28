# LilyPond MIDI Entry

Enter LilyPond notes like "c", "d", "gis" etc. using a MIDI keyboard.

## Features

- Supports two octaves below and above middle C.
- Toggle between sharp and flat note entries using an assignable midi key.

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
lilypond-midi-entry --port KeyboardName
```
