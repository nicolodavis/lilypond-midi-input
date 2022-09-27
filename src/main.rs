mod error;

use error::{Error, Result};
use midir::{MidiInput, MidiInputPort};
use midly::{live::LiveEvent, MidiMessage};

fn on_midi(_timestamp: u64, event: &[u8], _data: &mut ()) {
    let event = LiveEvent::parse(event).unwrap();
    match event {
        LiveEvent::Midi { channel, message } => match message {
            MidiMessage::NoteOn { key, vel: _ } => {
                println!("hit note {} on channel {}", key, channel);
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
