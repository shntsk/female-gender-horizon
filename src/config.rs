//! Settings of the layer. Defaults of `Breasts` in the original: size 0.6, cleavage 0, offsets 0,
//! uniboob on, physics on.

#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Breast size, `0.0..=1.0` (values above 0.7 are drawn larger, as in the original).
    pub size: f32,
    /// Gap between the breasts, `0.0..=0.1`; it becomes an outward rotation of up to 10 degrees.
    pub cleavage: f32,
    /// `[x, y, z]` offsets, in 1/16 block.
    pub offset: [f32; 3],
    /// Both breasts move together. Only matters with physics (Phase 2), so nothing reads it yet.
    #[allow(dead_code)]
    pub uniboob: bool,
    /// Draw the jacket overlay (second skin layer) over the breasts.
    pub jacket: bool,
    /// Rise and fall with the player's age, as the original's idle breathing.
    pub breathing: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            size: 0.6,
            cleavage: 0.0,
            offset: [0.0; 3],
            uniboob: true,
            jacket: true,
            breathing: true,
        }
    }
}
