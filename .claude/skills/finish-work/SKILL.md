---
name: finish-work
description: Close out the current ticket session cleanly, archive session, and clean up branch
---

# finish-work Skill

Close out the current ticket session cleanly.

## Usage

```
/finish-work
```

## What it does

1. Read `.claude/session.json` to retrieve `ticket_key` and `branch`

2. Check `git status`. If there are uncommitted changes, stage and commit them:
   ```
   <type>(<KEY>): <short description>
   ```

3. Run `jplan --archive` to move `session.json` to `.claude/history/`

4. Switch back to `main` (or `master`):
   ```
   git checkout main
   ```

5. Offer to delete the feature branch:
   ```
   git branch -d <branch>
   ```

6. Confirm the session is archived and branch is cleaned up
