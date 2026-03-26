use colored::Colorize;
use serde_yaml;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Returns all kubeconfig file paths, respecting the KUBECONFIG env var
/// which can contain colon-separated paths (standard kubectl behavior).
fn get_kubeconfig_paths() -> Vec<PathBuf> {
    if let Ok(val) = std::env::var("KUBECONFIG") {
        if !val.is_empty() {
            return val.split(':').map(PathBuf::from).collect();
        }
    }
    dirs::home_dir()
        .map(|h| vec![h.join(".kube/config")])
        .unwrap_or_default()
}

/// Loads and merges all kubeconfig files. Merging follows kubectl semantics:
/// - `current-context`: first non-empty value wins
/// - `contexts`, `clusters`, `users`: merged into a single list (first occurrence wins on name)
fn load_kubeconfig() -> Option<serde_yaml::Value> {
    let paths = get_kubeconfig_paths();
    let mut merged: Option<serde_yaml::Value> = None;

    for path in paths {
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let yaml: serde_yaml::Value = match serde_yaml::from_str(&content) {
            Ok(y) => y,
            Err(_) => continue,
        };
        match merged {
            None => merged = Some(yaml),
            Some(ref mut base) => {
                // Merge current-context: first non-empty wins
                if base.get("current-context").and_then(|v| v.as_str()).unwrap_or("").is_empty() {
                    if let Some(ctx) = yaml.get("current-context") {
                        if ctx.as_str().map_or(false, |s| !s.is_empty()) {
                            base["current-context"] = ctx.clone();
                        }
                    }
                }
                // Merge list fields (contexts, clusters, users)
                for key in &["contexts", "clusters", "users"] {
                    if let Some(serde_yaml::Value::Sequence(new_items)) = yaml.get(*key) {
                        let base_seq = base
                            .get_mut(*key)
                            .and_then(|v| v.as_sequence_mut());
                        match base_seq {
                            Some(existing) => {
                                for item in new_items {
                                    let name = item.get("name").and_then(|n| n.as_str());
                                    if let Some(n) = name {
                                        let already = existing.iter().any(|e| {
                                            e.get("name").and_then(|v| v.as_str()) == Some(n)
                                        });
                                        if !already {
                                            existing.push(item.clone());
                                        }
                                    }
                                }
                            }
                            None => {
                                base[*key] = serde_yaml::Value::Sequence(new_items.clone());
                            }
                        }
                    }
                }
            }
        }
    }
    merged
}

pub fn list_contexts() -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    if let Some(yaml) = load_kubeconfig() {
        if let Some(contexts) = yaml["contexts"].as_sequence() {
            for ctx in contexts {
                if let Some(name) = ctx["name"].as_str() {
                    names.push(name.to_string());
                }
            }
        }
    }
    names
}

pub fn get_context_clusters() -> HashMap<String, String> {
    let mut map = HashMap::new();
    if let Some(yaml) = load_kubeconfig() {
        if let Some(contexts) = yaml["contexts"].as_sequence() {
            for ctx in contexts {
                let name = ctx["name"].as_str().unwrap_or_default();
                let cluster = ctx["context"]["cluster"].as_str().unwrap_or_default();
                if !name.is_empty() && !cluster.is_empty() {
                    map.insert(name.to_string(), cluster.to_string());
                }
            }
        }
    }
    map
}

pub fn current_context() -> Option<String> {
    let yaml = load_kubeconfig()?;
    yaml["current-context"].as_str().map(|s| s.to_string())
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
