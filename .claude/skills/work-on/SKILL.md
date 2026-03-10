---
name: work-on
description: Load a Jira ticket and scaffold a Plan Mode session with branch and implementation plan
---

# work-on Skill

Load a Jira ticket and scaffold a Plan Mode session.

## Usage

```
/work-on <TICKET_KEY>
```

Examples:
- `/work-on OLP-27`
- `/work-on PROJ-123`

## What it does

1. Run `jplan <KEY>` — fetches the ticket, renders description + comments, scouts code refs, writes `.claude/session.json`

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
