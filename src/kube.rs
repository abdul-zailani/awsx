use colored::Colorize;
use std::process::Command;

pub fn list_contexts() -> Vec<String> {
    let output = Command::new("kubectl")
        .args(["config", "get-contexts", "-o", "name"])
        .output();
    match output {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout)
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect(),
        _ => vec![],
    }
}

/// Get cluster server/ARN for each kubectl context
/// Returns map of context_name → cluster_name (often an EKS ARN)
pub fn get_context_clusters() -> std::collections::HashMap<String, String> {
    let output = Command::new("kubectl")
        .args(["config", "view", "-o", "json"])
        .output();
    let mut map = std::collections::HashMap::new();
    let Ok(o) = output else { return map };
    if !o.status.success() { return map; }
    let Ok(json) = serde_json::from_slice::<serde_json::Value>(&o.stdout) else { return map };
    if let Some(contexts) = json["contexts"].as_array() {
        for ctx in contexts {
            let name = ctx["name"].as_str().unwrap_or_default();
            let cluster = ctx["context"]["cluster"].as_str().unwrap_or_default();
            if !name.is_empty() && !cluster.is_empty() {
                map.insert(name.to_string(), cluster.to_string());
            }
        }
    }
    map
}

pub fn current_context() -> Option<String> {
    let output = Command::new("kubectl")
        .args(["config", "current-context"])
        .output()
        .ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

pub fn switch_context(context: &str, namespace: Option<&str>) {
    let status = Command::new("kubectl")
        .args(["config", "use-context", context])
        .output();
    match status {
        Ok(o) if o.status.success() => {
            let short = context.rsplit('/').next().unwrap_or(context);
            eprintln!("{} Kubernetes: {}", "✓".green(), short.cyan());
        }
        _ => {
            eprintln!("{} Failed to switch kubectl context '{}'", "✗".red(), context);
            return;
        }
    }
    if let Some(ns) = namespace {
        let _ = Command::new("kubectl")
            .args(["config", "set-context", "--current", "--namespace", ns])
            .output();
        eprintln!("  Namespace: {}", ns.dimmed());
    }
}
