//! Breast dimensions derived from a [`Config`]: `GenderLayer.setupRender` of the original.
use crate::config::Config;

/// Sizes below this draw nothing, as the original does.
const MIN_VISIBLE: f32 = 0.02;
/// Most outward rotation, in degrees.
const MAX_OUTWARD: f32 = 10.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shape {
    /// Outward rotation of each breast, in degrees (from the cleavage).
    pub outward: f32,
    /// Forward shift along z, in blocks: larger breasts sit further out.
    pub z_offset: f32,
    /// Pitch factor: the breasts tilt by `-35 * rotation` degrees.
    pub rotation: f32,
}

impl Shape {
    /// `None` when the size is too small to draw.
    pub fn from_config(config: &Config) -> Option<Self> {
        let b_size = config.size;
        let outward = (config.cleavage * 100.0).round().min(MAX_OUTWARD);
        let mut size = (b_size * 1.5).min(0.7);
        if b_size > 0.7 {
            size = b_size;
        }
        if size < MIN_VISIBLE {
            return None;
        }
        let z_offset = 0.0625 - b_size * 0.0625;
        size += 0.5 * (b_size - 0.7).abs() * 2.0;
        Some(Self { outward, z_offset, rotation: size.min(1.0) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with_size(size: f32) -> Config {
        Config { size, ..Config::default() }
    }

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-5
    }

    #[test]
    fn defaults_match_the_original() {
        let shape = Shape::from_config(&Config::default()).unwrap();
        assert!(close(shape.outward, 0.0));
        assert!(close(shape.z_offset, 0.025));
        assert!(close(shape.rotation, 0.8));
    }

    #[test]
    fn tiny_sizes_draw_nothing() {
        assert!(Shape::from_config(&with_size(0.0)).is_none());
        assert!(Shape::from_config(&with_size(0.01)).is_none());
    }

    #[test]
    fn large_sizes_are_clamped() {
        let shape = Shape::from_config(&with_size(1.0)).unwrap();
        assert!(close(shape.rotation, 1.0));
        let wide = Shape::from_config(&Config { cleavage: 0.5, ..Config::default() }).unwrap();
        assert!(close(wide.outward, MAX_OUTWARD));
    }
}
