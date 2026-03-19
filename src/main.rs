mod aws;
mod config;
mod context;
mod interactive;
mod kube;
mod shell;

use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(name = "awsx", version, about = "AWS Context Switcher — switch AWS profile + kubectl context in one command")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Auto-discover AWS profiles and kubectl contexts, generate saved contexts
    Init,
    /// Switch to a saved context (or pick interactively)
    Use {
        /// Context name
        name: Option<String>,
    },
    /// Switch AWS profile only
    Profile {
        /// Profile name (interactive if omitted)
        name: Option<String>,
    },
    /// Switch kubectl context only
    Kube {
        /// Context name (interactive if omitted)
        name: Option<String>,
        /// Namespace
        #[arg(short, long)]
        namespace: Option<String>,
    },
    /// Save current or specified context
    Save {
        /// Context name
        name: String,
        /// AWS profile
        #[arg(long)]
        aws_profile: Option<String>,
        /// AWS region
        #[arg(long)]
        region: Option<String>,
        /// Kubectl context
        #[arg(long)]
        kube_context: Option<String>,
        /// Kubernetes namespace
        #[arg(long)]
        namespace: Option<String>,
        /// Environment tag (production, staging, development)
        #[arg(long)]
        environment: Option<String>,
    },
    /// Delete a saved context
    Delete {
        /// Context name
        name: String,
    },
    /// List saved contexts
    List,
    /// Show current active context
    Current,
    /// Output shell hook (add to .zshrc/.bashrc)
    ShellHook {
        /// Shell type: zsh, bash, fish
        shell: String,
        /// Include prompt integration
        #[arg(long)]
        prompt: bool,
    },
    /// Clear AWS environment variables
    Clear,
}

fn cmd_init() {
    let profiles = aws::list_profiles();
    let kube_contexts = kube::list_contexts();
    let mut config = config::load_config();
    let mut count = 0;

    println!("{}", "Scanning AWS profiles and kubectl contexts...".dimmed());
    println!();

    if !profiles.is_empty() {
        println!("  {} AWS profiles found:", profiles.len().to_string().cyan());
        for p in &profiles {
            println!("    {}", p);
        }
    }
    if !kube_contexts.is_empty() {
        println!("  {} kubectl contexts found:", kube_contexts.len().to_string().cyan());
        for k in &kube_contexts {
            println!("    {}", k);
        }
    }
    println!();

    // Try to match AWS profiles with kubectl contexts by common name patterns
    for profile in &profiles {
        // Skip if context already exists
        if config.contexts.contains_key(profile) {
            continue;
        }

        // Try to find matching kubectl context
        let kube_match = find_kube_match(profile, &kube_contexts);

        // Detect environment from name
        let environment = detect_environment(profile);

        // Detect region from aws config
        let region = aws::get_profile_region(profile);

        let ctx = config::Context {
            aws_profile: Some(profile.clone()),
            region,
            kube_context: kube_match.clone(),
            namespace: None,
            environment,
        };

        let display = format!("{}", ctx);
        config.contexts.insert(profile.clone(), ctx);
        count += 1;
        println!(
            "  {} {} → {}",
            "✓".green(),
            profile.cyan(),
            display.dimmed()
        );
    }

    // Add kubectl contexts that didn't match any AWS profile
    for kctx in &kube_contexts {
        let already_mapped = config.contexts.values().any(|c| {
            c.kube_context.as_deref() == Some(kctx)
        });
        if already_mapped || config.contexts.contains_key(kctx) {
            continue;
        }

        let environment = detect_environment(kctx);
        let ctx = config::Context {
            aws_profile: None,
            region: None,
            kube_context: Some(kctx.clone()),
            namespace: None,
            environment,
        };
        config.contexts.insert(kctx.clone(), ctx);
        count += 1;
        println!(
            "  {} {} → {}",
            "✓".green(),
            kctx.cyan(),
            "k8s only".dimmed()
        );
    }

    if count == 0 {
        println!("  No new contexts to add (all already configured).");
    } else {
        config::save_config(&config).expect("failed to save config");
        println!();
        println!(
            "{} {} contexts saved. Run {} to see them.",
            "✓".green(),
            count,
            "awsx list".cyan()
        );
    }
}

fn find_kube_match(profile: &str, kube_contexts: &[String]) -> Option<String> {
    // Exact match
    if kube_contexts.contains(&profile.to_string()) {
        return Some(profile.to_string());
    }

    // Normalize: strip common prefixes like "lion-" and compare
    let normalized = profile
        .replace("lion-", "")
        .replace("aws-", "")
        .replace("_", "-");

    for kctx in kube_contexts {
        let k_normalized = kctx
            .replace("lion-", "")
            .replace("aws-", "")
            .replace("_", "-");

        if normalized == k_normalized {
            return Some(kctx.clone());
        }

        // Partial: profile contains kube name or vice versa
        if normalized.contains(&k_normalized) || k_normalized.contains(&normalized) {
            return Some(kctx.clone());
        }
    }
    None
}

