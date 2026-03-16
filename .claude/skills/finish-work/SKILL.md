---
name: finish-work
description: Close out the current ticket session cleanly, push branch, create GitHub PR, and archive session
---

# finish-work Skill

Close out the current ticket session cleanly.

## Usage

```
/finish-work [TICKET_KEY]
```

TICKET_KEY is optional. If omitted, the skill resolves the target ticket automatically.

## What it does

1. Resolve which ticket to finish:

   a. If TICKET_KEY was provided:
      - Read `.claude/session.json`; if it doesn't exist, abort with "No active session."
      - If `ticket_key` in the file does not match the argument (case-insensitive),
        warn: "Active session is <KEY>, not <ARGUMENT>. Aborting." and stop.
      - If it matches, use this session and proceed to step 2.

   b. If TICKET_KEY was NOT provided:
      - Build the list of in-progress tickets:
          * If `.claude/session.json` exists → include it, label **[ACTIVE]**
          * For each file in `.claude/history/`, check `git branch --list <branch>`;
            if the branch exists locally → include it, label **[IN PROGRESS]**
      - If the list is empty → abort with "No in-progress tickets found."
      - If the list has exactly one entry → use it, proceed to step 2.
      - If the list has multiple entries → display the list and ask the user:
        "Which ticket do you want to finish? (enter the ticket key)"
        Wait for the user's reply, then use the matching entry and proceed to step 2.
        If the chosen ticket is from history (not the active session.json), inform the user
        that only the active session can be archived via `mechajira --archive`; offer to
        switch to that branch before finishing.

2. Check `git status`. If there are uncommitted changes, stage and commit them:
   ```
   <type>(<KEY>): <short description>
   ```

3. Run `mechajira --archive` to move `session.json` to `.claude/history/`

4. Push the feature branch to origin:
   ```
   git push -u origin <branch>
   ```

5. Detect if frontend changes are present by running:
   ```
   git diff main...HEAD --name-only
   ```
   Flag as **frontend** if any changed file matches:
   - Extensions: `.tsx`, `.jsx`, `.vue`, `.svelte`, `.css`, `.scss`, `.sass`, `.less`, `.html`
   - OR path segments: `components/`, `pages/`, `views/`, `app/`, `frontend/`, `ui/`, `web/`

6. Capture screenshots (frontend only, optional):

   If frontend changes were detected:
   - Ask: "Is a local dev server running? If so, what URL(s) should I screenshot?"
   - If the user provides URLs:
     - For each URL: use `mcp__chrome-devtools__navigate_page` → `mcp__chrome-devtools__take_screenshot`
     - Save each screenshot to `./screenshots/<KEY>-<n>.png` using the Write tool
     - Note paths for inclusion in the PR body
   - If the user says no or skips, continue without screenshots

7. Determine PR title:

   Derive `<type>` from the branch name prefix (e.g. `feat/olp-32-...` → `feat`).
   If the branch has no recognizable prefix (feat, fix, chore, refactor, test, docs, ci),
   derive type from the commit type used in step 2.

   PR title format (per CLAUDE.md):
   ```
   <type>(<KEY>): <summary>
   ```
   `<summary>` comes from `session.json → summary`.

8. Create PR using `gh`:

   Build PR body:
   ```markdown
   ## Summary
   - <1-3 bullet points derived from ticket description/comments>
   - Jira: <url from session.json>

   ## Test plan
   - [ ] <checklist items>

   ## Screenshots
   <If frontend: note "Screenshots saved to ./screenshots/ — drag into PR to attach." OR embed if supported>
   ```

   Run:
   ```bash
   gh pr create \
     --title "<type>(<KEY>): <summary>" \
     --body "$(cat <<'EOF'
   <PR body>
   EOF
   )"
   ```

9. Confirm:
   - Output the PR URL
   - Confirm that the session is archived
   - If screenshots were captured: remind the user to attach them to the PR if not already embedded
