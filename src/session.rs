/// Manages .claude/session.json persistence and archiving.
use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub ticket_key: String,
    pub summary: String,
    pub url: String,
    pub branch: String,
    pub created_at: String,
}

const SESSION_DIR: &str = ".claude";
const SESSION_FILE: &str = ".claude/session.json";
const HISTORY_DIR: &str = ".claude/history";

/// Write session.json and return the suggested branch name.
pub fn write_session(key: &str, summary: &str, domain: &str) -> Result<String> {
    let branch = make_branch_name(key, summary);
    let url = format!("https://{}/browse/{}", domain, key);

    let session = Session {
        ticket_key: key.to_string(),
        summary: summary.to_string(),
        url,
        branch: branch.clone(),
        created_at: Utc::now().to_rfc3339(),
    };

    fs::create_dir_all(SESSION_DIR)
        .with_context(|| format!("Could not create directory {}", SESSION_DIR))?;

    let json = serde_json::to_string_pretty(&session)?;
    fs::write(SESSION_FILE, json)
        .with_context(|| format!("Could not write {}", SESSION_FILE))?;

    Ok(branch)
}

/// Move session.json to .claude/history/<KEY>-<timestamp>.json
pub fn archive_session() -> Result<()> {
    let src = PathBuf::from(SESSION_FILE);
    if !src.exists() {
        anyhow::bail!("No active session found at {}", SESSION_FILE);
    }

    let content = fs::read_to_string(&src)?;
    let session: Session = serde_json::from_str(&content)
        .with_context(|| "session.json is malformed")?;

    fs::create_dir_all(HISTORY_DIR)
        .with_context(|| format!("Could not create {}", HISTORY_DIR))?;

    let ts = Utc::now().format("%Y%m%dT%H%M%S");
    let dest = PathBuf::from(format!(
        "{}/{}-{}.json",
        HISTORY_DIR, session.ticket_key, ts
    ));

    fs::rename(&src, &dest)
        .with_context(|| format!("Could not move session to {}", dest.display()))?;

    println!("Archived to {}", dest.display());
    Ok(())
}

/// Derive a branch slug: <KEY>-<slugified-summary> (no type prefix).
/// The `work-on` skill prepends the conventional commit type, e.g. `feat/<slug>`.
fn make_branch_name(key: &str, summary: &str) -> String {
    // Try to get current git branch; if already on a branch for this key, keep it
    let current = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                String::from_utf8(o.stdout).ok().map(|s| s.trim().to_string())
            } else {
                None
            }
        });

    if let Some(ref b) = current {
        if b.contains(&key.to_lowercase()) {
            return b.clone();
        }
    }

    let slug: String = summary
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .take(6)
        .collect::<Vec<_>>()
        .join("-");

    // Placeholder type — the work-on skill replaces this with the confirmed type
    format!("<type>/{}-{}", key.to_lowercase(), slug)
}
