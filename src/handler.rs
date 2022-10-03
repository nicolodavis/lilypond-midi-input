use crate::data::{Data, Duration, Mode};
use enigo::KeyboardControllable;

/// Handler for MIDI events.
pub fn process_note_on(key: u8, data: &mut Data) {
    if key == data.args.midi_flat_toggle {
        data.mode = data.mode.flip();
        return;
    }

    if key == data.args.midi_duration_dot {
        data.duration_dot = true;
    }

    if key == data.args.midi_macro_toggle {
        data.macro_mode = true;
    }

    if key == data.args.midi_duration_1 {
        data.duration = Duration::D1;
    }

    if key == data.args.midi_duration_2 {
        data.duration = Duration::D2;
    }

    if key == data.args.midi_duration_4 {
        data.duration = Duration::D4;
    }

    if key == data.args.midi_duration_8 {
        data.duration = Duration::D8;
    }

    if key == data.args.midi_duration_16 {
        data.duration = Duration::D16;
    }

    if data.macro_mode {
        process_macro(key, data);
    } else {
        let mut enigo = enigo::Enigo::new();
        let key = translate(key, data);
        if !key.is_empty() {
            enigo.key_sequence(&format!("{key} "));
        }
    }
}

/// Handler for MIDI events.
pub fn process_note_off(key: u8, data: &mut Data) {
    if key == data.args.midi_duration_dot {
        data.duration_dot = false;
    }

    if key == data.args.midi_macro_toggle {
        data.macro_mode = false;
    }

    if key == data.args.midi_duration_1 {
        data.duration = Duration::D0;
    }

    if key == data.args.midi_duration_2 {
        data.duration = Duration::D0;
    }

    if key == data.args.midi_duration_4 {
        data.duration = Duration::D0;
    }

    if key == data.args.midi_duration_8 {
        data.duration = Duration::D0;
    }

    if key == data.args.midi_duration_16 {
        data.duration = Duration::D0;
    }
}

/// Translate the MIDI key into a macro sequence.
fn process_macro(key: u8, data: &Data) {
    if let Some(value) = data.macros.get(key.to_string()).and_then(|t| t.as_str()) {
        let mut enigo = enigo::Enigo::new();
        enigo.key_sequence(value);
    }
}

/// Translate MIDI keys into LilyPond note entries.
fn translate(key: u8, data: &Data) -> String {
    let mut note: Vec<_> = note_with_octave_modifier(key, data).chars().collect();

    if note.is_empty() {
        return "".to_string();
    }

    // Append the duration (if any).
    note.extend(data.duration.to_string().chars());

    // Append a dot if necessary.
    if data.duration_dot {
        note.push('.');
    }

    note.into_iter().collect()
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
            51 => "es",
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
