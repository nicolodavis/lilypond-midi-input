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

## Running

```
lilypond-midi-entry --port KeyboardName
```

## Compatibility

Uses xdo to emit key events on Linux running X11. No support for Windows / Mac yet.
