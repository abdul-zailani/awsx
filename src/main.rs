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

fn cmd_use(name: Option<String>) {
    let cfg = config::load_config();
    let ctx_name = match name {
        Some(n) => n,
        None => {
            let names: Vec<String> = cfg.contexts.keys().cloned().collect();
            if names.is_empty() {
                eprintln!("No saved contexts. Use {} to create one.", "awsx save".cyan());
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

    // Export AWSX_CONTEXT for prompt
    println!("export AWSX_CONTEXT={ctx_name}");

    // AWS
    if let Some(profile) = &ctx.aws_profile {
        for cmd in aws::export_commands(profile, ctx.region.as_deref()) {
            println!("{cmd}");
        }
        aws::switch_profile(profile);
    }

    // Kubernetes
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
    // AWS
    if let Ok(profile) = std::env::var("AWS_PROFILE") {
        print!("{} AWS: {}", "☁️".to_string(), profile.cyan());
        if let Ok(region) = std::env::var("AWS_DEFAULT_REGION") {
            print!(" ({})", region);
        }
        println!();
    } else {
        println!("{} AWS: {}", "☁️".to_string(), "not set".dimmed());
    }

    // K8s
    match kube::current_context() {
        Some(ctx) => {
            let short = ctx.rsplit('/').next().unwrap_or(&ctx);
            println!("{} K8s: {}", "☸".to_string(), short.cyan());
        }
        None => println!("{} K8s: {}", "☸".to_string(), "not set".dimmed()),
    }

    // Context
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
