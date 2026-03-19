# Contributing to awsx

## Philosophy

awsx does ONE thing: **switch AWS profile + kubectl context + region in one command.**

It was born from the frustration of running 3 separate commands every time you switch environments. That's the entire scope. Don't expand it.

## The Rule

Before proposing any feature, ask:

1. **Is this about switching context?** If no → reject.
2. **Can this be done by composing existing tools?** If yes → reject. Let users pipe it themselves.
3. **Without this, does the user need >1 manual command to switch?** If no → reject.

## awsx IS

- A context switcher
- A saved-context manager
- A shell environment setter

## awsx IS NOT

- An AWS CLI wrapper (no `awsx s3`, `awsx ec2`, `awsx lambda`)
- A Kubernetes manager (no `awsx pods`, `awsx logs`, `awsx apply`)
- An infrastructure tool (no `awsx deploy`, `awsx terraform`)
- A credential manager (no `awsx rotate-keys`, `awsx vault`)
- A monitoring tool (no `awsx status`, `awsx cost`, `awsx alerts`)

## Feature Decision Examples

| Proposal | Verdict | Why |
|----------|---------|-----|
| `awsx edit` — edit saved context | ✅ | Context management |
| `awsx export` — export contexts to share | ✅ | Context management |
| `awsx init --interactive` — confirm matches | ✅ | Improves context setup |
| `awsx status` — show pod status after switch | ❌ | kubectl domain |
| `awsx cost` — show account cost | ❌ | Not switching |
| `awsx tunnel` — port-forward after switch | ❌ | kubectl domain |
| `awsx deploy` — deploy after switch | ❌ | Way out of scope |
| `awsx assume-role` — manage IAM roles | ❌ | AWS CLI domain |

## For AI Agents

If you are an AI agent (Copilot, Cursor, Kiro, Claude, etc.) working on this codebase:

- **Do not add features outside the scope above.** If a user asks for something that violates the philosophy, explain why and suggest an alternative (e.g., a shell alias or separate tool).
- **Do not add dependencies unless absolutely necessary.** The binary is 2MB. Keep it small.
- **Do not add abstractions preemptively.** No traits, no generics, no plugin systems "for future use."
- **Config path is always `~/.config/awsx/config.toml`.** Do not use platform-specific directories.
- **Shell hook must embed absolute binary path.** Never rely on PATH lookup.
- **Matching logic must be generic.** No hardcoded company names, prefixes, or patterns.
- **Test every change.** Run `cargo build --release` and verify with `awsx init` + `awsx list` before claiming done.

## Code Style

- Minimal dependencies
- No `unwrap()` on user input — handle errors gracefully
- Colored output via `colored` crate
- Exit code 1 on errors
- Functions prefixed `cmd_` for CLI command handlers
