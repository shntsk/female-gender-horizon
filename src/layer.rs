//! The entity layer: `GenderLayer.setupTransformations` and `WildfireModelRenderer` boxes of the
//! original, without physics (Phase 2).
use crate::config::Config;
use crate::shape::Shape;
use crate::uv::{LEFT, LEFT_WEAR, Layout, RIGHT, RIGHT_WEAR};
use mod_api::{BlockEntityDrawContext, EntityRenderLayer, EntityRenderState, HumanoidPart, PosedParts};

/// One skin pixel in blocks.
const PIXEL: f32 = 0.0625;
/// Height of the model's origin above the entity's feet, as `render_living_pre` places it.
const MODEL_ORIGIN_Y: f32 = -1.501;
/// Where the breasts sit on the chest, in blocks.
const CHEST_Y: f32 = 0.05625;
const CHEST_Z: f32 = -0.125;
/// Depth offset per unit of the config's z offset.
const OFFSET_Z_STEP: f32 = 0.0425;
/// Resting pitch of a breast, in degrees at rotation 1.
const PITCH: f32 = -35.0;
/// Idle breathing: speed of the sway and its amplitude.
const BREATH_SPEED: f32 = 0.09;
const BREATH_AMPLITUDE: f32 = 0.45;
/// The jacket overlay: pushed out of the skin box and inflated by 5 percent.
const JACKET_Z: f32 = -0.015;
const JACKET_SCALE: f32 = 1.05;
/// Narrows the box slightly in x so the two halves do not z-fight at the cleavage.
const SEAM_SCALE: f32 = 0.9995;

/// Which breast is being drawn. Left is on the player's left, drawn at positive x.
#[derive(Clone, Copy)]
enum Side {
    Left,
    Right,
}

impl Side {
    const BOTH: [Side; 2] = [Side::Left, Side::Right];

    /// `1.0` for left, `-1.0` for right: mirrors offsets and the outward rotation.
    fn sign(self) -> f32 {
        match self {
            Side::Left => 1.0,
            Side::Right => -1.0,
        }
    }

    /// Corners of the 4x5x3 box, in skin pixels, each half starting at the cleavage.
    fn bounds(self) -> ([f32; 3], [f32; 3]) {
        match self {
            Side::Left => ([-4.0, 0.0, 0.0], [0.0, 5.0, 3.0]),
            Side::Right => ([0.0, 0.0, 0.0], [4.0, 5.0, 3.0]),
        }
    }

    fn skin(self) -> Layout {
        match self {
            Side::Left => LEFT,
            Side::Right => RIGHT,
        }
    }

    fn jacket(self) -> Layout {
        match self {
            Side::Left => LEFT_WEAR,
            Side::Right => RIGHT_WEAR,
        }
    }
}

pub struct GenderLayer {
    config: Config,
}

impl GenderLayer {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// One breast, in the body's frame.
    fn draw_breast(
        &self,
        draw: &mut dyn BlockEntityDrawContext,
        entity: &EntityRenderState,
        shape: &Shape,
        side: Side,
    ) {
        let c = &self.config;
        let s = side.sign();
        // Offsets as the original reads them: x as is, y and z inverted.
        let (ox, oy, oz) = (c.offset[0], -c.offset[1], -c.offset[2]);

        draw.push();
        draw.translate([
            s * ox * PIXEL,
            CHEST_Y + oy * PIXEL,
            shape.z_offset + CHEST_Z + oz * OFFSET_Z_STEP,
        ]);
        // With physics (Phase 2) the bounce rotation goes here, between the two shifts, and
        // `uniboob` decides whether both sides share it.
        draw.rotate_about([0.0, 1.0, 0.0], s * shape.outward, [0.0; 3]);
        draw.rotate_about([1.0, 0.0, 0.0], PITCH * shape.rotation, [0.0; 3]);
        if c.breathing {
            let sway = -(entity.age * BREATH_SPEED).cos() * BREATH_AMPLITUDE + BREATH_AMPLITUDE;
            draw.rotate_about([1.0, 0.0, 0.0], sway, [0.0; 3]);
        }
        draw.scale([SEAM_SCALE, 1.0, 1.0]);
        Self::draw_box(draw, side, side.skin());
        if c.jacket {
            draw.translate([0.0, 0.0, JACKET_Z]);
            draw.scale([JACKET_SCALE; 3]);
            Self::draw_box(draw, side, side.jacket());
        }
        draw.pop();
    }

    fn draw_box(draw: &mut dyn BlockEntityDrawContext, side: Side, layout: Layout) {
        let (from, to) = side.bounds();
        draw.cube(from, to, layout.faces(), 0xFFFFFF);
    }
}

/// `model.body.translateAndRotate`: move, then rotate about z, y, x.
fn enter_body_frame(draw: &mut dyn BlockEntityDrawContext, position: [f32; 3], rotation: [f32; 3]) {
    // The model's own frame, as `render_living_pre` draws (+y down, front to -z).
    draw.scale([1.0, -1.0, -1.0]);
    draw.translate([0.0, MODEL_ORIGIN_Y, 0.0]);
    let [x, y, z] = position;
    draw.translate([x / 16.0, y / 16.0, z / 16.0]);
    let [x_rot, y_rot, z_rot] = rotation;
    draw.rotate_about([0.0, 0.0, 1.0], z_rot.to_degrees(), [0.0; 3]);
    draw.rotate_about([0.0, 1.0, 0.0], y_rot.to_degrees(), [0.0; 3]);
    draw.rotate_about([1.0, 0.0, 0.0], x_rot.to_degrees(), [0.0; 3]);
}

impl EntityRenderLayer for GenderLayer {
    fn submit(
        &self,
        entity: &EntityRenderState,
        parts: Option<&PosedParts>,
        draw: &mut dyn BlockEntityDrawContext,
    ) -> Result<(), String> {
        let Some(body) = parts.and_then(|p| p.get(HumanoidPart::Body)) else { return Ok(()) };
        let Some(shape) = Shape::from_config(&self.config) else { return Ok(()) };

        draw.push();
        let p = body.pose;
        enter_body_frame(draw, [p.x, p.y, p.z], [p.x_rot, p.y_rot, p.z_rot]);
        for side in Side::BOTH {
            self.draw_breast(draw, entity, &shape, side);
        }
        draw.pop();
        Ok(())
    }
}
