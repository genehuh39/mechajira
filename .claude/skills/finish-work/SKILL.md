---
name: finish-work
description: Close out the current ticket session cleanly, archive session, and clean up branch
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

4. Switch back to `main` (or `master`):
   ```
   git checkout main
   ```

5. Offer to delete the feature branch:
   ```
   git branch -d <branch>
   ```

6. Confirm the session is archived and branch is cleaned up
