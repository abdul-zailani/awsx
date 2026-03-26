---
description: Core bug prevention and quality assurance guidelines for awsx development
---

When developing features or fixing bugs in `awsx`, you must adhere to the following general constraints to prevent regressions:

1. **Environment Variables**: Never assume an env var holds a single simple value. Variables like `KUBECONFIG` can contain multiple paths (colon-separated). Split, merge, and handle them gracefully.
2. **Missing Dependencies**: Do not panic if `~/.kube/config` or `~/.aws/config` does not exist or has permission issues. Skip them silently or warn the user.
3. **Shell Output Strictness**: The shell hook (`awsx` evaluation) relies on stdout. ALL user-facing messages, logs, and UI components must go to stderr (`eprintln!`). ONLY shell injection commands (`export`, `set`) go to stdout.
4. **Assume Complex States**: Before declaring a task finished, test what happens if `AWSX_CONTEXT` is set but the corresponding TOML config doesn't have an `aws_profile` or `kube_context`. Do not assume all config fields are universally populated.
5. **Local Verification**: Always run `cargo build` and test the compiled binary simulating both empty configurations and complex multi-path overrides.
