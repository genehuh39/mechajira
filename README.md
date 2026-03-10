# jplan

Fetch a Jira ticket and scaffold a [Claude Code](https://claude.ai/claude-code) Plan Mode session — ADF to Markdown, code reference scouting, and session persistence in one command.

## Install

### Recommended — install script

```bash
git clone https://github.com/your-org/jira-to-plan.git
cd jira-to-plan
./install.sh
```

The script will:
1. Install Rust via rustup if `cargo` is not found
2. Build a release binary
3. Copy it to `~/.local/bin/jplan`
4. Warn if `~/.local/bin` is not in your `PATH`

If prompted, add the following to your `~/.zshrc` and reload:

```bash
export PATH="$HOME/.local/bin:$PATH"
source ~/.zshrc
```

### Manual (cargo)

```bash
git clone https://github.com/your-org/jira-to-plan.git
cd jira-to-plan
cargo build --release
cp target/release/jplan ~/.local/bin/
```

> Requires Rust. Install via [rustup](https://rustup.rs/): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## Setup

First run launches an interactive wizard. Credentials are stored in `~/.config/jplan/`.

```bash
jplan --setup   # re-run wizard at any time
```

Override with env vars or a `.env` file (see `.env.example`):

```
JIRA_EMAIL / JIRA_API_TOKEN / JIRA_DOMAIN
```

## Usage

```bash
jplan PROJ-123      # fetch ticket, write .claude/session.json
jplan --archive     # move session to .claude/history/ when done
```

## Claude Code skills

### Installing skills into a repo

Once `jplan` is installed, run this once inside any repository you want to use it with:

```bash
cd /path/to/your/repo
jplan --install-skills
```

This copies `work-on` and `finish-work` into `.claude/skills/` so Claude Code picks them up automatically.

### Using the skills

**`work-on PROJ-123`**
1. Runs `jplan PROJ-123`
2. Determines the conventional commit type (`feat`, `fix`, etc.) — asks if unclear
3. Creates branch `<type>/<KEY>-<slug>` (e.g. `feat/proj-123-oauth2-login`)
4. Enters Plan Mode with a step-by-step implementation plan

**`finish-work`**
1. Commits any remaining changes as `<type>(<KEY>): description`
2. Runs `jplan --archive`
3. Switches to `main` and offers to delete the feature branch

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
