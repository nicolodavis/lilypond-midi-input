mod args;
mod data;
mod error;
mod handler;
mod macros;

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
                    handler::process_note_on(key.as_int(), data);
                }

                if vel == 0 {
                    handler::process_note_off(key.as_int(), data);
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
        println!("input port: {name}");
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

/// Read macros from the config file.
fn read_macros(args: &args::Args) -> Result<macros::Macros> {
    let s = if args.macro_file.is_empty() {
        resource::resource_str!("macros.config").to_string()
    } else {
        std::fs::read_to_string(&args.macro_file)?
    };

    serde_yaml::from_str(&s).map_err(|_| Error::MacroParsingError)
}

fn main() -> Result<()> {
    let args = args::Args::parse();
    let macros = read_macros(&args)?;
    let data = Data::new(&args, &macros);
    let client = MidiInput::new("lilypond-midi-entry")?;
    let port = get_input_port(&client, &data)?;
    let _connection = client.connect(&port, "midi-through", on_midi_event, data)?;

    loop {
        std::thread::sleep(std::time::Duration::from_millis(2));
    }

    #[allow(unreachable_code)]
    Ok(())
}
