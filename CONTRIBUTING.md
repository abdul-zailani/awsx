# Contributing to awsx

## Philosophy

awsx does ONE thing: **switch AWS profile + kubectl context + region in one command.**

It was born from the frustration of running 3 separate commands every time you switch environments. That's the entire scope. Don't expand it.

## The Rule

Before proposing any feature, ask:

1. **Is this about switching context?** If no → reject.
2. **Can this be done by composing existing tools?** If yes → reject. Let users pipe it themselves.
3. **Without this, does the user need >1 manual command to switch?** If no → reject.

## Scope

**IS:** context switcher, saved-context manager, shell environment setter.

**IS NOT:** AWS CLI wrapper, Kubernetes manager, infrastructure tool, credential manager, monitoring tool.

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

## Code Style

- Minimal dependencies — binary is ~2MB, keep it small
- No `unwrap()` on user input — handle errors gracefully, exit code 1
- Colored output via `colored` crate
- Functions prefixed `cmd_` for CLI command handlers
- Config path is always `~/.config/awsx/config.toml`
- Shell hook must embed absolute binary path via `std::env::current_exe()`
- Matching logic must be generic — no hardcoded company names or prefixes
- Test every change: `cargo build --release`, verify with `awsx init` + `awsx list`
