use crate::config::{load_config, save_config, Context};
use crate::{aws, kube};
use colored::Colorize;

pub fn save_context(
    name: &str,
    aws_profile: Option<String>,
    region: Option<String>,
    kube_context: Option<String>,
    namespace: Option<String>,
    environment: Option<String>,
) {
    // Auto-detect from current state if not specified
    let aws_profile = aws_profile.or_else(|| std::env::var("AWS_PROFILE").ok());
    let region = region.or_else(|| {
        std::env::var("AWS_DEFAULT_REGION")
            .or_else(|_| std::env::var("AWS_REGION"))
            .ok()
    });
    let kube_context = kube_context.or_else(|| kube::current_context());

    let mut config = load_config();
    let ctx = Context {
        aws_profile: aws_profile.clone(),
        region: region.clone(),
        kube_context: kube_context.clone(),
        namespace,
        environment,
    };
    config.contexts.insert(name.to_string(), ctx);
    save_config(&config).expect("failed to save config");

    println!("{} Context '{}' saved", "✓".green(), name.cyan());
    if let Some(p) = &aws_profile {
        println!("  AWS profile: {}", p.dimmed());
    }
    if let Some(r) = &region {
        println!("  Region: {}", r.dimmed());
    }
    if let Some(k) = &kube_context {
        let short = k.rsplit('/').next().unwrap_or(k);
        println!("  K8s context: {}", short.dimmed());
    }
}

pub fn delete_context(name: &str) {
    let mut config = load_config();
    if config.contexts.remove(name).is_some() {
        save_config(&config).expect("failed to save config");
        println!("{} Context '{}' deleted", "✓".green(), name);
    } else {
        eprintln!("{} Context '{}' not found", "✗".red(), name);
        std::process::exit(1);
    }
}

pub fn list_contexts() {
    let config = load_config();
    if config.contexts.is_empty() {
        println!("No saved contexts. Use {} to create one.", "awsx save".cyan());
        return;
    }
    let max_name = config.contexts.keys().map(|k| k.len()).max().unwrap_or(0);
    for (name, ctx) in &config.contexts {
        let env_tag = match ctx.environment.as_deref() {
            Some("production" | "prd") => "PRD".red().bold(),
            Some("staging" | "stg") => "STG".yellow().bold(),
            Some("development" | "dev") => "DEV".green().bold(),
            Some(e) => e.normal(),
            None => "---".dimmed(),
        };
        println!("  {:<width$}  [{}]  {}", name.cyan(), env_tag, ctx, width = max_name);
    }
}
