use crate::args::Args;

pub struct Data {
    pub args: Args,
    pub mode: Mode,
}

impl Data {
    pub fn new(args: &Args) -> Self {
        Self {
            args: args.clone(),
            mode: Mode::Sharp,
        }
    }
}

/// Determines if we will be entering sharp or flat keys
/// when the black keys are pressed.
#[derive(Debug)]
pub enum Mode {
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
