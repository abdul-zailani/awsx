use colored::Colorize;
use std::process::Command;

pub fn list_profiles() -> Vec<String> {
    let output = Command::new("aws")
        .args(["configure", "list-profiles"])
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

pub struct SessionInfo {
    pub account: String,
    pub arn: String,
}

pub fn check_session(profile: &str) -> Option<SessionInfo> {
    let output = Command::new("aws")
        .args(["sts", "get-caller-identity", "--output", "json", "--profile", profile])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).ok()?;
    Some(SessionInfo {
        account: json["Account"].as_str().unwrap_or("").to_string(),
        arn: json["Arn"].as_str().unwrap_or("").to_string(),
    })
}

pub fn sso_login(profile: &str) -> bool {
    println!("{} Session expired, logging in...", "🔐".to_string());
    Command::new("aws")
        .args(["sso", "login", "--profile", profile])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn switch_profile(profile: &str) {
    if check_session(profile).is_some() {
        print_session_info(profile);
    } else if sso_login(profile) {
        print_session_info(profile);
    } else {
        eprintln!("{} Failed to login to profile '{}'", "✗".red(), profile);
        std::process::exit(1);
    }
}

fn print_session_info(profile: &str) {
    if let Some(info) = check_session(profile) {
        let role = info.arn.split('/').nth(1).unwrap_or(&info.arn);
        println!("{} AWS profile: {} (account: {})", "✓".green(), profile.cyan(), info.account);
        println!("  Role: {}", role.dimmed());
    }
}

/// Output shell-eval commands to export AWS env vars
pub fn export_commands(profile: &str, region: Option<&str>) -> Vec<String> {
    let mut cmds = vec![format!("export AWS_PROFILE={profile}")];
    if let Some(r) = region {
        cmds.push(format!("export AWS_DEFAULT_REGION={r}"));
        cmds.push(format!("export AWS_REGION={r}"));
    }
    cmds
}
