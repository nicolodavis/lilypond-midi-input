mod args;
mod data;
mod error;
mod handler;

use clap::Parser;
use data::Data;
use error::{Error, Result};
use midir::{MidiInput, MidiInputPort};
use midly::{live::LiveEvent, MidiMessage};

/// Called on each MIDI event that flows through the input port.
fn on_midi_event(_timestamp: u64, event: &[u8], data: &mut Data) {
    let event = LiveEvent::parse(event).unwrap();
    match event {
        LiveEvent::Midi {
            channel: _,
            message,
        } => match message {
            MidiMessage::NoteOn { key, vel } => {
                if vel > 0 {
                    handler::process_key(key.as_int(), data);
                }
            }
            _ => {}
        },
        _ => {}
    }
}

/// Get the input MIDI port that we are going to listen to for events.
fn get_input_port(client: &MidiInput, data: &Data) -> Result<MidiInputPort> {
    let mut names = Vec::new();
    for port in client.ports() {
        let name = client.port_name(&port)?;
        names.push(name.clone());
        if name.contains(&data.args.port) {
            return Ok(port.clone());
        }
    }

    println!("Available ports:");
    for name in &names {
        println!("{name}");
    }

    Err(Error::PortNotFound)
}

fn main() -> Result<()> {
    let args = args::Args::parse();
    let data = Data::new(&args);
    let client = MidiInput::new("lilypond-midi-entry")?;
    let port = get_input_port(&client, &data)?;
    let _connection = client.connect(&port, "midi-through", on_midi_event, data)?;

    loop {
        std::thread::sleep(std::time::Duration::from_millis(2));
    }

    #[allow(unreachable_code)]
    Ok(())
}
