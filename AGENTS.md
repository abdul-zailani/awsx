# awsx — Workspace Guidelines

This document defines the core architecture and boundaries for the `awsx` project. Both human contributors and AI assistants must strictly adhere to these rules.

## Core Philosophy
`awsx` does exactly ONE thing: **switch AWS profile + kubectl context + region in one single command.**
Do not expand this scope. It is not an AWS CLI wrapper, nor a Kubernetes manager.

## Technical Constraints
1. **Scope strictness**: No features outside context switching. If a feature can be achieved by piping existing tools, it should be rejected.
2. **Minimal footprint**: Keep dependencies to an absolute minimum. The binary must remain small and fast.
3. **No preemptive abstractions**: Avoid unnecessary traits, generics, or plugin systems designed "for future use".
4. **Configuration location**: Config path is strictly `~/.config/awsx/config.toml`. Do not use dynamic OS-specific config directories.
5. **Shell integration**: Shell hooks must always evaluate absolute binary paths via `std::env::current_exe()`. Never rely on the user's `$PATH`.
6. **Matching logic**: Context auto-discovery must be generic. Match by Account ID first, then token-based string scoring.
7. **Error handling**: Handle errors gracefully. Exit code 1 on known failure states. Avoid `unwrap()` on runtime environments.
8. **Testing standard**: Verify changes locally via `cargo build` and run standard flows (`awsx init`, `awsx list`) before committing.

*Please refer to `CONTRIBUTING.md` for feature decision examples.*
