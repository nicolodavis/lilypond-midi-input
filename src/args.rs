use clap::Parser;

/// LilyPond MIDI note entry.
#[derive(Clone, Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The MIDI port to read events from.
    #[clap(short, long, value_parser)]
    pub port: String,

    /// The MIDI event that is used to toggle between sharp / flat modes.
    #[clap(short, long, value_parser, default_value_t = 96)]
    pub flat_toggle: u8,
}
