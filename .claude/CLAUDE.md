<!-- SPOX:START -->
# Spec Oxide Project

This project is developed with spec-driven development using Spec Oxide. Specs are the source of truth.

## Project Structure

- `specs/mission.md` — Project mission and context
- `specs/` — Current truth (capability specs)
- `specs/_changes/` — Proposed changes (deltas)

## Tool Priority

Spox (specs) → Serena (code) → Context7 (docs) → text tools (fallback)

## Workflows

- `/spox:plan` — Create change proposals
- `/spox:implement` — Implement approved changes
- `/spox:record` — Merge completed changes

## Standards

See `.claude/rules/spox/` for guardrails and standards.
<!-- SPOX:END -->
