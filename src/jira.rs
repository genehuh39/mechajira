use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

use crate::config::AppConfig;

pub struct JiraClient {
    client: Client,
    base_url: String,
    auth: String,
}

impl JiraClient {
    pub fn new(cfg: &AppConfig) -> Result<Self> {
        // Allow .env overrides at runtime
        let email = std::env::var("JIRA_EMAIL").unwrap_or_else(|_| cfg.email.clone());
        let token = std::env::var("JIRA_API_TOKEN").unwrap_or_else(|_| cfg.api_token.clone());
        let domain = std::env::var("JIRA_DOMAIN").unwrap_or_else(|_| cfg.domain.clone());

        let tmp_cfg = AppConfig { email, api_token: token, domain: domain.clone() };
        Ok(Self {
            client: Client::new(),
            base_url: format!("https://{}/rest/api/3", domain),
            auth: tmp_cfg.basic_auth(),
        })
    }

    pub async fn get_ticket(&self, key: &str) -> Result<JiraIssue> {
        let url = format!("{}/issue/{}", self.base_url, key);
        let resp = self
            .client
            .get(&url)
            .header("Authorization", format!("Basic {}", self.auth))
            .header("Accept", "application/json")
            .send()
            .await
            .with_context(|| format!("Failed to connect to Jira for ticket {}", key))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("Jira API error {}: {}", status, body);
        }

        resp.json::<JiraIssue>()
            .await
            .with_context(|| "Failed to parse Jira response")
    }
}

// ---------- Jira API response types ----------

#[derive(Debug, Deserialize)]
pub struct JiraIssue {
    pub fields: IssueFields,
}

#[derive(Debug, Deserialize)]
pub struct IssueFields {
    pub summary: String,
    pub description: Option<Value>, // ADF or null
    pub status: Status,
    pub assignee: Option<User>,
    pub comment: Option<CommentPage>,
}

#[derive(Debug, Deserialize)]
pub struct Status {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct User {
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[derive(Debug, Deserialize)]
pub struct CommentPage {
    pub comments: Vec<Comment>,
}

#[derive(Debug, Deserialize)]
pub struct Comment {
    pub author: User,
    pub body: Value, // ADF body
}