fn detect_environment(name: &str) -> Option<String> {
    let lower = name.to_lowercase();
    if lower.contains("prd") || lower.contains("prod") {
        Some("production".to_string())
    } else if lower.contains("stg") || lower.contains("staging") {
        Some("staging".to_string())
    } else if lower.contains("dev") {
        Some("development".to_string())
    } else {
        None
    }
}

fn cmd_use(name: Option<String>) {
    let cfg = config::load_config();
    let ctx_name = match name {
        Some(n) => n,
        None => {
            let names: Vec<String> = cfg.contexts.keys().cloned().collect();
            if names.is_empty() {
                eprintln!("No saved contexts. Run {} to auto-discover.", "awsx init".cyan());
                std::process::exit(1);
            }
            match interactive::pick(&names, "Context> ") {
                Some(n) => n,
                None => return,
            }
        }
    };
    let ctx = match cfg.contexts.get(&ctx_name) {
        Some(c) => c,
        None => {
            eprintln!("{} Context '{}' not found", "✗".red(), ctx_name);
            std::process::exit(1);
        }
    };

    println!("export AWSX_CONTEXT={ctx_name}");

    if let Some(profile) = &ctx.aws_profile {
        for cmd in aws::export_commands(profile, ctx.region.as_deref()) {
            println!("{cmd}");
        }
        aws::switch_profile(profile);
    }

    if let Some(kctx) = &ctx.kube_context {
        kube::switch_context(kctx, ctx.namespace.as_deref());
    }
}

fn cmd_profile(name: Option<String>) {
    let profile = match name {
        Some(n) => n,
        None => {
            let profiles = aws::list_profiles();
            if profiles.is_empty() {
                eprintln!("{} No AWS profiles found in ~/.aws/config", "✗".red());
                std::process::exit(1);
            }
            match interactive::pick(&profiles, "AWS Profile> ") {
                Some(p) => p,
                None => return,
            }
        }
    };
    for cmd in aws::export_commands(&profile, None) {
        println!("{cmd}");
    }
    aws::switch_profile(&profile);
}

fn cmd_kube(name: Option<String>, namespace: Option<String>) {
    let context = match name {
        Some(n) => n,
        None => {
            let contexts = kube::list_contexts();
            if contexts.is_empty() {
                eprintln!("{} No kubectl contexts found", "✗".red());
                std::process::exit(1);
            }
            match interactive::pick(&contexts, "K8s Context> ") {
                Some(c) => c,
                None => return,
            }
        }
    };
    kube::switch_context(&context, namespace.as_deref());
}

fn cmd_current() {
    if let Ok(profile) = std::env::var("AWS_PROFILE") {
        print!("{} AWS: {}", "☁️".to_string(), profile.cyan());
        if let Ok(region) = std::env::var("AWS_DEFAULT_REGION") {
            print!(" ({})", region);
        }
        println!();
    } else {
        println!("{} AWS: {}", "☁️".to_string(), "not set".dimmed());
    }

    match kube::current_context() {
        Some(ctx) => {
            let short = ctx.rsplit('/').next().unwrap_or(&ctx);
            println!("{} K8s: {}", "☸".to_string(), short.cyan());
        }
        None => println!("{} K8s: {}", "☸".to_string(), "not set".dimmed()),
    }

    if let Ok(ctx) = std::env::var("AWSX_CONTEXT") {
        println!("{} Context: {}", "📌".to_string(), ctx.cyan().bold());
    }
}

fn cmd_clear() {
    for var in ["AWS_PROFILE", "AWS_DEFAULT_REGION", "AWS_REGION", "AWS_ACCESS_KEY_ID", "AWS_SECRET_ACCESS_KEY", "AWS_SESSION_TOKEN", "AWSX_CONTEXT"] {
        println!("unset {var}");
    }
    println!("{} AWS environment cleared", "✓".green());
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Init) => cmd_init(),
        Some(Commands::Use { name }) => cmd_use(name),
        Some(Commands::Profile { name }) => cmd_profile(name),
        Some(Commands::Kube { name, namespace }) => cmd_kube(name, namespace),
        Some(Commands::Save { name, aws_profile, region, kube_context, namespace, environment }) => {
            context::save_context(&name, aws_profile, region, kube_context, namespace, environment);
        }
        Some(Commands::Delete { name }) => context::delete_context(&name),
        Some(Commands::List) => context::list_contexts(),
        Some(Commands::Current) => cmd_current(),
        Some(Commands::ShellHook { shell, prompt }) => {
            shell::shell_hook(&shell);
            if prompt {
                shell::prompt_hook(&shell);
            }
        }
        Some(Commands::Clear) => cmd_clear(),
        None => cmd_current(),
    }
}
