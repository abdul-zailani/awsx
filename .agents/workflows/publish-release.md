---
description: Standardized workflow for bumping versions, tagging, and publishing awsx to crates.io
---

1. Update `version` in `Cargo.toml` using Semantic Versioning.
2. Commit the change: `git add Cargo.toml && git commit -m "chore: bump version to X.Y.Z"`
3. Create a Git tag with the 'v' prefix: `git tag vX.Y.Z`
4. Push the commit and the tag: `git push origin main --tags`
5. Publish to Cargo registry: `cargo publish`
