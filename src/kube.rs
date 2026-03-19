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
            println!("{} Kubernetes: {}", "✓".green(), short.cyan());
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
        println!("  Namespace: {}", ns.dimmed());
    }
}
