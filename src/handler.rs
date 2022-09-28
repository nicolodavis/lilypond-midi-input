use crate::data::{Data, Mode};
use enigo::KeyboardControllable;

/// Handler for MIDI events.
pub fn process_key(key: u8, data: &mut Data) {
    if key == data.args.flat_toggle {
        data.mode = data.mode.flip();
        return;
    }

    let mut enigo = enigo::Enigo::new();
    let key = translate(key, data);
    if !key.is_empty() {
        enigo.key_sequence(&format!("{key} "));
    }
}

/// Translate MIDI keys into LilyPond note entries.
fn translate(key: u8, data: &Data) -> String {
    let note = note_with_octave_modifier(key, data);
    note
}

/// Calculate the note with an appropriate octave modifier.
fn note_with_octave_modifier(key: u8, data: &Data) -> String {
    match key {
        36..=47 => format!("{},", raw_note(key + 12, data)),
        48..=59 => format!("{}", raw_note(key, data)),
        60..=71 => format!("{}'", raw_note(key - 12, data)),
        72..=83 => format!("{}''", raw_note(key - 24, data)),
        _ => "".to_string(),
    }
}

/// Calculate the note without octave modifiers.
fn raw_note(key: u8, data: &Data) -> &'static str {
    match data.mode {
        Mode::Sharp => match key {
            48 => "c",
            49 => "cis",
            50 => "d",
            51 => "dis",
            52 => "e",
            53 => "f",
            54 => "fis",
            55 => "g",
            56 => "gis",
            57 => "a",
            58 => "ais",
            59 => "b",
            _ => "",
        },

        Mode::Flat => match key {
            48 => "c",
            49 => "des",
            50 => "d",
            51 => "ees",
            52 => "e",
            53 => "f",
            54 => "ges",
            55 => "g",
            56 => "aes",
            57 => "a",
            58 => "bes",
            59 => "b",
            _ => "",
        },
    }
}
