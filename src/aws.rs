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
    let info = if let Some(info) = check_session(profile) {
        info
    } else if sso_login(profile) {
        check_session(profile).unwrap()
    } else {
        eprintln!("{} Failed to login to profile '{}'", "✗".red(), profile);
        std::process::exit(1);
    };
    let role = info.arn.split('/').nth(1).unwrap_or(&info.arn);
    eprintln!("{} AWS profile: {} (account: {})", "✓".green(), profile.cyan(), info.account);
    eprintln!("  Role: {}", role.dimmed());
}

/// Read region from aws config for a profile
pub fn get_profile_region(profile: &str) -> Option<String> {
    aws_config_get("region", profile)
}

/// Read AWS account ID from profile config.
/// Tries: sso_account_id, role_arn (contains account), source_profile chain.
pub fn get_profile_account_id(profile: &str) -> Option<String> {
    // Try sso_account_id (SSO profiles)
    if let Some(id) = aws_config_get("sso_account_id", profile) {
        return Some(id);
    }
    // Try role_arn (assume-role profiles): arn:aws:iam::<account>:role/...
    if let Some(arn) = aws_config_get("role_arn", profile) {
        if let Some(id) = arn.split(':').nth(4) {
            if !id.is_empty() { return Some(id.to_string()); }
        }
    }
    None
}

fn aws_config_get(key: &str, profile: &str) -> Option<String> {
    let output = Command::new("aws")
        .args(["configure", "get", key, "--profile", profile])
        .output()
        .ok()?;
    if output.status.success() {
        let val = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if val.is_empty() { None } else { Some(val) }
    } else {
        None
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
