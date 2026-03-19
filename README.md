# awsx

**AWS Context Switcher** — switch AWS profile + kubectl context + region in one command.

Built for DevOps/SRE engineers managing multiple AWS accounts and EKS clusters.

## Features

- 🔀 **Unified switching** — AWS profile + kubectl context + region in one command
- 💾 **Saved contexts** — define environment combos, switch instantly
- 🔍 **Fuzzy picker** — interactive selection powered by skim (Rust fzf)
- 🔐 **Auto SSO login** — detects expired sessions, triggers `aws sso login`
- 📸 **Auto-detect** — captures current AWS profile, region, and kubectl context automatically
- 🐚 **Shell integration** — eval-based env export for zsh/bash/fish
- 🎨 **Color-coded** — environments tagged as PRD/STG/DEV with colors
- ⚡ **Fast** — native Rust binary, sub-millisecond startup

## Install

### One-liner (recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/abdul-zailani/awsx/main/install.sh | sh
```

This will:
- Detect your OS and architecture (macOS/Linux, x86_64/arm64)
- Download the latest release binary (or build from source if no release available)
- Install to `/usr/local/bin`
- Auto-add shell hook to your `.zshrc`, `.bashrc`, or `config.fish`

### From source

```bash
cargo install --git https://github.com/abdul-zailani/awsx
```

### Manual shell setup (if not using the installer)

```bash
# zsh
echo 'eval "$(awsx shell-hook zsh --prompt)"' >> ~/.zshrc

# bash
echo 'eval "$(awsx shell-hook bash --prompt)"' >> ~/.bashrc

# fish
echo 'awsx shell-hook fish --prompt | source' >> ~/.config/fish/config.fish
```

## Quick Start

Each engineer sets up contexts locally based on their own environment. No shared config needed.

### 1. Save contexts from current state (recommended)

Switch to your environment manually once, then let `awsx` capture it:

```bash
# Switch to your staging environment
export AWS_PROFILE=my-stg-profile
kubectl config use-context my-stg-cluster

# Save — awsx auto-detects current profile, region, and kubectl context
awsx save stg --environment staging
# ✓ Context 'stg' saved
#   AWS profile: my-stg-profile
#   Region: ap-southeast-1
#   K8s context: my-stg-cluster

# Repeat for other environments
export AWS_PROFILE=my-prd-profile
kubectl config use-context my-prd-cluster
awsx save prd --environment production
```

### 2. Or save with explicit flags

```bash
awsx save prd \
  --aws-profile my-prd-profile \
  --region ap-southeast-1 \
  --kube-context my-prd-cluster \
  --namespace default \
  --environment production
```

### 3. Switch

```bash
# Interactive picker
awsx use

# Direct switch
awsx use prd

# Show current status
awsx current

# List saved contexts
awsx list
#   dev  [DEV]  aws=my-dev-profile | region=ap-southeast-1 | k8s=my-dev-cluster | ns=default
#   prd  [PRD]  aws=my-prd-profile | region=ap-southeast-1 | k8s=my-prd-cluster | ns=default
#   stg  [STG]  aws=my-stg-profile | region=ap-southeast-1 | k8s=my-stg-cluster | ns=default
```

## Commands

| Command | Description |
|---------|-------------|
| `awsx use [name]` | Switch to saved context (interactive if no name) |
| `awsx profile [name]` | Switch AWS profile only |
| `awsx kube [name]` | Switch kubectl context only |
| `awsx save <name>` | Save a context (auto-detects current state, or use flags) |
| `awsx delete <name>` | Delete a saved context |
| `awsx list` | List all saved contexts |
| `awsx current` | Show current active context |
| `awsx shell-hook <shell>` | Output shell hook (zsh/bash/fish) |
| `awsx clear` | Unset all AWS environment variables |

### Save flags

All flags are optional — omitted values are auto-detected from current environment:

| Flag | Description | Auto-detect source |
|------|-------------|--------------------|
| `--aws-profile` | AWS CLI profile name | `$AWS_PROFILE` |
| `--region` | AWS region | `$AWS_DEFAULT_REGION` or `$AWS_REGION` |
| `--kube-context` | kubectl context name | `kubectl config current-context` |
| `--namespace` | Kubernetes namespace | — |
| `--environment` | Environment tag (production/staging/development) | — |

## Config

Contexts are stored locally per engineer in `~/.config/awsx/config.toml`:

```toml
[contexts.prd]
aws_profile = "my-prd-profile"
region = "ap-southeast-1"
kube_context = "my-prd-cluster"
namespace = "default"
environment = "production"

[contexts.stg]
aws_profile = "my-stg-profile"
region = "ap-southeast-1"
kube_context = "my-stg-cluster"
namespace = "default"
environment = "staging"
```

This file is local to each engineer — context names and kubectl context names can differ between machines.

## Requirements

- AWS CLI v2
- kubectl (optional, for k8s switching)

## License

MIT
