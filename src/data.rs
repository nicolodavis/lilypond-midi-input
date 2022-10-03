use crate::args::Args;
use crate::macros::Macros;

pub struct Data {
    pub args: Args,
    pub macros: Macros,
    pub mode: Mode,
    pub duration: Duration,
    pub duration_dot: bool,
    pub macro_mode: bool,
}

impl Data {
    pub fn new(args: &Args, macros: &Macros) -> Self {
        Self {
            args: args.clone(),
            macros: macros.clone(),
            mode: Mode::Sharp,
            duration: Duration::D0,
            duration_dot: false,
            macro_mode: false,
        }
    }
}

/// Determines if we will be entering sharp or flat keys
/// when the black keys are pressed.
#[derive(Debug)]
pub enum Duration {
    D0,
    D1,
    D2,
    D4,
    D8,
    D16,
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Duration::D0 => write!(f, ""),
            Duration::D1 => write!(f, "1"),
            Duration::D2 => write!(f, "2"),
            Duration::D4 => write!(f, "4"),
            Duration::D8 => write!(f, "8"),
            Duration::D16 => write!(f, "16"),
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
