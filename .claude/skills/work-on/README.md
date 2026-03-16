# work-on Skill

Fetches a Jira ticket, creates a branch, and scaffolds a development session — either via **Plan Mode** or **spec-driven mode** using [spectremon](https://github.com/spectremon/spectremon).

## Usage

```
/work-on [TICKET_KEY] [spec|plan]
```

### Mode argument reference

| Invocation | Behavior |
|------------|----------|
| `/work-on PROJ-123` | Auto-detect: no spectremon → Plan Mode; spectremon present → prompts you to choose |
| `/work-on PROJ-123 spec` | Explicit spec-driven mode (errors if spectremon not installed) |
| `/work-on PROJ-123 plan` | Explicit Plan Mode (skips spectremon even if installed) |
| `/work-on` | List in-progress and archived tickets |

### Auto-detect behaviour

When no mode argument is supplied:

1. If `.claude/spectremon.md` is **absent** → Plan Mode, no prompt.
2. If `.claude/spectremon.md` is **present** → you are asked to choose:
   - `[1] spec` — Discovery agent writes EARS-style requirements + design + tasks
   - `[2] plan` — standard mechajira Plan Mode

## Examples

```
# Standard mechajira flow
/work-on OLP-27

# Force spec-driven mode for a complex feature ticket
/work-on PROJ-456 spec

# Force Plan Mode even if spectremon is installed
/work-on PROJ-456 plan

# See all in-progress tickets
/work-on
```

## spectremon integration

When spec mode is selected, the skill:

1. Composes an intent block from the Jira ticket (key, summary, URL, description, comments, code references, branch name).
2. Triggers spectremon's Discovery agent by saying "Start Spectremon" with that intent block.
3. Discovery generates three files:
   - `specs/requirements.md` — EARS-style functional + non-functional requirements
   - `specs/design.md` — architecture and design decisions
   - `specs/tasks.md` — atomic implementation tasks
4. The spectremon Implementer/Architect loop takes over from there — Plan Mode is **not** entered.

### Installing spectremon

Spectremon must be scaffolded in the repo before spec mode is available:

```
spectremon
```

This creates `.claude/spectremon.md`, which the skill detects as the presence signal.

## Finishing work

Use `/finish-work` after either mode to archive the session — it is spectremon-agnostic and works the same regardless of which mode was used.
