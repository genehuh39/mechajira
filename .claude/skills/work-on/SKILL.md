---
name: work-on
description: Load a Jira ticket and scaffold a Plan Mode session with branch and implementation plan
---

# work-on Skill

Load a Jira ticket and scaffold a Plan Mode session.

## Usage

```
/work-on [TICKET_KEY]
```

If TICKET_KEY is omitted, list all in-progress tickets instead of starting a new session.

Examples:
- `/work-on OLP-27`
- `/work-on PROJ-123`
- `/work-on` — list in-progress tickets

## List mode (no argument)

When invoked as `/work-on` with no ticket key:

1. Check if `.claude/session.json` exists. If so, display the active ticket:
   - ticket_key, summary, branch, created_at
   - Label it **[ACTIVE]**

2. Read every file in `.claude/history/` (if the directory exists). For each archived session file:
   - Parse the JSON to get ticket_key, summary, branch
   - Run `git branch --list <branch>` to check whether the branch still exists locally
   - If the branch exists, display the entry and label it **[IN PROGRESS — branch exists]**
   - If the branch does not exist, display the entry and label it **[ARCHIVED]**

3. If no active session and no history files exist, inform the user there are no in-progress tickets.

4. Ask the user which ticket they'd like to work on, or prompt them to supply a ticket key.

## What it does (when a TICKET_KEY is provided)

1. Run `mechajira <KEY>` — fetches the ticket, renders description + comments, scouts code refs, writes `.claude/session.json`

2. Determine the conventional commit type that best fits the ticket (e.g. `feat`, `fix`, `refactor`). Confirm with user if unclear.

3. Propose a branch name derived from the type and ticket key:
   ```
   <type>/<KEY>-<short-slug>
   ```
   Example: `feat/proj-123-oauth2-login-flow`

4. Create the branch if it doesn't exist locally:
   ```
   git checkout -b <type>/<KEY>-<short-slug>
   ```

5. Enter Claude Code Plan Mode (`/plan`) and produce a step-by-step implementation plan based on the ticket description, comments, and identified code references.

6. Ask user to confirm the plan before writing code.

7. When committing during implementation, use Conventional Commits format:
   ```
   <type>(<KEY>): <short description>
   ```
   Example: `feat(PROJ-123): add OAuth2 login flow`
