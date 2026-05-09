# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Purpose

This is a knowledge-base repository for AI-assisted development workflows. It collects best practices, rules, and workflow documentation — not executable code.

## Structure

- `00-genRules/` — Collected best practices and workflow guides for using Distack and Superpowers skills across the full design-to-deploy pipeline.

## Workflow Philosophy (from bestpractices.md)

Work is organized in three phases, with documentation deposited at each step so downstream AI sessions can pick up context without starting over:

1. **Planning** — Validate requirements (`office-hours`), strategic review (`plan-ceo-review`), architecture review (`plan-eng-review`), design review (`plan-design-review`)
2. **Development** — Structured ideation (`brainstorming`), PRD creation (`write-a-prd`), task decomposition (`writing-plans`), parallel subagent execution (`subagent-driven-development`)
3. **Release** — Code review (`review`), browser QA (`qa`), ship workflow (`ship`)

## Conventions

- All documentation and rules files are in Chinese.
- File and folder names use numeric prefixes (e.g., `00-`) for ordering.
- This repo is not a git repository and contains no build/test commands.

## Skill routing

When the user's request matches an available skill, ALWAYS invoke it using the Skill
tool as your FIRST action. Do NOT answer directly, do NOT use other tools first.
The skill has specialized workflows that produce better results than ad-hoc answers.

Key routing rules:
- Product ideas, "is this worth building", brainstorming → invoke office-hours
- Bugs, errors, "why is this broken", 500 errors → invoke investigate
- Ship, deploy, push, create PR → invoke ship
- QA, test the site, find bugs → invoke qa
- Code review, check my diff → invoke review
- Update docs after shipping → invoke document-release
- Weekly retro → invoke retro
- Design system, brand → invoke design-consultation
- Visual audit, design polish → invoke design-review
- Architecture review → invoke plan-eng-review
- Save progress, checkpoint, resume → invoke checkpoint
- Code quality, health check → invoke health
