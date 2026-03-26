use colored::Colorize;
use serde_yaml;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn get_kubeconfig_path() -> Option<PathBuf> {
    if let Ok(val) = std::env::var("KUBECONFIG") {
        if !val.is_empty() {
            return Some(PathBuf::from(val));
        }
    }
    dirs::home_dir().map(|h| h.join(".kube/config"))
}

fn load_kubeconfig() -> Option<serde_yaml::Value> {
    let path = get_kubeconfig_path()?;
    let content = fs::read_to_string(path).ok()?;
    serde_yaml::from_str(&content).ok()
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
