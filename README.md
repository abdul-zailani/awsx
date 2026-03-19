# awsx

**AWS Context Switcher** — switch AWS profile + kubectl context + region in one command.

Built for DevOps/SRE engineers managing multiple AWS accounts and EKS clusters.

## Features

- 🔀 **Unified switching** — AWS profile + kubectl context + region in one command
- 💾 **Saved contexts** — define environment combos, switch instantly
- 🔍 **Fuzzy picker** — interactive selection powered by skim (Rust fzf)
- 🔐 **Auto SSO login** — detects expired sessions, triggers `aws sso login`
- 🐚 **Shell integration** — eval-based env export for zsh/bash/fish
- 🎨 **Color-coded** — environments tagged as PRD/STG/DEV with colors
- ⚡ **Fast** — native Rust binary, sub-millisecond startup

## Install

### From source

```bash
cargo install --git https://github.com/abdul-zailani/awsx
```

### From releases

```bash
# macOS (Apple Silicon)
curl -L https://github.com/abdul-zailani/awsx/releases/latest/download/awsx-aarch64-apple-darwin -o /usr/local/bin/awsx
chmod +x /usr/local/bin/awsx

# macOS (Intel)
curl -L https://github.com/abdul-zailani/awsx/releases/latest/download/awsx-x86_64-apple-darwin -o /usr/local/bin/awsx
chmod +x /usr/local/bin/awsx

# Linux
curl -L https://github.com/abdul-zailani/awsx/releases/latest/download/awsx-x86_64-unknown-linux-gnu -o /usr/local/bin/awsx
chmod +x /usr/local/bin/awsx
```

## Shell Setup

Add to your `~/.zshrc` (or `~/.bashrc`):

```bash
eval "$(awsx shell-hook zsh)"          # required: enables env export
eval "$(awsx shell-hook zsh --prompt)" # optional: adds prompt integration
```

## Quick Start

```bash
# Save contexts for your environments
awsx save gen-prd \
  --aws-profile lion-gen-prd \
  --region ap-southeast-1 \
  --kube-context arn:aws:eks:ap-southeast-1:106022784090:cluster/genesis-prd \
  --namespace default \
  --environment production

awsx save gen-stg \
  --aws-profile lion-gen-stg \
  --region ap-southeast-1 \
  --kube-context arn:aws:eks:ap-southeast-1:166984819683:cluster/genesis-stg \
  --namespace default \
  --environment staging

# Switch to a context (interactive picker)
awsx use

# Switch to a specific context
awsx use gen-prd

# List saved contexts
awsx list
#   gen-dev  [DEV]  aws=lion-gen-dev | region=ap-southeast-1 | k8s=genesis-dev | ns=default
#   gen-prd  [PRD]  aws=lion-gen-prd | region=ap-southeast-1 | k8s=genesis-prd | ns=default
#   gen-stg  [STG]  aws=lion-gen-stg | region=ap-southeast-1 | k8s=genesis-stg | ns=default

# Show current status
awsx current

# Switch AWS profile only (interactive)
awsx profile

# Switch kubectl context only (interactive)
awsx kube

# Clear all AWS env vars
awsx clear
```

## Commands

| Command | Description |
|---------|-------------|
| `awsx use [name]` | Switch to saved context (interactive if no name) |
| `awsx profile [name]` | Switch AWS profile only |
| `awsx kube [name]` | Switch kubectl context only |
| `awsx save <name>` | Save a context with `--aws-profile`, `--region`, `--kube-context`, `--namespace`, `--environment` |
| `awsx delete <name>` | Delete a saved context |
| `awsx list` | List all saved contexts |
| `awsx current` | Show current active context |
| `awsx shell-hook <shell>` | Output shell hook (zsh/bash/fish) |
| `awsx clear` | Unset all AWS environment variables |

## Config

Contexts are stored in `~/.config/awsx/config.toml`:

```toml
[contexts.gen-prd]
aws_profile = "lion-gen-prd"
region = "ap-southeast-1"
kube_context = "arn:aws:eks:ap-southeast-1:106022784090:cluster/genesis-prd"
namespace = "default"
environment = "production"
```

## Requirements

- AWS CLI v2
- kubectl (optional, for k8s switching)

## License

MIT
