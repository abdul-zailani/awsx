# Contributing to awsx

First off, thank you for considering contributing to `awsx`! It's people like you that make `awsx` such a great tool.

## Philosophy

`awsx` does exactly ONE thing: **switch AWS profile + kubectl context + region in one single command.**

Before submitting a Pull Request, please ensure your feature aligns with our core [AGENTS.md](AGENTS.md) philosophy:
- We don't wrap AWS CLI commands.
- We keep the binary footprint minimal.
- We rely on native `~/.aws/config` and `~/.kube/config` semantics.

## Development Setup

1. Clone the repository.
2. Ensure you have Rust installed (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`).
3. Build the project: `cargo build`
4. Run tests (if applicable) and manually verify the binary using `cargo run -- <command>`.

## Submitting a Pull Request

1. **Create a branch** for your feature or bug fix: `git checkout -b feat/my-new-feature` or `bug/fix-issue-1`.
2. **Commit your changes**. Keep commit messages clear (e.g., `feat: added auto-discovery for GKE`).
3. **Open a Pull Request** against the `main` branch. Provide a clear description of the problem and your solution.

## Release Process (Maintainers)

Releases are managed using the local `.agents/workflows/publish-release.md` process.
