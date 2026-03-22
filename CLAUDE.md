# DeltaRust — Claude Instructions

## Version Bumping

After completing any feature, bug fix, or meaningful change in this project, always run the `/version-bump` skill automatically without waiting to be asked. Do not skip this step.

Use this priority order to decide the bump:

| What changed | Bump |
|---|---|
| Breaks save compatibility, removes major systems, overhauls core game loop | MAJOR |
| New player-visible feature, new game system, new console command, new content | MINOR |
| Bug fix, text change, balance tweak, internal refactor | PATCH |

Edit only the `version = "X.Y.Z"` line in `Cargo.toml`. Do not commit. Do not tag.
