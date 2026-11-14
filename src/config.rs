//! Defaults of `Breasts` (size 0.6, cleavage 0, offsets 0, uniboob on, physics on), from the original.
#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub size: f32,
    pub cleavage: f32,
    pub offset: [f32; 3],
    pub uniboob: bool,
    pub jacket: bool,
    pub breathing: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self { size: 0.6, cleavage: 0.0, offset: [0.0; 3], uniboob: true, jacket: true, breathing: true }
    }
}
