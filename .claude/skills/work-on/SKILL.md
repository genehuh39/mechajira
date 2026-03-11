---
name: work-on
description: Load a Jira ticket and scaffold a session — Plan Mode or spec-driven via spectremon
---

# work-on Skill

Load a Jira ticket and scaffold a session with branch and implementation plan.

## Usage

```
/work-on [TICKET_KEY] [spec|plan]
```

| Invocation | Behavior |
|------------|----------|
| `/work-on PROJ-123` | Auto-detect: if spectremon absent → Plan Mode; if present → prompt user to choose |
| `/work-on PROJ-123 spec` | Explicit spec-driven mode; error if `.claude/spectremon.md` absent |
| `/work-on PROJ-123 plan` | Explicit Plan Mode; skips spectremon even if installed |
| `/work-on` | List in-progress tickets (no-arg list mode — unchanged) |

Examples:
- `/work-on OLP-27`
- `/work-on PROJ-123 spec`
- `/work-on PROJ-123 plan`
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

5. Determine which execution mode to use:

   a. If MODE argument is "plan" → go to step 5-PLAN.

   b. If MODE argument is "spec":
      - Check for `.claude/spectremon.md`. If absent, abort with:
        "Spectremon is not installed in this repo. Run `spectremon` to scaffold it,
         or use `/work-on <KEY> plan` for Plan Mode."
      - Go to step 5-SPEC.

   c. If MODE argument is omitted:
      - If `.claude/spectremon.md` is absent → go to step 5-PLAN.
      - If `.claude/spectremon.md` is present → ask the user:
        "Spectremon is available. Which mode?
         [1] spec — spec-driven (Discovery generates requirements, design, tasks)
         [2] plan — Plan Mode (standard mechajira flow)"
        Route based on answer.

### 5-PLAN

Enter Plan Mode (`/plan`) and produce a step-by-step implementation plan based on the ticket description, comments, and identified code references. Ask the user to confirm the plan before writing code.

### 5-SPEC

a. If the ticket description is very short (fewer than ~3 sentences), ask the user to add more context before proceeding.

b. Compose a spectremon intent block containing:
   - "Ticket <KEY>: <summary> (<url>)"
   - Full description markdown
   - Recent comments
   - Code references identified in step 1
   - "Branch <branch> has already been created."

c. Say "Start Spectremon" with this intent block.

d. Let Discovery generate `.sdd/requirements.md`, `.sdd/design.md`, `.sdd/tasks.md`.

e. Do NOT enter Plan Mode — spectremon's Implementer/Architect loop handles execution from here.

6. When committing during implementation, use Conventional Commits format:
   ```
   <type>(<KEY>): <short description>
   ```
   Example: `feat(PROJ-123): add OAuth2 login flow`
