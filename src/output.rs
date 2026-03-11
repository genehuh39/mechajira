/// Renders the final plan output to stdout.
pub fn print_plan(
    key: &str,
    summary: &str,
    status: &str,
    assignee: Option<&str>,
    description: &str,
    comments: &str,
    code_refs: &[String],
    branch: &str,
    domain: &str,
) {
    let url = format!("https://{}/browse/{}", domain, key);
    let sep = "─".repeat(60);

    println!("\n{}", sep);
    println!("  {} · {}", key, summary);
    println!("{}", sep);
    println!("  Status   : {}", status);
    println!(
        "  Assignee : {}",
        assignee.unwrap_or("Unassigned")
    );
    println!("  URL      : {}", url);
    println!("  Branch   : {}", branch);
    println!("{}\n", sep);

    // Description
    if !description.is_empty() {
        println!("## Description\n");
        println!("{}", description);
    }

    // Comments
    if !comments.is_empty() {
        println!("## Last Comments\n");
        println!("{}", comments);
        println!();
    }

    // Code references
    if !code_refs.is_empty() {
        println!("## Potential Code References\n");
        for r in code_refs {
            println!("  - {}", r);
        }
        println!();
    }

    // Session hint
    println!("{}", sep);
    println!("  session.json written to .claude/session.json");
    println!("  Run `mechajira --archive` when done to preserve history.");
    println!("{}\n", sep);
}
