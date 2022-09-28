mod error;

use error::{Error, Result};
use midir::{MidiInput, MidiInputPort};
use midly::{live::LiveEvent, MidiMessage};

struct Data {
    pub mode: Mode,
}

impl Default for Data {
    fn default() -> Self {
        Self { mode: Mode::Sharp }
    }
}

#[derive(Debug)]
enum Mode {
    Sharp,
    Flat,
}

impl Mode {
    pub fn flip(&self) -> Self {
        match self {
            Mode::Sharp => Mode::Flat,
            Mode::Flat => Mode::Sharp,
        }
    }
}

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

fn process_key(key: u8, data: &mut Data) {
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

fn on_midi(_timestamp: u64, event: &[u8], data: &mut Data) {
    let event = LiveEvent::parse(event).unwrap();
    match event {
        LiveEvent::Midi {
            channel: _,
            message,
        } => match message {
            MidiMessage::NoteOn { key, vel } => {
                if vel > 0 {
                    process_key(key.as_int(), data);
                }
            }
            _ => {}
        },
        _ => {}
    }
}

fn get_input_port(client: &MidiInput) -> Result<MidiInputPort> {
    let midi_through_port = std::env::var("MIDI_INPUT_PORT").unwrap_or("Keystation".to_string());
    for port in client.ports() {
        let name = client.port_name(&port)?;
        if name.contains(&midi_through_port) {
            return Ok(port.clone());
        }
    }
    Err(Error::ThroughPortNotFound)
}

fn run() -> Result<()> {
    let client = MidiInput::new("lilypond-midi-entry")?;
    let port = get_input_port(&client)?;
    let data = Data::default();
    let _connection = client.connect(&port, "midi-through", on_midi, data)?;

    loop {
        std::thread::sleep(std::time::Duration::from_millis(2));
    }

    #[allow(unreachable_code)]
    Ok(())
}

fn main() {
    run().expect("ok");
}
