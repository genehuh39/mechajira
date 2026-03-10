mod config;
mod jira;
mod adf;
mod scout;
mod session;
mod output;

use anyhow::Result;
use clap::Parser;
use config::AppConfig;

#[derive(Parser)]
#[command(
    name = "jplan",
    about = "Fetch Jira tickets and scaffold Claude Code Plan Mode sessions",
    version
)]
struct Cli {
    /// Jira ticket key (e.g. PROJ-123)
    ticket_key: Option<String>,

    /// Archive current session and clear active state
    #[arg(long)]
    archive: bool,

    /// Run the setup wizard to configure credentials
    #[arg(long)]
    setup: bool,

    /// Show current stored configuration
    #[arg(long)]
    config: bool,

    /// Copy work-on and finish-work skills into .claude/skills/ in the current directory
    #[arg(long)]
    install_skills: bool,
}

fn install_skills() -> Result<()> {
    let src = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
        .join(".local/share/jplan/skills");

    if !src.exists() {
        anyhow::bail!(
            "Skills source not found at {}.\nRe-run the installer: ./install.sh",
            src.display()
        );
    }

    let dest = std::path::Path::new(".claude/skills");
    std::fs::create_dir_all(dest)?;

    for entry in std::fs::read_dir(&src)? {
        let entry = entry?;
        let skill_dir = entry.path();
        if skill_dir.is_dir() {
            let skill_name = skill_dir.file_name().unwrap();
            let target = dest.join(skill_name);
            std::fs::create_dir_all(&target)?;
            for file in std::fs::read_dir(&skill_dir)? {
                let file = file?;
                std::fs::copy(file.path(), target.join(file.file_name()))?;
            }
            println!("✓ Installed skill: {}", skill_name.to_string_lossy());
        }
    }

    println!("\nSkills installed to .claude/skills/");
    println!("Use /work-on <KEY> and /finish-work in Claude Code.");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    let cli = Cli::parse();

    if cli.config {
        return config::print_config();
    }

    if cli.install_skills {
        return install_skills();
    }

    if cli.setup {
        let existing = config::load_config().ok();
        let cfg = config::run_setup_wizard(existing)?;
        config::save_config(&cfg)?;
        return Ok(());
    }

    if cli.archive {
        session::archive_session()?;
        println!("Session archived. Active session cleared.");
        return Ok(());
    }

    let key = match cli.ticket_key {
        Some(k) => k.to_uppercase(),
        None => {
            eprintln!("Usage: jplan <TICKET-KEY> [--archive] [--setup] [--config]");
            std::process::exit(1);
        }
    };

    let cfg: AppConfig = {
        let loaded = config::load_config()?;
        if loaded.is_complete() {
            loaded
        } else {
            println!("Jira credentials are not configured. Starting setup wizard...\n");
            let cfg = config::run_setup_wizard(Some(loaded))?;
            config::save_config(&cfg)?;
            cfg
        }
    };

    // Apply .env overrides at runtime (not stored)
    let cfg = AppConfig {
        email: std::env::var("JIRA_EMAIL").unwrap_or(cfg.email),
        api_token: std::env::var("JIRA_API_TOKEN").unwrap_or(cfg.api_token),
        domain: std::env::var("JIRA_DOMAIN").unwrap_or(cfg.domain),
    };

    let client = jira::JiraClient::new(&cfg)?;
    let ticket = client.get_ticket(&key).await?;

    let description_md = adf::adf_to_markdown(&ticket.fields.description);

    let comments_md = ticket
        .fields
        .comment
        .as_ref()
        .map(|c| {
            c.comments
                .iter()
                .rev()
                .take(5)
                .rev()
                .map(|cm| {
                    let author = &cm.author.display_name;
                    let body = adf::adf_to_markdown(&Some(cm.body.clone()));
                    format!("> **{}**: {}", author, body.replace('\n', "\n> "))
                })
                .collect::<Vec<_>>()
                .join("\n\n")
        })
        .unwrap_or_default();

    let refs = scout::find_code_references(&description_md, &comments_md);
    let branch = session::write_session(&key, &ticket.fields.summary, &cfg.domain)?;

    output::print_plan(
        &key,
        &ticket.fields.summary,
        &ticket.fields.status.name,
        ticket.fields.assignee.as_ref().map(|a| a.display_name.as_str()),
        &description_md,
        &comments_md,
        &refs,
        &branch,
        &cfg.domain,
    );

    Ok(())
}
