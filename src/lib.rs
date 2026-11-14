//! Port of Wildfire's Female Gender Mod (LGPL-3.0-or-later, see NOTICE.md).
//!
//! Phase 1: a static layer with the mod's default settings, drawn from the player's own skin
//! (`horizon:skin`). Layout of the crate:
//!
//! - [`config`]: the mod's settings (defaults of the original).
//! - [`shape`]: breast dimensions derived from the settings (`GenderLayer.setupRender`).
//! - [`uv`]: the skin UV layouts of the boxes.
//! - [`layer`]: the entity layer that draws them.
use mod_api::{self, Hooks, LayerTarget, PlayerModel, RegisterEntityLayersEvent, RegistrationContext};
use std::sync::Arc;

mod config;
mod layer;
mod shape;
mod uv;

/// Both player models the layer is attached to.
const PLAYER_MODELS: [PlayerModel; 2] = [PlayerModel::Wide, PlayerModel::Slim];

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl Hooks for Mod {
    fn register_entity_layers(
        &self,
        _context: &RegistrationContext,
        event: &mut RegisterEntityLayersEvent<'_>,
    ) {
        // One layer instance, shared by both models: it holds no per-player state yet (Phase 2).
        let layer = Arc::new(layer::GenderLayer::new(config::Config::default()));
        for model in PLAYER_MODELS {
            // A refused registration only leaves that model without the layer; the host reports it.
            let _ = event.layers.add(LayerTarget::Player(model), layer.clone());
        }
    }
}
