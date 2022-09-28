use crate::data::{Data, Mode};

/// Handler for MIDI events.
pub fn process_key(key: u8, data: &mut Data) {
    // C7 toggles the sharp / flat mode.
    if key == 96 {
        data.mode = data.mode.flip();
        return;
    }

    let key = translate(key, data);
    if !key.is_empty() {
        xdotool::keyboard::type_text(key, opts());
        xdotool::keyboard::send_key("space", opts());
    }
}

/// Translate MIDI keys into LilyPond note entries.
fn translate(key: u8, data: &Data) -> &'static str {
    match data.mode {
        Mode::Sharp => match key {
            48 | 60 | 72 | 84 => "c",
            49 | 61 | 73 => "cis",
            50 | 62 | 74 => "d",
            51 | 63 | 75 => "dis",
            52 | 64 | 76 => "e",
            53 | 65 | 77 => "f",
            54 | 66 | 78 => "fis",
            55 | 67 | 79 => "g",
            56 | 68 | 80 => "gis",
            57 | 69 | 81 => "a",
            58 | 70 | 82 => "ais",
            59 | 71 | 83 => "b",
            _ => "",
        },

        Mode::Flat => match key {
            48 | 60 | 72 | 84 => "c",
            49 | 61 | 73 => "des",
            50 | 62 | 74 => "d",
            51 | 63 | 75 => "ees",
            52 | 64 | 76 => "e",
            53 | 65 | 77 => "f",
            54 | 66 | 78 => "ges",
            55 | 67 | 79 => "g",
            56 | 68 | 80 => "aes",
            57 | 69 | 81 => "a",
            58 | 70 | 82 => "bes",
            59 | 71 | 83 => "b",
            _ => "",
        },
    }
}

fn opts() -> xdotool::optionvec::OptionVec<xdotool::command::options::KeyboardOption> {
    xdotool::optionvec::OptionVec::<xdotool::command::options::KeyboardOption>::new()
}
