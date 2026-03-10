# Project Rules

## Active Session
Always check `.claude/session.json` before performing any Git operations.
If the file exists, read it to determine the active ticket key, summary, and branch.

## Commit Format
All commits **must** use [Conventional Commits](https://www.conventionalcommits.org/) with the Jira key as the scope:
```
<type>(KEY-123): short description
```

Common types: `feat`, `fix`, `chore`, `refactor`, `test`, `docs`, `ci`.

Examples:
```
feat(PROJ-123): add OAuth2 login flow
fix(PROJ-123): handle missing refresh token
chore(PROJ-123): update dependencies
```

Breaking changes must include `!` after the type/scope and a `BREAKING CHANGE:` footer:
```
feat(PROJ-123)!: replace session tokens with JWT

BREAKING CHANGE: existing session cookies are invalidated
```

## Pull Request Format
PR titles **must** follow Conventional Commits format with the Jira key as scope:
```
<type>(KEY-123): summary of the ticket
```

## Branch Hygiene
- Work on the branch specified in `.claude/session.json`
- Do not merge or rebase without reading `session.json` first
- Run `jplan --archive` via the `finish-work` skill when a ticket is complete
