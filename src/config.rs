use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub contexts: BTreeMap<String, Context>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub aws_profile: Option<String>,
    pub region: Option<String>,
    pub kube_context: Option<String>,
    pub namespace: Option<String>,
    pub environment: Option<String>,
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<String> = [
            self.aws_profile.as_ref().map(|v| format!("aws={v}")),
            self.region.as_ref().map(|v| format!("region={v}")),
            self.kube_context.as_ref().map(|v| {
                let short = v.rsplit('/').next().unwrap_or(v);
                format!("k8s={short}")
            }),
            self.namespace.as_ref().map(|v| format!("ns={v}")),
        ]
        .into_iter()
        .flatten()
        .collect();
        write!(f, "{}", parts.join(" | "))
    }
}

pub fn config_path() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    home.join(".config").join("awsx").join("config.toml")
}

pub fn load_config() -> AppConfig {
    let path = config_path();
    if path.exists() {
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_default()
    } else {
        AppConfig::default()
    }
}

pub fn save_config(config: &AppConfig) -> std::io::Result<()> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let content = toml::to_string_pretty(config).expect("failed to serialize config");
    std::fs::write(&path, content)
}
