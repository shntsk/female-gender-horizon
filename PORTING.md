# Porting status

## Done (Phase 1, nothing compiled or tested yet)

- **Fork API** (`fork-patch/`, applied to the `shntsk/horizon` fork):
  - `uuid: Option<u128>` on `EntityRenderState` (already in the fork).
  - The `horizon:skin` sprite: a layer draws with the player's own skin (`skin-in-layers-v2.patch`, redone
    because the first version never reached the fork).
- **Mod** (`src/`): a layer on both player models, 4x5x3 boxes with the original UV layout, the jacket
  overlay, breathing, and the transforms of `GenderLayer.setupTransformations` without physics.
- **Repository**: `[native]` manifest with pinned commits, LGPL-3.0 license, credits, and a build Action.

## Known risks of Phase 1

- `PartPose` fields and the use of `body.pose` may differ from what the compiler expects.
- UV orientation of `cube` (`FaceBakery` baking) may come out mirrored compared to the original.
- The layer applies to every player: there is no gender selection yet.
- Producing the `.hmod` needs a signed SDK release for the pinned `horizon` commit; without one, only
  `cargo check` runs in CI.

## Remaining work (suggested order)

1. **Phase 1 for real**: compile, run in game, fix UVs and transforms.
2. **Phase 2, physics** (`BreastPhysics.java`): per-player state kept in the mod, keyed by `entity.uuid`.
   Per-tick reads through the `Host`'s `entity_position`, `entity_velocity` and `entity_pose` in
   `client_tick_post` (to be confirmed), plus `body_rot`, `walk_animation_pos/speed` and `pose` from
   `EntityRenderState`. Pose mapping and arm swing are still to be worked out.
3. **Phase 3, armor**: `assets/minecraft/female_gender_mod/armor_data` and the per-armor chest and
   physics logic.
4. **Configuration**: size, offsets, cleavage, uniboob, floppiness and jacket, per player.
5. Out of scope for now: config screen, cloud sync, mannequins and networking.
6. After this mod: needsofnature (1.21.11 only, last) and the other 26.2/26.3 mods.
