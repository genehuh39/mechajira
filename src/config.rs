use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AppConfig {
    pub email: String,
    pub api_token: String,
    pub domain: String, // always stored without scheme, e.g. "mycompany.atlassian.net"
}

impl AppConfig {
    pub fn is_complete(&self) -> bool {
        !self.email.is_empty() && !self.api_token.is_empty() && !self.domain.is_empty()
    }

    pub fn basic_auth(&self) -> String {
        use base64::{engine::general_purpose::STANDARD, Engine};
        STANDARD.encode(format!("{}:{}", self.email, self.api_token).as_bytes())
    }
}

/// ~/.config/jplan/config.json
pub fn config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".config").join("jplan").join("config.json")
}

pub fn load_config() -> Result<AppConfig> {
    let path = config_path();
    if !path.exists() {
        return Ok(AppConfig::default());
    }
    let raw = fs::read_to_string(&path)
        .with_context(|| format!("Could not read config at {}", path.display()))?;
    serde_json::from_str(&raw)
        .with_context(|| format!("Config at {} is malformed", path.display()))
}

pub fn save_config(cfg: &AppConfig) -> Result<()> {
    let path = config_path();
    fs::create_dir_all(path.parent().unwrap())?;
    let json = serde_json::to_string_pretty(cfg)?;
    fs::write(&path, json)
        .with_context(|| format!("Could not write config to {}", path.display()))?;
    println!("Config saved to {}", path.display());
    Ok(())
}

pub fn print_config() -> Result<()> {
    let path = config_path();
    let cfg = load_config()?;
    println!("Config file : {}", path.display());
    if cfg.is_complete() {
        let token_preview = if cfg.api_token.len() > 8 {
            format!("{}...", &cfg.api_token[..8])
        } else {
            "(set)".to_string()
        };
        println!("Domain      : {}", cfg.domain);
        println!("Email       : {}", cfg.email);
        println!("API token   : {}", token_preview);
    } else {
        println!("(no configuration found — run `jplan --setup`)");
    }
    Ok(())
}

/// Strip scheme and trailing slashes from domain input.
fn normalize_domain(raw: &str) -> String {
    raw.trim()
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches('/')
        .to_string()
}

pub fn run_setup_wizard(existing: Option<AppConfig>) -> Result<AppConfig> {
    let e = existing.unwrap_or_default();
    let theme = ColorfulTheme::default();

    println!("=== jplan Setup ===\n");

    let domain_raw: String = Input::with_theme(&theme)
        .with_prompt("Jira domain (e.g. mycompany.atlassian.net)")
        .default(e.domain.clone())
        .interact_text()?;
    let domain = normalize_domain(&domain_raw);

    let email: String = Input::with_theme(&theme)
        .with_prompt("Jira email")
        .default(e.email.clone())
        .interact_text()?;

    let token_prompt = if e.api_token.is_empty() {
        "Jira API token".to_string()
    } else {
        let preview = &e.api_token[..8.min(e.api_token.len())];
        format!("Jira API token (leave blank to keep {}...)", preview)
    };
    let api_token_input: String = Input::with_theme(&theme)
        .with_prompt(token_prompt)
        .allow_empty(true)
        .interact_text()?;
    let api_token = if api_token_input.trim().is_empty() {
        e.api_token.clone()
    } else {
        api_token_input.trim().to_string()
    };

    Ok(AppConfig { email: email.trim().to_string(), api_token, domain })
}
