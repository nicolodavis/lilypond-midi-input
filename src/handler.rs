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
fn translate(key: u8, data: &Data) -> &'static str {
    match data.mode {
        Mode::Sharp => match key {
            36 | 48 | 60 | 72 | 84 => "c",
            37 | 49 | 61 | 73 => "cis",
            38 | 50 | 62 | 74 => "d",
            39 | 51 | 63 | 75 => "dis",
            40 | 52 | 64 | 76 => "e",
            41 | 53 | 65 | 77 => "f",
            42 | 54 | 66 | 78 => "fis",
            43 | 55 | 67 | 79 => "g",
            44 | 56 | 68 | 80 => "gis",
            45 | 57 | 69 | 81 => "a",
            46 | 58 | 70 | 82 => "ais",
            47 | 59 | 71 | 83 => "b",
            _ => "",
        },

        Mode::Flat => match key {
            36 | 48 | 60 | 72 | 84 => "c",
            37 | 49 | 61 | 73 => "des",
            38 | 50 | 62 | 74 => "d",
            39 | 51 | 63 | 75 => "ees",
            40 | 52 | 64 | 76 => "e",
            41 | 53 | 65 | 77 => "f",
            42 | 54 | 66 | 78 => "ges",
            43 | 55 | 67 | 79 => "g",
            44 | 56 | 68 | 80 => "aes",
            45 | 57 | 69 | 81 => "a",
            46 | 58 | 70 | 82 => "bes",
            47 | 59 | 71 | 83 => "b",
            _ => "",
        },
    }
}
