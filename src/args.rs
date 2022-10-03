use clap::Parser;

/// LilyPond MIDI note entry.
#[derive(Clone, Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// MIDI port to read events from.
    #[clap(short, long, value_parser)]
    pub port: String,

    /// File to read macros from.
    #[clap(long, value_parser, default_value = "")]
    pub macro_file: String,

    /// MIDI event that is used to enable duration 1.
    #[clap(long, value_parser, default_value_t = 84)]
    pub midi_duration_1: u8,

    /// MIDI event that is used to toggle between sharp / flat modes.
    #[clap(long, value_parser, default_value_t = 85)]
    pub midi_flat_toggle: u8,

    /// MIDI event that is used to enable duration 2.
    #[clap(long, value_parser, default_value_t = 86)]
    pub midi_duration_2: u8,

    /// MIDI event that is used to enable a dotted duration.
    #[clap(long, value_parser, default_value_t = 87)]
    pub midi_duration_dot: u8,

    /// MIDI event that is used to enable duration 4.
    #[clap(long, value_parser, default_value_t = 88)]
    pub midi_duration_4: u8,

    /// MIDI event that is used to enable duration 8.
    #[clap(long, value_parser, default_value_t = 89)]
    pub midi_duration_8: u8,

    /// MIDI event that is used to enable duration 16.
    #[clap(long, value_parser, default_value_t = 91)]
    pub midi_duration_16: u8,

    // MIDI event that toggles the macro layer.
    #[clap(long, value_parser, default_value_t = 96)]
    pub midi_macro_toggle: u8,
}
