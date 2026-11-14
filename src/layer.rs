//! `GenderLayer.setupTransformations` and `WildfireModelRenderer` boxes, without physics (Phase 2).
use crate::config::Config;
use mod_api::{BlockEntityDrawContext, CubeFace, EntityRenderState, EntityRenderLayer, HumanoidPart, PosedParts};

const SKIN: &str = "horizon:skin";
/// A 64x64 skin's pixels in texels of 16 per side.
const PX: f32 = 0.25;

/// `[x1, y1, x2, y2]` in skin pixels; all zero is unused, as the original's `UVQuad.UNUSED`.
type Quad = [i32; 4];
/// East, west, down, up, north: the original's `UVLayout` order.
struct Layout([Quad; 5]);

const LEFT: Layout = Layout([[24, 21, 27, 26], [16, 21, 20, 26], [20, 17, 24, 21], [20, 25, 24, 27], [20, 21, 24, 26]]);
const RIGHT: Layout = Layout([[28, 21, 32, 26], [21, 21, 24, 26], [24, 17, 28, 21], [24, 25, 28, 27], [24, 21, 28, 26]]);
const LEFT_WEAR: Layout = Layout([[0; 4], [17, 37, 20, 42], [20, 34, 24, 37], [20, 41, 24, 44], [20, 37, 24, 42]]);
const RIGHT_WEAR: Layout = Layout([[28, 37, 31, 42], [0; 4], [24, 34, 28, 37], [24, 41, 28, 44], [24, 37, 28, 42]]);

impl Layout {
    fn face(&self, i: usize) -> Option<CubeFace<'static>> {
        let [a, b, c, d] = self.0[i];
        (a | b | c | d != 0).then(|| CubeFace::new(SKIN).with_uv([a, b, c, d].map(|p| p as f32 * PX)))
    }
    /// `Direction` order: down, up, north, south, west, east.
    fn faces(&self) -> [Option<CubeFace<'static>>; 6] {
        [self.face(2), self.face(3), self.face(4), None, self.face(1), self.face(0)]
    }
}

pub struct GenderLayer {
    config: Config,
}

impl GenderLayer {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl EntityRenderLayer for GenderLayer {
    fn submit(
        &self,
        entity: &EntityRenderState,
        parts: Option<&PosedParts>,
        draw: &mut dyn BlockEntityDrawContext,
    ) -> Result<(), String> {
        let Some(body) = parts.and_then(|p| p.get(HumanoidPart::Body)) else { return Ok(()) };
        let c = &self.config;
        // setupRender: sizes, clamped as the original does.
        let b_size = c.size;
        let outward = (c.cleavage * 100.0).round().min(10.0);
        let mut size = (b_size * 1.5).min(0.7);
        if b_size > 0.7 {
            size = b_size;
        }
        if size < 0.02 {
            return Ok(());
        }
        let z_offset = 0.0625 - b_size * 0.0625;
        size += 0.5 * (b_size - 0.7).abs() * 2.0;
        let (ox, oy, oz) = (c.offset[0], -c.offset[1], -c.offset[2]);

        draw.push();
        // The model's own frame, as `render_living_pre` draws (+y down, front to -z).
        draw.scale([1.0, -1.0, -1.0]);
        draw.translate([0.0, -1.501, 0.0]);
        // model.body.translateAndRotate: move, then z, y, x.
        let p = body.pose;
        draw.translate([p.x / 16.0, p.y / 16.0, p.z / 16.0]);
        draw.rotate_about([0.0, 0.0, 1.0], p.z_rot.to_degrees(), [0.0; 3]);
        draw.rotate_about([0.0, 1.0, 0.0], p.y_rot.to_degrees(), [0.0; 3]);
        draw.rotate_about([1.0, 0.0, 0.0], p.x_rot.to_degrees(), [0.0; 3]);

        for left in [true, false] {
            let s = if left { 1.0 } else { -1.0 };
            draw.push();
            draw.translate([s * ox * 0.0625, 0.05625 + oy * 0.0625, z_offset - 0.125 + oz * 0.0425]);
            if !c.uniboob {
                // Only matters with physics' bounce rotation between the two shifts (Phase 2).
            }
            let rotation = size.min(size + 0.2).min(1.0);
            draw.rotate_about([0.0, 1.0, 0.0], s * outward, [0.0; 3]);
            draw.rotate_about([1.0, 0.0, 0.0], -35.0 * rotation, [0.0; 3]);
            if c.breathing {
                let f5 = -(entity.age * 0.09).cos() * 0.45 + 0.45;
                draw.rotate_about([1.0, 0.0, 0.0], f5, [0.0; 3]);
            }
            draw.scale([0.9995, 1.0, 1.0]);
            let (from, to) = if left { ([-4.0, 0.0, 0.0], [0.0, 5.0, 3.0]) } else { ([0.0, 0.0, 0.0], [4.0, 5.0, 3.0]) };
            draw.cube(from, to, if left { LEFT } else { RIGHT }.faces(), 0xFFFFFF);
            if c.jacket {
                draw.translate([0.0, 0.0, -0.015]);
                draw.scale([1.05, 1.05, 1.05]);
                draw.cube(from, to, if left { LEFT_WEAR } else { RIGHT_WEAR }.faces(), 0xFFFFFF);
            }
            draw.pop();
        }
        draw.pop();
        Ok(())
    }
}
