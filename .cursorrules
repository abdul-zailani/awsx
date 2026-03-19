# awsx — AI Agent Guidelines

awsx does ONE thing: **switch AWS profile + kubectl context + region in one command.** Do not expand this scope.

## Rules

1. **No features outside context switching.** No AWS CLI wrapping, no kubectl management, no deploy/infra tools.
2. **Before adding a feature, ask:** Is this about switching context? Can existing tools do this? Without this, does the user need >1 command to switch? If any answer disqualifies it, reject.
3. **No unnecessary dependencies.** Binary is ~2MB. Keep it small.
4. **No preemptive abstractions.** No traits, generics, or plugin systems "for future use."
5. **Config path is always `~/.config/awsx/config.toml`.** No platform-specific directories.
6. **Shell hook must embed absolute binary path** via `std::env::current_exe()`. Never rely on PATH.
7. **Matching logic must be generic.** No hardcoded company names or prefixes. Match by account ID first, then token-based name scoring.
8. **Test every change.** `cargo build --release`, then verify with `awsx init` + `awsx list`.
9. **Handle errors gracefully.** No `unwrap()` on user input. Exit code 1 on errors.

## Scope

**IS:** context switcher, saved-context manager, shell environment setter.
**IS NOT:** AWS CLI wrapper, k8s manager, infra tool, credential manager, monitoring tool.

See CONTRIBUTING.md for full details and decision examples.
