//! Skin UV layouts of the breast boxes: the original's `UVs` and `UVLayout`.
use mod_api::CubeFace;

/// The layer draws with the player's own skin.
const SKIN: &str = "horizon:skin";
/// One skin pixel in texels of 16 per side (a 64x64 skin).
const PX: f32 = 0.25;

/// `[x1, y1, x2, y2]` in skin pixels; all zero is unused, as the original's `UVQuad.UNUSED`.
type Quad = [i32; 4];

/// East, west, down, up, north: the original's `UVLayout` order.
pub struct Layout([Quad; 5]);

pub const LEFT: Layout =
    Layout([[24, 21, 27, 26], [16, 21, 20, 26], [20, 17, 24, 21], [20, 25, 24, 27], [20, 21, 24, 26]]);
pub const RIGHT: Layout =
    Layout([[28, 21, 32, 26], [21, 21, 24, 26], [24, 17, 28, 21], [24, 25, 28, 27], [24, 21, 28, 26]]);
pub const LEFT_WEAR: Layout =
    Layout([[0; 4], [17, 37, 20, 42], [20, 34, 24, 37], [20, 41, 24, 44], [20, 37, 24, 42]]);
pub const RIGHT_WEAR: Layout =
    Layout([[28, 37, 31, 42], [0; 4], [24, 34, 28, 37], [24, 41, 28, 44], [24, 37, 28, 42]]);

impl Layout {
    fn face(&self, i: usize) -> Option<CubeFace<'static>> {
        let [a, b, c, d] = self.0[i];
        (a | b | c | d != 0).then(|| CubeFace::new(SKIN).with_uv([a, b, c, d].map(|p| p as f32 * PX)))
    }

    /// The faces in `Direction` order: down, up, north, south, west, east. The south face is
    /// against the body and is never drawn.
    pub fn faces(&self) -> [Option<CubeFace<'static>>; 6] {
        [self.face(2), self.face(3), self.face(4), None, self.face(1), self.face(0)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unused_quads_have_no_face() {
        // The jacket has no east face on the left and no west face on the right.
        assert!(LEFT_WEAR.face(0).is_none());
        assert!(RIGHT_WEAR.face(1).is_none());
        assert!(LEFT.face(0).is_some());
    }

    #[test]
    fn the_south_face_is_never_drawn() {
        for layout in [LEFT, RIGHT, LEFT_WEAR, RIGHT_WEAR] {
            assert!(layout.faces()[3].is_none());
        }
    }
}
