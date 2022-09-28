# LilyPond MIDI Entry

A program that allows using a MIDI keyboard to enter LilyPond notes like "c", "d", "gis" etc.

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
lilypond-midi-entry --input-port KeyboardName
```
