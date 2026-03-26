use colored::Colorize;
use serde_json;
use std::fs;
use std::process::Command;

pub fn list_profiles() -> Vec<String> {
    let mut profiles = Vec::new();
    let config_path = dirs::home_dir().map(|h| h.join(".aws/config"));
    if let Some(path) = config_path {
        if let Ok(content) = fs::read_to_string(path) {
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("[profile ") && trimmed.ends_with(']') {
                    profiles.push(trimmed[9..trimmed.len() - 1].to_string());
                } else if trimmed == "[default]" {
                    profiles.push("default".to_string());
                }
            }
        }
    }
    profiles
}

pub struct SessionInfo {
    pub account: String,
    pub arn: String,
}

pub fn check_session(profile: &str) -> Option<SessionInfo> {
    // 1. Fast path: check local SSO cache
    if let Some(info) = check_sso_cache(profile) {
        return Some(info);
    }

    // 2. Fallback: call sts get-caller-identity
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

fn check_sso_cache(profile: &str) -> Option<SessionInfo> {
    let start_url = aws_config_get("sso_start_url", profile)?;
    let account_id = aws_config_get("sso_account_id", profile)?;
    let role_name = aws_config_get("sso_role_name", profile)?;

    let cache_dir = dirs::home_dir()?.join(".aws/sso/cache");
    if let Ok(entries) = fs::read_dir(cache_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                        if json["startUrl"].as_str() == Some(&start_url) {
                            if let Some(expires_at) = json["expiresAt"].as_str() {
                                if let Ok(expires) = chrono::DateTime::parse_from_rfc3339(expires_at) {
                                    if expires.timestamp() > chrono::Utc::now().timestamp() {
                                        return Some(SessionInfo {
                                            account: account_id.clone(),
                                            arn: format!("arn:aws:sts::{}:assumed-role/{}/local-check", account_id, role_name),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

pub fn sso_login(profile: &str) -> bool {
    eprintln!("{} Session expired, logging in...", "🔐".to_string());
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

pub fn get_profile_region(profile: &str) -> Option<String> {
    aws_config_get("region", profile)
}

pub fn get_profile_account_id(profile: &str) -> Option<String> {
    if let Some(id) = aws_config_get("sso_account_id", profile) {
        return Some(id);
    }
    if let Some(arn) = aws_config_get("role_arn", profile) {
        if let Some(id) = arn.split(':').nth(4) {
            if !id.is_empty() { return Some(id.to_string()); }
        }
    }
    None
}

fn aws_config_get(key: &str, profile: &str) -> Option<String> {
    let path = dirs::home_dir()?.join(".aws/config");
    let content = fs::read_to_string(path).ok()?;
    let mut in_profile = false;
    let target_profile = if profile == "default" {
        "[default]".to_string()
    } else {
        format!("[profile {}]", profile)
    };

    // First pass: check in profile block
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_profile = trimmed == target_profile;
            continue;
        }
        if in_profile && trimmed.starts_with(key) {
            if let Some(val) = trimmed.split('=').nth(1) {
                return Some(val.trim().to_string());
            }
        }
    }

    // Second pass: if key is sso_start_url or sso_region, check linked sso-session
    if key == "sso_start_url" || key == "sso_region" {
        if let Some(session_name) = aws_config_get("sso_session", profile) {
            let session_block = format!("[sso-session {}]", session_name);
            let mut in_session = false;
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with('[') {
                    in_session = trimmed == session_block;
                    continue;
                }
                if in_session && trimmed.starts_with(key) {
                    if let Some(val) = trimmed.split('=').nth(1) {
                        return Some(val.trim().to_string());
                    }
                }
            }
        }
    }

    None
}

pub fn export_commands(profile: &str, region: Option<&str>) -> Vec<String> {
    let mut cmds = vec![format!("export AWS_PROFILE={profile}")];
    if let Some(r) = region {
        cmds.push(format!("export AWS_DEFAULT_REGION={r}"));
        cmds.push(format!("export AWS_REGION={r}"));
    }
    cmds
}
