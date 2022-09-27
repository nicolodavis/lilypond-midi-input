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
            60 => "c",
            61 => "cis",
            62 => "d",
            63 => "dis",
            64 => "e",
            65 => "f",
            66 => "fis",
            67 => "g",
            68 => "gis",
            69 => "a",
            70 => "ais",
            71 => "b",
            _ => "",
        },

        Mode::Flat => match key {
            60 => "c",
            61 => "des",
            62 => "d",
            63 => "ees",
            64 => "e",
            65 => "f",
            66 => "ges",
            67 => "g",
            68 => "aes",
            69 => "a",
            70 => "bes",
            71 => "b",
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

fn get_through_port(client: &MidiInput) -> Result<MidiInputPort> {
    let midi_through_port =
        std::env::var("MIDI_THROUGH_PORT").unwrap_or("Midi Through".to_string());
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
    let port = get_through_port(&client)?;
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
