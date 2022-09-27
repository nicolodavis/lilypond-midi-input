mod error;

use error::{Error, Result};
use midir::{MidiInput, MidiInputPort};
use midly::{live::LiveEvent, MidiMessage};

fn translate(key: u8) -> &'static str {
    match key {
        60 => "c",
        61 => "c+i+s",
        62 => "d",
        63 => "d+i+s",
        64 => "e",
        65 => "f",
        66 => "f+i+s",
        67 => "g",
        68 => "g+i+s",
        69 => "a",
        70 => "a+i+s",
        71 => "b",
        _ => "",
    }
}

fn process_key(key: u8) {
    println!("{}", key);
    let opts = xdotool::optionvec::OptionVec::<xdotool::command::options::KeyboardOption>::new();
    let key = translate(key);
    if !key.is_empty() {
        xdotool::keyboard::send_key(key, opts);
    }
}

fn on_midi(_timestamp: u64, event: &[u8], _data: &mut ()) {
    let event = LiveEvent::parse(event).unwrap();
    match event {
        LiveEvent::Midi {
            channel: _,
            message,
        } => match message {
            MidiMessage::NoteOn { key, vel } => {
                if vel > 0 {
                    process_key(key.as_int());
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
    let _connection = client.connect(&port, "midi-through", on_midi, ())?;

    loop {
        std::thread::sleep(std::time::Duration::from_millis(2));
    }

    Ok(())
}

fn main() {
    run().expect("ok");
}
