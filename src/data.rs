pub struct Data {
    pub mode: Mode,
}

impl Default for Data {
    fn default() -> Self {
        Self { mode: Mode::Sharp }
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
