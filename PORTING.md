# Porting status

## Done (Phase 1, compiles against the pinned API, not yet run in game)

- **Fork API** (`fork-patch/`, applied to the `shntsk/horizon` fork):
  - `uuid: Option<u128>` on `EntityRenderState` (already in the fork).
  - The `horizon:skin` sprite: a layer draws with the player's own skin (`skin-in-layers-v2.patch`, redone
    because the first version never reached the fork).
- **Mod** (`src/`): a layer on both player models, 4x5x3 boxes with the original UV layout, the jacket
  overlay, breathing, and the transforms of `GenderLayer.setupTransformations` without physics. Split
  into `config`, `shape`, `uv` and `layer`; the derived dimensions and UV layouts have unit tests.
- **Repository**: `[native]` manifest with pinned commits, LGPL-3.0 license, credits, and a build Action.
- **Packaging fix**: `cargo horizon-mod build` refused the manifest with `a content mod has
  capabilities = ["registries"]`. The pinned manifest validator (`Manifest::parse`) requires exactly
  `capabilities = ["registries"]` for every mod, so the manifest now declares it.

## Known risks of Phase 1

- `PartPose` fields and the use of `body.pose` compile, but the transform order is unverified in game.
- UV orientation of `cube` (`FaceBakery` baking) may come out mirrored compared to the original.
- The layer applies to every player: there is no gender selection yet.
- Producing the `.hmod` needs a signed SDK release for the pinned `horizon` commit; without one, only
  `cargo check` runs in CI. The workflow warns about it (and fails on `v*` tags).
- The unit tests are compiled by `cargo check --all-targets` but CI does not run them.
- `Cargo.lock` is not committed, so dependency versions float; commit one once the build is stable.

## Remaining work (suggested order)

1. **Phase 1 for real**: run in game, fix UVs and transforms.
2. **Phase 2, physics** (`BreastPhysics.java`): per-player state kept in the mod, keyed by `entity.uuid`.
   Per-tick reads through the `Host`'s `entity_position`, `entity_velocity` and `entity_pose` in
   `client_tick_post` (to be confirmed), plus `body_rot`, `walk_animation_pos/speed` and `pose` from
   `EntityRenderState`. Pose mapping and arm swing are still to be worked out. `Config::uniboob` and the
   bounce rotation in `layer.rs` are waiting for this.
3. **Phase 3, armor**: `assets/minecraft/female_gender_mod/armor_data` and the per-armor chest and
   physics logic.
4. **Configuration**: size, offsets, cleavage, uniboob, floppiness and jacket, per player.
5. Out of scope for now: config screen, cloud sync, mannequins and networking.
6. After this mod: needsofnature (1.21.11 only, last) and the other 26.2/26.3 mods.
