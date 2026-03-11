# mechajira

<img src="logo.png" alt="mechajira logo" width="300" />

Fetch a Jira ticket and scaffold a [Claude Code](https://claude.ai/claude-code) Plan Mode session — ADF to Markdown, code reference scouting, and session persistence in one command.

## Install

### Recommended — install script

```bash
git clone https://github.com/your-org/mechajira.git
cd mechajira
./install.sh
```

The script will:
1. Install Rust via rustup if `cargo` is not found
2. Build a release binary
3. Copy it to `~/.local/bin/mechajira`
4. Copy skills to `~/.local/share/mechajira/skills/`
5. Warn if `~/.local/bin` is not in your `PATH`

If prompted, add the following to your `~/.zshrc` and reload:

```bash
export PATH="$HOME/.local/bin:$PATH"
source ~/.zshrc
```

### Manual (cargo)

```bash
git clone https://github.com/your-org/mechajira.git
cd mechajira
cargo build --release
cp target/release/mechajira ~/.local/bin/
```

> Requires Rust. Install via [rustup](https://rustup.rs/): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## Setup

First run launches an interactive wizard. Credentials are stored in `~/.config/mechajira/`.

```bash
mechajira --setup   # re-run wizard at any time
```

Override with env vars or a `.env` file (see `.env.example`):

```
JIRA_EMAIL / JIRA_API_TOKEN / JIRA_DOMAIN
```

## Usage

```bash
mechajira PROJ-123        # fetch ticket, write .claude/session.json
mechajira --archive       # move session to .claude/history/ when done
mechajira --install-skills  # copy skills into .claude/skills/ of current repo
mechajira --uninstall     # remove binary, skills store, and config
```

## Claude Code skills

### Installing skills into a repo

Once `mechajira` is installed, run this once inside any repository you want to use it with:

```bash
cd /path/to/your/repo
mechajira --install-skills
```

This copies `work-on` and `finish-work` into `.claude/skills/` so Claude Code picks them up automatically.

### Using the skills

**`/work-on [PROJ-123] [spec|plan]`**

With a ticket key:
1. Runs `mechajira PROJ-123`
2. Determines the conventional commit type (`feat`, `fix`, etc.) — asks if unclear
3. Creates branch `<type>/<KEY>-<slug>` (e.g. `feat/proj-123-oauth2-login`)
4. Enters the chosen execution mode (see below)

Without a ticket key (`/work-on`):
1. Lists the active session (from `.claude/session.json`) labeled **[ACTIVE]**
2. Lists any archived sessions in `.claude/history/` whose branch still exists locally, labeled **[IN PROGRESS]**
3. Prompts you to pick a ticket or supply a new key

#### Execution modes

| Invocation | Behavior |
|------------|----------|
| `/work-on PROJ-123` | Auto-detect: if spectremon absent → Plan Mode; if present → prompts you to choose |
| `/work-on PROJ-123 spec` | Spec-driven mode via spectremon (errors if not installed) |
| `/work-on PROJ-123 plan` | Plan Mode — skips spectremon even if installed |

#### spectremon integration (SDD)

mechajira integrates with [spectremon](https://github.com/spectremon/spectremon) to turn a Jira ticket into a formal spec before any code is written.

When spec mode is used, the skill composes an intent block from the ticket (summary, description, comments, code references, branch) and triggers spectremon's Discovery agent. Discovery produces three files:

- `.sdd/requirements.md` — EARS-style functional and non-functional requirements
- `.sdd/design.md` — architecture and design decisions
- `.sdd/tasks.md` — atomic implementation tasks

The spectremon Implementer/Architect loop then drives execution — Plan Mode is not entered.

To enable spec mode, scaffold spectremon once in your repo:

```bash
spectremon
```

This creates `.claude/spectremon.md`, which `/work-on` detects automatically.

**`/finish-work [PROJ-123]`**

With a ticket key — verifies it matches the active session, then closes it out:
1. Commits any remaining changes as `<type>(<KEY>): description`
2. Runs `mechajira --archive`
3. Switches to `main` and offers to delete the feature branch

Without a ticket key — auto-resolves the target:
1. If there is exactly one in-progress ticket, uses it
2. If there are multiple, lists them and asks which to finish
3. Proceeds with commit, archive, and branch cleanup

## Uninstall

```bash
mechajira --uninstall
```

Removes:
- `~/.local/bin/mechajira` — the binary
- `~/.local/share/mechajira/` — the skills store
- `~/.config/mechajira/` — stored credentials

Does **not** remove `.claude/skills/` from individual repos — that is local repo state you own.

## Commit conventions

Uses [Conventional Commits](https://www.conventionalcommits.org/) with the Jira key as scope:

```
feat(PROJ-123): add OAuth2 login flow
fix(PROJ-456): handle missing refresh token
feat(PROJ-123)!: replace cookies with JWT

BREAKING CHANGE: clients must re-authenticate
```

Types: `feat` `fix` `refactor` `test` `docs` `chore` `ci` `perf`

## Session file

`.claude/session.json` written on every run, archived to `.claude/history/<KEY>-<timestamp>.json` on `--archive`.

```json
{
  "ticket_key": "PROJ-123",
  "summary": "Add OAuth2 login flow",
  "url": "https://mycompany.atlassian.net/browse/PROJ-123",
  "branch": "feat/proj-123-add-oauth2-login-flow",
  "created_at": "2026-03-10T09:00:00Z"
}
```
