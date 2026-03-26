# awsx

[![Crates.io](https://img.shields.io/crates/v/aws-context-switcher.svg?style=flat-square)](https://crates.io/crates/aws-context-switcher)
[![GitHub release](https://img.shields.io/github/v/release/abdul-zailani/awsx?style=flat-square)](https://github.com/abdul-zailani/awsx/releases)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![Built with Rust](https://img.shields.io/badge/built%20with-Rust-orange?style=flat-square)](https://www.rust-lang.org/)

**Stop running 3 commands every time you switch AWS environments.**

`awsx` switches your AWS profile + kubectl context + region in one command. Zero config — it auto-discovers your existing setup.

```bash
$ awsx use prd
✓ AWS profile: my-prd-profile (account: 123456789012)
  Role: AdminRole
✓ Kubernetes: prd-cluster
```

Built for DevOps/SRE engineers managing multiple AWS accounts and EKS clusters.

## Features

- 🔀 **Unified switching** — AWS profile + kubectl context + region in one command
- 🧠 **Auto-discovery** — `awsx init` scans your AWS profiles and kubeconfig, matches them automatically
- 💾 **Saved contexts** — define environment combos, switch instantly
- 🔍 **Fuzzy picker** — interactive selection powered by skim (Rust fzf)
- 🔐 **Auto SSO login** — detects expired sessions, triggers `aws sso login`
- 📸 **Auto-detect** — captures current AWS profile, region, and kubectl context on save
- 🐚 **Shell integration** — eval-based env export for zsh/bash/fish
- 🎨 **Color-coded** — environments tagged as PRD/STG/DEV with colors
- ⚡ **Fast** — native Rust binary, ~6ms startup

## Install

### One-liner (recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/abdul-zailani/awsx/main/install.sh | sh
```

This will:
- Detect your OS and architecture (macOS/Linux, x86_64/arm64)
- Download the latest release binary (or build from source if no release available)
- Auto-add shell hook to your `.zshrc`, `.bashrc`, or `config.fish`
- Run `awsx init` to auto-discover your environments

After install, reload your shell:

```bash
source ~/.zshrc  # or ~/.bashrc
awsx list        # see discovered contexts
awsx use         # start switching
```

### With Cargo (crates.io)

```bash
cargo install aws-context-switcher
eval "$(awsx shell-hook zsh --prompt)"  # add to your rc file
awsx init
```

### From source (GitHub)

```bash
cargo install --git https://github.com/abdul-zailani/awsx
eval "$(awsx shell-hook zsh --prompt)"  # add to your rc file
awsx init
```

## Quick Start

### Option 1: Auto-discover (recommended)

```bash
awsx init
```

Scans your existing AWS profiles and kubectl contexts, then intelligently matches them:

```
Scanning AWS profiles and kubectl contexts...

  5 AWS profiles found:
    default, my-app-dev, my-app-stg, my-app-prd, data-platform
  3 kubectl contexts found:
    app-dev, app-stg, app-prd

  ✓ default → aws=default | region=us-east-1
  ✓ my-app-dev → aws=my-app-dev | k8s=app-dev
  ✓ my-app-stg → aws=my-app-stg | k8s=app-stg
  ✓ my-app-prd → aws=my-app-prd | k8s=app-prd
  ✓ data-platform → aws=data-platform

✓ 5 contexts saved.
```

**How matching works:**

1. **Account ID matching** — reads `sso_account_id` or `role_arn` from your AWS config, matches to EKS cluster ARNs in kubeconfig
2. **Token-based name scoring** — tokenizes names (splits by `-`, `_`, `.`), scores by overlap percentage
3. **Unmatched entries** — AWS-only profiles and kubectl-only contexts are saved as standalone entries

Works with any setup: AWS SSO, IAM assume-role, IAM access keys, EKS, GKE, self-hosted clusters, or kubectl-only environments.

### Option 2: Save from current state

Switch to your environment manually once, then let `awsx` capture it:

```bash
export AWS_PROFILE=my-stg-profile
kubectl config use-context my-stg-cluster

# Save — auto-detects current profile, region, and kubectl context
awsx save stg --environment staging
```

### Option 3: Save with explicit flags

```bash
awsx save prd \
  --aws-profile my-prd-profile \
  --region ap-southeast-1 \
  --kube-context my-prd-cluster \
  --namespace default \
  --environment production
```

### Switch

```bash
awsx use          # interactive fuzzy picker
awsx use prd      # direct switch
awsx current      # show current status
awsx list         # list saved contexts
awsx clear        # unset all AWS env vars
```

## Commands

| Command | Description |
|---------|-------------|
| `awsx init` | Auto-discover AWS profiles and kubectl contexts |
| `awsx use [name]` | Switch to saved context (interactive if no name) |
| `awsx profile [name]` | Switch AWS profile only |
| `awsx kube [name]` | Switch kubectl context only |
| `awsx save <name>` | Save a context (auto-detects current state, or use flags) |
| `awsx delete <name>` | Delete a saved context |
| `awsx list` | List all saved contexts |
| `awsx current` | Show current active context |
| `awsx shell-hook <shell>` | Output shell hook (zsh/bash/fish) |
| `awsx clear` | Unset all AWS environment variables |

## Config

Contexts are stored per engineer in `~/.config/awsx/config.toml`:

```toml
[contexts.prd]
aws_profile = "my-prd-profile"
region = "ap-southeast-1"
kube_context = "my-prd-cluster"
namespace = "default"
environment = "production"
```

This file is local — context names and mappings can differ between machines. Run `awsx init` on each machine to auto-generate.

## Uninstall

```bash
curl -fsSL https://raw.githubusercontent.com/abdul-zailani/awsx/main/uninstall.sh | sh
```

## Requirements

- AWS CLI v2
- kubectl (optional, for Kubernetes context switching)

## Why awsx?

| Tool | AWS profile | kubectl context | Region | Auto-discover | One binary |
|------|:-----------:|:---------------:|:------:|:-------------:|:----------:|
| **awsx** | ✅ | ✅ | ✅ | ✅ | ✅ |
| awsp | ✅ | ❌ | ❌ | ❌ | ❌ (shell) |
| aws-vault | ✅ | ❌ | ❌ | ❌ | ✅ |
| kubectx | ❌ | ✅ | ❌ | ❌ | ✅ |
| awsume | ✅ | ❌ | ✅ | ❌ | ❌ (python) |

## License

MIT
