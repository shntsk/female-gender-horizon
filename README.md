# Female Gender Mod for Horizon

A client-side port of [Wildfire's Female Gender Mod](https://github.com/FemaleGenderMod/FemaleGenderMod)
to the [Horizon](https://github.com/shntsk/horizon) mod API. It draws a configurable breast layer over
players, textured from each player's own skin.

> **Status: Phase 1 (static layer), untested.** Nothing here has been compiled or run in-game yet.
> See [PORTING.md](PORTING.md) for what is done and what is left.

## What it does today

- Adds one entity layer to both player models (wide and slim).
- Draws two 4x5x3 boxes with the original mod's default UV layout, plus the jacket overlay.
- Uses the original defaults: size 0.6, no cleavage, no offsets, unified movement, breathing.

Not yet implemented: breast physics, armor, per-player settings, a config screen, and cloud sync.

## Requirements

- A Horizon build that includes the `horizon:skin` layer sprite and `EntityRenderState.uuid`
  (the fork patches are listed in [PORTING.md](PORTING.md)). The exact commit is pinned in
  `horizon-mod.toml` under `[native]`.
- Minecraft 26.3 (`[native].game`).

## Building

The mod is a native Horizon package: `horizon-mod.toml`, `Cargo.toml` and `src/lib.rs`.

```sh
cargo install --locked --git https://github.com/shntsk/horizon --rev <pinned commit> cargo-horizon-mod
cargo horizon-mod build        # target/horizon-mod/female_gender.hmod
```

`cargo horizon-mod build` fetches the SDK of the pinned commit, which must exist as a signed SDK release.
The GitHub Action in `.github/workflows/build.yml` runs `cargo check` against the fork's API on every push,
and uploads the source package and, when the SDK exists, the `.hmod` as workflow artifacts.

## Credits and license

This project is a port of **Female Gender Mod** by WildfireRomeo and contributors. The transforms, UV
layouts and defaults come from its `GenderLayer`, `WildfireModelRenderer` and `UVs` classes. It is
licensed under the **GNU Lesser General Public License v3.0 or later**, like the original; see
[LICENSE](LICENSE) and [NOTICE.md](NOTICE.md). It is not affiliated with or endorsed by the original authors.
