/// Convert an Atlassian Document Format (ADF) JSON value into GitHub-Flavored Markdown.
/// Handles: paragraph, heading, bulletList, orderedList, listItem, codeBlock,
///          blockquote, rule, hardBreak, text (with marks: strong, em, code, link, strike).
use serde_json::Value;

pub fn adf_to_markdown(adf: &Option<Value>) -> String {
    match adf {
        None => String::new(),
        Some(v) => render_node(v, 0).trim_end().to_string(),
    }
}

fn render_node(node: &Value, list_depth: usize) -> String {
    let node_type = node["type"].as_str().unwrap_or("");

    match node_type {
        "doc" => render_children(node, list_depth),

        "paragraph" => {
            let content = render_children(node, list_depth);
            if content.trim().is_empty() {
                String::new()
            } else {
                format!("{}\n\n", content)
            }
        }

        "heading" => {
            let level = node["attrs"]["level"].as_u64().unwrap_or(1) as usize;
            let prefix = "#".repeat(level.clamp(1, 6));
            let content = render_children(node, list_depth);
            format!("{} {}\n\n", prefix, content.trim())
        }

        "bulletList" => {
            let items = render_list_items(node, list_depth, false);
            format!("{}\n", items)
        }

        "orderedList" => {
            let items = render_list_items(node, list_depth, true);
            format!("{}\n", items)
        }

        "listItem" => {
            // Rendered by render_list_items directly
            render_children(node, list_depth)
        }

        "codeBlock" => {
            let lang = node["attrs"]["language"]
                .as_str()
                .unwrap_or("")
                .to_string();
            let content = render_children(node, list_depth);
            format!("```{}\n{}\n```\n\n", lang, content.trim_end())
        }

        "blockquote" => {
            let content = render_children(node, list_depth);
            let quoted = content
                .lines()
                .map(|l| format!("> {}", l))
                .collect::<Vec<_>>()
                .join("\n");
            format!("{}\n\n", quoted)
        }

        "rule" => "---\n\n".to_string(),

        "hardBreak" => "\n".to_string(),

        "text" => render_text(node),

        "inlineCard" | "blockCard" => {
            let url = node["attrs"]["url"].as_str().unwrap_or("");
            format!("[{}]({})", url, url)
        }

        "mediaSingle" | "media" => {
            // Skip media attachments gracefully
            String::new()
        }

        "table" => render_table(node, list_depth),

        _ => render_children(node, list_depth),
    }
}

fn render_children(node: &Value, list_depth: usize) -> String {
    node["content"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .map(|child| render_node(child, list_depth))
                .collect::<Vec<_>>()
                .join("")
        })
        .unwrap_or_default()
}

fn render_list_items(node: &Value, list_depth: usize, ordered: bool) -> String {
    let indent = "  ".repeat(list_depth);
    node["content"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .enumerate()
                .map(|(i, item)| {
                    let marker = if ordered {
                        format!("{}{}. ", indent, i + 1)
                    } else {
                        format!("{}- ", indent)
                    };
                    let body = render_children(item, list_depth + 1);
                    format!("{}{}", marker, body.trim_start())
                })
                .collect::<Vec<_>>()
                .join("")
        })
        .unwrap_or_default()
}

fn render_text(node: &Value) -> String {
    let raw = node["text"].as_str().unwrap_or("").to_string();
    if raw.is_empty() {
        return String::new();
    }

    let marks = node["marks"].as_array();
    if marks.is_none() {
        return raw;
    }

    let mut result = raw;
    for mark in marks.unwrap() {
        let mark_type = mark["type"].as_str().unwrap_or("");
        result = match mark_type {
            "strong" => format!("**{}**", result),
            "em" => format!("_{}_", result),
            "code" => format!("`{}`", result),
            "strike" => format!("~~{}~~", result),
            "link" => {
                let href = mark["attrs"]["href"].as_str().unwrap_or("#");
                format!("[{}]({})", result, href)
            }
            _ => result,
        };
    }
    result
}

fn render_table(node: &Value, list_depth: usize) -> String {
    let rows = match node["content"].as_array() {
        Some(r) => r,
        None => return String::new(),
    };

    let mut output = String::new();
    let mut header_done = false;

    for (row_idx, row) in rows.iter().enumerate() {
        let cells = match row["content"].as_array() {
            Some(c) => c,
            None => continue,
        };

        let cell_texts: Vec<String> = cells
            .iter()
            .map(|cell| render_children(cell, list_depth).replace('\n', " ").trim().to_string())
            .collect();

        output.push_str("| ");
        output.push_str(&cell_texts.join(" | "));
        output.push_str(" |\n");

        // Insert separator after first row (header)
        if row_idx == 0 && !header_done {
            let sep = cells.iter().map(|_| "---").collect::<Vec<_>>().join(" | ");
            output.push_str(&format!("| {} |\n", sep));
            header_done = true;
        }
    }

    format!("{}\n", output)
}
