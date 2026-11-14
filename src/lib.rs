//! Port of Wildfire's Female Gender Mod (LGPL-3.0-or-later, see NOTICE.md). Phase 1: a static layer
//! with the mod's default settings, drawn from the player's own skin (`horizon:skin`).
use mod_api::{self, Hooks, LayerTarget, PlayerModel, RegisterEntityLayersEvent, RegistrationContext};
use std::sync::Arc;

mod config;
mod layer;

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl Hooks for Mod {
    fn register_entity_layers(
        &self,
        _context: &RegistrationContext,
        event: &mut RegisterEntityLayersEvent<'_>,
    ) {
        let layer = Arc::new(layer::GenderLayer::new(config::Config::default()));
        for model in [PlayerModel::Wide, PlayerModel::Slim] {
            let _ = event.layers.add(LayerTarget::Player(model), layer.clone());
        }
    }
}
